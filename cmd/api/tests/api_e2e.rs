//! Pruebas de extremo a extremo: el servidor real (`startup::run`) contra MongoDB y Redis en
//! contenedores efímeros, conducido solo por HTTP.
//!
//! Necesitan Docker: `cargo test -p quizz-api --test api_e2e`. Todo el recorrido vive en un
//! único test porque el servidor corre en el runtime de ese test.

use mongodb::Database;
use mongodb::bson::{Document, doc};
use quizz_api::cache::crear_conexion_redis;
use quizz_api::configuration::{CorsSettings, DatabaseSettings, JwtSettings, LoginSettings};
use quizz_api::indices::crear_indices;
use quizz_api::mongo::create_mongo_client;
use quizz_api::startup::{OpcionesHttp, init_casbin_enforcer, run};
use reqwest::{Method, StatusCode};
use serde_json::{Value, json};
use std::net::TcpListener;
use testcontainers::core::{IntoContainerPort, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, GenericImage, ImageExt};

const ADMIN_ID: &str = "6f1c2a3b-4d5e-4f60-8a7b-9c0d1e2f3a4b";
const ADMIN_DOCUMENTO: &str = "70000001";
const ADMIN_PASSWORD: &str = "clave-admin-de-pruebas";
// Holgado para todos los logins del recorrido; el último paso lo agota a propósito.
const MAX_INTENTOS_POR_IP: u64 = 60;
const MAX_FALLOS_POR_DOCUMENTO: u64 = 3;

struct Entorno {
    url: String,
    http: reqwest::Client,
    db: Database,
    _mongo: ContainerAsync<GenericImage>,
    _redis: ContainerAsync<GenericImage>,
}

async fn levantar() -> Entorno {
    let mongo = GenericImage::new("mongo", "8.0")
        .with_exposed_port(27017.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Waiting for connections"))
        .start()
        .await
        .expect("no se pudo iniciar MongoDB (¿está Docker en marcha?)");
    let redis = GenericImage::new("redis", "7-alpine")
        .with_exposed_port(6379.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
        .with_cmd(["redis-server", "--save", ""])
        .start()
        .await
        .expect("no se pudo iniciar Redis");

    let db = create_mongo_client(&DatabaseSettings {
        uri: Some(format!(
            "mongodb://127.0.0.1:{}",
            mongo.get_host_port_ipv4(27017).await.unwrap()
        )),
        host: String::new(),
        port: None,
        username: String::new(),
        password: String::new(),
        database_name: "quizz_e2e".to_string(),
    })
    .await
    .unwrap();
    crear_indices(&db).await;
    let redis_conexion = crear_conexion_redis(&format!(
        "redis://127.0.0.1:{}",
        redis.get_host_port_ipv4(6379).await.unwrap()
    ))
    .await
    .unwrap();

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());
    let servidor = run(
        listener,
        db.clone(),
        redis_conexion,
        init_casbin_enforcer().await.unwrap(),
        OpcionesHttp {
            jwt: JwtSettings {
                secret: "secreto-de-pruebas-e2e-de-mas-de-32-bytes".to_string(),
                expiration_seconds: 3600,
            },
            cors: CorsSettings::default(),
            login: LoginSettings {
                max_intentos_por_ip: MAX_INTENTOS_POR_IP,
                max_fallos_por_documento: MAX_FALLOS_POR_DOCUMENTO,
                ..LoginSettings::default()
            },
        },
    )
    .unwrap();
    tokio::spawn(servidor);

    // No hay endpoint público para crear el primer admin: se siembra en la base de datos.
    let hash = bcrypt::hash(ADMIN_PASSWORD, 4).unwrap();
    db.collection::<Document>("admin")
        .insert_one(doc! {
            "_id": ADMIN_ID,
            "nombre": "Ada",
            "primer_apellido": "Admin",
            "segundo_apellido": "Root",
            "documento": ADMIN_DOCUMENTO,
            "password": hash,
        })
        .await
        .unwrap();

    Entorno {
        url,
        http: reqwest::Client::new(),
        db,
        _mongo: mongo,
        _redis: redis,
    }
}

impl Entorno {
    async fn pedir(
        &self,
        metodo: Method,
        ruta: &str,
        token: Option<&str>,
        cuerpo: Option<Value>,
    ) -> (StatusCode, Value) {
        let mut peticion = self.http.request(metodo, format!("{}{ruta}", self.url));
        if let Some(token) = token {
            peticion = peticion.bearer_auth(token);
        }
        if let Some(cuerpo) = cuerpo {
            peticion = peticion.json(&cuerpo);
        }
        let respuesta = peticion.send().await.unwrap();
        let estado = respuesta.status();
        let texto = respuesta.text().await.unwrap();
        (
            estado,
            serde_json::from_str(&texto).unwrap_or(Value::String(texto)),
        )
    }

    async fn login(&self, documento: &str, password: &str) -> (StatusCode, Value) {
        self.pedir(
            Method::POST,
            "/login",
            None,
            Some(json!({ "documento": documento, "password": password })),
        )
        .await
    }

    async fn token(&self, documento: &str, password: &str) -> String {
        let (estado, cuerpo) = self.login(documento, password).await;
        assert_eq!(estado, StatusCode::OK, "login de {documento}: {cuerpo}");
        cuerpo["token"].as_str().unwrap().to_string()
    }

    /// Registra un postulante y devuelve su contraseña inicial.
    async fn registrar_postulante(&self, admin: &str, id: &str, documento: &str) -> String {
        let (estado, cuerpo) = self
            .pedir(
                Method::POST,
                &format!("/postulantes/{id}"),
                Some(admin),
                Some(json!({
                    "documento": documento,
                    "nombre": "Juan",
                    "primer_apellido": "Perez",
                    "segundo_apellido": "Quispe",
                    "fecha_nacimiento": "1990-01-01",
                    "grado_instruccion": "superior",
                    "genero": "masculino",
                })),
            )
            .await;
        assert_eq!(
            estado,
            StatusCode::CREATED,
            "registrar postulante: {cuerpo}"
        );
        documento[documento.len() - 4..].to_string()
    }
}

/// Crea un examen con una pregunta, lo asocia a una evaluación nueva y la publica.
async fn evaluacion_publicada(e: &Entorno, admin: &str) -> String {
    let examen_id = "2fcb7b0d-30e2-4853-afbf-9df79dd83ecb";
    let evaluacion_id = "2cf52b7a-0ee3-43a9-9b89-4a8baaa22250";

    let (estado, cuerpo) = e
        .pedir(
            Method::POST,
            &format!("/examenes/{examen_id}"),
            Some(admin),
            Some(json!({ "titulo": "Entrevista", "descripcion": "Entrevista", "instrucciones": "Responda" })),
        )
        .await;
    assert_eq!(estado, StatusCode::CREATED, "crear examen: {cuerpo}");

    let (estado, cuerpo) = e
        .pedir(
            Method::PUT,
            &format!("/examenes/{examen_id}"),
            Some(admin),
            Some(json!({ "preguntas": [{
                "etiqueta": "no",
                "tipo_de_pregunta": "alternativa_unica",
                "imagen_ref": "",
                "contenido": "¿Usted ha sido entrevistado?",
                "alternativas": { "A": "Sí", "B": "No" },
                "puntaje": { "A": 1, "B": 0 },
            }]})),
        )
        .await;
    assert_eq!(estado, StatusCode::OK, "agregar pregunta: {cuerpo}");

    let (estado, cuerpo) = e
        .pedir(
            Method::POST,
            &format!("/evaluaciones/{evaluacion_id}"),
            Some(admin),
            Some(json!({ "titulo": "Licencia A-I", "descripcion": "Proceso de evaluación" })),
        )
        .await;
    assert_eq!(estado, StatusCode::CREATED, "crear evaluacion: {cuerpo}");

    let (estado, cuerpo) = e
        .pedir(
            Method::PUT,
            &format!("/evaluaciones/{evaluacion_id}"),
            Some(admin),
            Some(json!({ "examenes": [examen_id] })),
        )
        .await;
    assert_eq!(estado, StatusCode::OK, "asociar examen: {cuerpo}");

    let (estado, cuerpo) = e
        .pedir(
            Method::PATCH,
            &format!("/evaluaciones/{evaluacion_id}"),
            Some(admin),
            Some(json!({})),
        )
        .await;
    assert_eq!(estado, StatusCode::OK, "publicar evaluacion: {cuerpo}");

    evaluacion_id.to_string()
}

/// Asigna la evaluación al postulante y devuelve el id de su hoja de respuestas.
async fn asignar(e: &Entorno, admin: &str, evaluacion_id: &str, postulante_id: &str) -> String {
    let (estado, cuerpo) = e
        .pedir(
            Method::POST,
            &format!("/evaluaciones/{evaluacion_id}/respuestas"),
            Some(admin),
            Some(json!({ "postulante_id": postulante_id })),
        )
        .await;
    assert_eq!(estado, StatusCode::CREATED, "asignar: {cuerpo}");
    let id = cuerpo["id"]
        .as_str()
        .expect("la asignación devuelve el id")
        .to_string();
    assert_eq!(
        cuerpo["_links"]["self"]["href"],
        format!("/respuestas/{id}")
    );
    id
}

#[tokio::test(flavor = "multi_thread")]
async fn flujo_completo_y_controles_de_acceso() {
    let e = levantar().await;
    let admin = e.token(ADMIN_DOCUMENTO, ADMIN_PASSWORD).await;

    // Errores de entrada: 400 con el formato {"error": ...}, no 500.
    let (estado, cuerpo) = e.pedir(Method::POST, "/login", None, Some(json!({}))).await;
    assert_eq!(estado, StatusCode::BAD_REQUEST);
    assert!(cuerpo["error"].is_string(), "{cuerpo}");
    let (estado, cuerpo) = e
        .pedir(
            Method::POST,
            "/examenes/no-es-un-uuid",
            Some(&admin),
            Some(json!({ "titulo": "T", "descripcion": "D", "instrucciones": "I" })),
        )
        .await;
    assert_eq!(estado, StatusCode::BAD_REQUEST, "{cuerpo}");

    // SEC-10: no se puede crear una cuenta de personal con una contraseña vacía o corta.
    for password in ["", "        ", "corta"] {
        let (estado, cuerpo) = e
            .pedir(
                Method::POST,
                "/admins/c3d4e5f6-a7b8-9012-cdef-123456789012",
                Some(&admin),
                Some(json!({
                    "nombre": "Ana", "primer_apellido": "Paz", "segundo_apellido": "Rey",
                    "documento": "70000002", "password": password,
                })),
            )
            .await;
        assert_eq!(estado, StatusCode::BAD_REQUEST, "{password:?}: {cuerpo}");
    }
    let (estado, _) = e.login("70000002", "").await;
    assert_eq!(estado, StatusCode::UNAUTHORIZED);

    // Credenciales incorrectas: misma respuesta para documento inexistente y clave errónea.
    let (estado, _) = e.login(ADMIN_DOCUMENTO, "otra-clave").await;
    assert_eq!(estado, StatusCode::UNAUTHORIZED);
    let (estado, _) = e.login("79999999", "otra-clave").await;
    assert_eq!(estado, StatusCode::UNAUTHORIZED);

    let postulante_a = "e17439e0-79e1-47e3-b5f9-5b54367fa290";
    let postulante_b = "d4e92ac2-882a-4e46-ae56-b2361ac5327e";
    let clave_a = e
        .registrar_postulante(&admin, postulante_a, "71111111")
        .await;
    let clave_b = e
        .registrar_postulante(&admin, postulante_b, "72222222")
        .await;
    let token_a = e.token("71111111", &clave_a).await;
    let token_b = e.token("72222222", &clave_b).await;

    // SEC-03: un postulante solo lee su propio registro, pida lo que pida en la query.
    for query in [
        format!("?id={postulante_a}&documento=72222222"),
        "?documento=72222222".to_string(),
        format!("?id={postulante_b}"),
        String::new(),
    ] {
        let (estado, cuerpo) = e
            .pedir(
                Method::GET,
                &format!("/postulantes{query}"),
                Some(&token_a),
                None,
            )
            .await;
        assert_eq!(estado, StatusCode::OK, "{query}: {cuerpo}");
        assert_eq!(cuerpo["documento"], "71111111", "{query}: {cuerpo}");
    }
    let (estado, cuerpo) = e
        .pedir(
            Method::GET,
            "/postulantes?documento=72222222",
            Some(&admin),
            None,
        )
        .await;
    assert_eq!(estado, StatusCode::OK);
    assert_eq!(cuerpo["documento"], "72222222");

    // SEC-01: una ruta codificada no salta la autorización.
    let cuerpo_admin = json!({
        "nombre": "Mallory", "primer_apellido": "M", "segundo_apellido": "M",
        "documento": "73333333", "password": "clave-de-mallory-123",
    });
    for ruta in [
        "/%61dmins/c3d4e5f6-a7b8-9012-cdef-123456789012",
        "/admins/c3d4e5f6-a7b8-9012-cdef-123456789012",
    ] {
        let (estado, _) = e
            .pedir(
                Method::POST,
                ruta,
                Some(&token_a),
                Some(cuerpo_admin.clone()),
            )
            .await;
        assert_eq!(estado, StatusCode::FORBIDDEN, "{ruta}");
    }
    let creados =
        e.db.collection::<Document>("admin")
            .count_documents(doc! { "documento": "73333333" })
            .await
            .unwrap();
    assert_eq!(creados, 0);

    // Flujo del postulante: empezar, contestar, finalizar.
    let evaluacion_id = evaluacion_publicada(&e, &admin).await;
    let hoja_a = asignar(&e, &admin, &evaluacion_id, postulante_a).await;
    let hoja_b = asignar(&e, &admin, &evaluacion_id, postulante_b).await;

    // DAT-01: asignar dos veces la misma evaluación es un conflicto, también en paralelo.
    let (estado, _) = e
        .pedir(
            Method::POST,
            &format!("/evaluaciones/{evaluacion_id}/respuestas"),
            Some(&admin),
            Some(json!({ "postulante_id": postulante_a })),
        )
        .await;
    assert_eq!(estado, StatusCode::CONFLICT);
    let postulante_c = "0b9fbef0-fc03-4e24-b5d0-dfb42b537324";
    e.registrar_postulante(&admin, postulante_c, "73333331")
        .await;
    let ruta = format!("/evaluaciones/{evaluacion_id}/respuestas");
    let simultaneas = (0..8).map(|_| {
        e.pedir(
            Method::POST,
            &ruta,
            Some(&admin),
            Some(json!({ "postulante_id": postulante_c })),
        )
    });
    let estados: Vec<StatusCode> = futures::future::join_all(simultaneas)
        .await
        .into_iter()
        .map(|(estado, _)| estado)
        .collect();
    assert_eq!(
        estados
            .iter()
            .filter(|e| **e == StatusCode::CREATED)
            .count(),
        1,
        "{estados:?}"
    );
    assert!(
        estados
            .iter()
            .all(|e| *e == StatusCode::CREATED || *e == StatusCode::CONFLICT),
        "{estados:?}"
    );
    let hojas_c =
        e.db.collection::<Document>("respuesta")
            .count_documents(doc! { "postulante_id": postulante_c })
            .await
            .unwrap();
    assert_eq!(hojas_c, 1);

    // R-035: un borrador no se puede asignar.
    let borrador = "8e65028c-30e6-47e5-b7dc-121ee2133d49";
    let (estado, _) = e
        .pedir(
            Method::POST,
            &format!("/evaluaciones/{borrador}"),
            Some(&admin),
            Some(json!({ "titulo": "Borrador", "descripcion": "Sin publicar" })),
        )
        .await;
    assert_eq!(estado, StatusCode::CREATED);
    let (estado, _) = e
        .pedir(
            Method::POST,
            &format!("/evaluaciones/{borrador}/respuestas"),
            Some(&admin),
            Some(json!({ "postulante_id": postulante_a })),
        )
        .await;
    assert_eq!(estado, StatusCode::CONFLICT);

    let (estado, lista) = e
        .pedir(Method::GET, "/respuestas", Some(&token_a), None)
        .await;
    assert_eq!(estado, StatusCode::OK, "{lista}");
    assert_eq!(lista["items"].as_array().map(Vec::len), Some(1), "{lista}");

    let (estado, cuerpo) = e
        .pedir(
            Method::PATCH,
            &format!("/respuestas/{hoja_a}/estado"),
            Some(&token_a),
            Some(json!({ "accion": "empezar" })),
        )
        .await;
    assert_eq!(estado, StatusCode::OK, "empezar: {cuerpo}");
    assert_eq!(cuerpo["estado"], "en_proceso");
    // Empezar de nuevo es idempotente.
    let (estado, _) = e
        .pedir(
            Method::PATCH,
            &format!("/respuestas/{hoja_a}/estado"),
            Some(&token_a),
            Some(json!({ "accion": "empezar" })),
        )
        .await;
    assert_eq!(estado, StatusCode::OK);
    // Una acción desconocida es una petición inválida.
    let (estado, _) = e
        .pedir(
            Method::PATCH,
            &format!("/respuestas/{hoja_a}/estado"),
            Some(&token_a),
            Some(json!({ "accion": "reiniciar" })),
        )
        .await;
    assert_eq!(estado, StatusCode::BAD_REQUEST);

    let (estado, hoja) = e
        .pedir(
            Method::GET,
            &format!("/respuestas/{hoja_a}"),
            Some(&token_a),
            None,
        )
        .await;
    assert_eq!(estado, StatusCode::OK, "{hoja}");
    let examen = &hoja["evaluacion"]["examenes"][0];
    let examen_id = examen["id"].as_str().unwrap();
    let pregunta_id = examen["preguntas"][0]["id"].as_str().unwrap();
    let contestar =
        format!("/respuestas/{hoja_a}/examenes/{examen_id}/preguntas/{pregunta_id}/contestaciones");

    // COR-02: una sola respuesta por pregunta, y debe ser una de sus alternativas.
    for respuestas in [json!(["A", "A", "A"]), json!(["A", "B"]), json!(["Z"])] {
        let (estado, cuerpo) = e
            .pedir(
                Method::POST,
                &contestar,
                Some(&token_a),
                Some(json!({ "respuestas": respuestas })),
            )
            .await;
        assert_eq!(estado, StatusCode::BAD_REQUEST, "{respuestas}: {cuerpo}");
    }

    let (estado, cuerpo) = e
        .pedir(
            Method::POST,
            &contestar,
            Some(&token_a),
            Some(json!({ "respuestas": ["A"] })),
        )
        .await;
    assert_eq!(estado, StatusCode::OK, "contestar: {cuerpo}");

    let (estado, cuerpo) = e
        .pedir(
            Method::PATCH,
            &format!("/respuestas/{hoja_a}/estado"),
            Some(&token_a),
            Some(json!({ "accion": "finalizar" })),
        )
        .await;
    assert_eq!(estado, StatusCode::OK, "finalizar: {cuerpo}");
    assert_eq!(cuerpo["estado"], "finalizado");

    // SEC-09: el postulante no ve los puntos de su hoja; el personal sí (R-032).
    let (estado, vista_a) = e
        .pedir(
            Method::GET,
            &format!("/respuestas/{hoja_a}"),
            Some(&token_a),
            None,
        )
        .await;
    assert_eq!(estado, StatusCode::OK);
    assert_eq!(vista_a["estado"], "finalizado");
    let texto = vista_a.to_string();
    assert!(!texto.contains("puntos"), "{texto}");
    let (estado, vista_admin) = e
        .pedir(
            Method::GET,
            &format!("/respuestas/{hoja_a}"),
            Some(&admin),
            None,
        )
        .await;
    assert_eq!(estado, StatusCode::OK, "{vista_admin}");
    assert_eq!(
        vista_admin["evaluacion"]["examenes"][0]["preguntas"][0]["puntos"],
        1
    );
    assert_eq!(
        vista_admin["evaluacion"]["examenes"][0]["puntos_obtenidos"],
        1
    );

    // Tras finalizar no se aceptan más respuestas.
    let (estado, cuerpo) = e
        .pedir(
            Method::POST,
            &contestar,
            Some(&token_a),
            Some(json!({ "respuestas": ["B"] })),
        )
        .await;
    assert_eq!(
        estado,
        StatusCode::CONFLICT,
        "contestar tras finalizar: {cuerpo}"
    );

    // SEC-02: un postulante no puede escribir en la hoja de otro (404: no revela que existe).
    let contestar_en_b = contestar.replace(&hoja_a, &hoja_b);
    let (estado, _) = e
        .pedir(
            Method::POST,
            &contestar_en_b,
            Some(&token_a),
            Some(json!({ "respuestas": ["A"] })),
        )
        .await;
    assert_eq!(estado, StatusCode::NOT_FOUND);
    for accion in ["empezar", "finalizar"] {
        let (estado, _) = e
            .pedir(
                Method::PATCH,
                &format!("/respuestas/{hoja_b}/estado"),
                Some(&token_a),
                Some(json!({ "accion": accion })),
            )
            .await;
        assert_eq!(estado, StatusCode::NOT_FOUND, "{accion}");
    }
    let hoja_b_guardada =
        e.db.collection::<Document>("respuesta")
            .find_one(doc! { "_id": &hoja_b })
            .await
            .unwrap()
            .unwrap();
    assert_eq!(hoja_b_guardada.get_str("estado").unwrap(), "creado");

    // DAT-05 y R-046: solo se califica una hoja finalizada, y queda registrado quién lo hizo.
    let psicologo = "a1b2c3d4-e5f6-7890-abcd-ef1234567890";
    let (estado, cuerpo) = e
        .pedir(
            Method::POST,
            &format!("/psicologos/{psicologo}"),
            Some(&admin),
            Some(json!({
                "nombre": "Maria", "primer_apellido": "Garcia", "segundo_apellido": "Lopez",
                "documento": "44556677", "especialidad": "Clinica", "colegiatura": "CPsP-123",
                "password": "clave-del-psicologo",
            })),
        )
        .await;
    assert_eq!(estado, StatusCode::CREATED, "{cuerpo}");
    let token_psicologo = e.token("44556677", "clave-del-psicologo").await;
    let revision = |respuesta_id: &str| {
        (
            format!("/revisiones/{respuesta_id}"),
            json!({
                "evaluacion_id": evaluacion_id,
                "resultado": "apto",
                "examenes": [{ "examen_id": examen_id, "observacion": "Bien" }],
            }),
        )
    };
    let (ruta, cuerpo) = revision(&hoja_b);
    let (estado, _) = e
        .pedir(Method::POST, &ruta, Some(&token_psicologo), Some(cuerpo))
        .await;
    assert_eq!(
        estado,
        StatusCode::CONFLICT,
        "una hoja sin finalizar no se califica"
    );
    let (ruta, cuerpo) = revision(&hoja_a);
    let (estado, cuerpo) = e
        .pedir(Method::POST, &ruta, Some(&token_psicologo), Some(cuerpo))
        .await;
    assert_eq!(estado, StatusCode::CREATED, "{cuerpo}");
    let (estado, calificada) = e.pedir(Method::GET, &ruta, Some(&admin), None).await;
    assert_eq!(estado, StatusCode::OK, "{calificada}");
    assert_eq!(
        calificada["psicologo"]["nombre_completo"],
        "Maria Garcia Lopez"
    );
    assert_eq!(calificada["resultado"], "apto");
    assert!(calificada["fecha_revision"].is_string(), "{calificada}");
    assert_eq!(
        calificada["evaluacion"]["examenes"][0]["observacion"],
        "Bien"
    );

    // Un postulante no lee la hoja de otro, y no ve las preguntas de la suya antes de empezar.
    let (estado, _) = e
        .pedir(
            Method::GET,
            &format!("/respuestas/{hoja_b}"),
            Some(&token_a),
            None,
        )
        .await;
    assert_eq!(estado, StatusCode::NOT_FOUND);
    let (estado, vista_b) = e
        .pedir(
            Method::GET,
            &format!("/respuestas/{hoja_b}"),
            Some(&token_b),
            None,
        )
        .await;
    assert_eq!(estado, StatusCode::OK);
    assert_eq!(vista_b["estado"], "creado");
    assert_eq!(vista_b["evaluacion"]["examenes"][0]["preguntas"], json!([]));
    assert!(vista_b["_links"]["empezar"].is_object(), "{vista_b}");

    // SEC-04: tras /logout el mismo token deja de autenticar.
    let (estado, _) = e
        .pedir(Method::GET, "/respuestas", Some(&token_b), None)
        .await;
    assert_eq!(estado, StatusCode::OK);
    let (estado, _) = e.pedir(Method::POST, "/logout", Some(&token_b), None).await;
    assert_eq!(estado, StatusCode::NO_CONTENT);
    let (estado, _) = e
        .pedir(Method::GET, "/respuestas", Some(&token_b), None)
        .await;
    assert_eq!(estado, StatusCode::UNAUTHORIZED);
    // Cerrar sesión dos veces no es un error.
    let (estado, _) = e.pedir(Method::POST, "/logout", Some(&token_b), None).await;
    assert_eq!(estado, StatusCode::NO_CONTENT);

    // Un inicio de sesión nuevo reemplaza la sesión anterior.
    let token_b2 = e.token("72222222", &clave_b).await;
    let token_b3 = e.token("72222222", &clave_b).await;
    let (estado, _) = e
        .pedir(Method::GET, "/respuestas", Some(&token_b2), None)
        .await;
    assert_eq!(estado, StatusCode::UNAUTHORIZED);
    let (estado, _) = e
        .pedir(Method::GET, "/respuestas", Some(&token_b3), None)
        .await;
    assert_eq!(estado, StatusCode::OK);

    // SEC-08: tras varios fallos para un documento, ni la contraseña correcta entra (429).
    for _ in 0..MAX_FALLOS_POR_DOCUMENTO {
        let (estado, _) = e.login("72222222", "clave-erronea").await;
        assert_eq!(estado, StatusCode::UNAUTHORIZED);
    }
    let (estado, _) = e.login("72222222", &clave_b).await;
    assert_eq!(estado, StatusCode::TOO_MANY_REQUESTS);

    // Y desde una misma IP hay un máximo de intentos por ventana, existan o no los documentos.
    let mut limitado = false;
    for i in 0..MAX_INTENTOS_POR_IP {
        let (estado, _) = e.login(&format!("7{i:07}"), "clave-erronea").await;
        if estado == StatusCode::TOO_MANY_REQUESTS {
            limitado = true;
            break;
        }
        assert_eq!(estado, StatusCode::UNAUTHORIZED);
    }
    assert!(limitado);
}
