//! Índices de MongoDB, creados al arrancar (crear un índice que ya existe no hace nada).
//!
//! Los índices únicos hacen cumplir invariantes que la aplicación no puede garantizar con
//! "comprobar y luego escribir" cuando llegan peticiones simultáneas. Si un índice único no se
//! puede crear porque ya hay duplicados, se registra el error y la API arranca igual: hay que
//! limpiar los duplicados (el log indica la colección) y reiniciar.
use mongodb::bson::{Document, doc};
use mongodb::options::IndexOptions;
use mongodb::{Database, IndexModel};
use tracing::{error, info};

struct Indice {
    coleccion: &'static str,
    claves: Document,
    unico: bool,
    motivo: &'static str,
}

fn indices() -> Vec<Indice> {
    vec![
        Indice {
            coleccion: "admin",
            claves: doc! { "documento": 1 },
            unico: true,
            motivo: "un documento identifica a una sola cuenta (y se busca en cada login)",
        },
        Indice {
            coleccion: "psicologo",
            claves: doc! { "documento": 1 },
            unico: true,
            motivo: "un documento identifica a una sola cuenta (y se busca en cada login)",
        },
        Indice {
            coleccion: "postulante",
            claves: doc! { "documento": 1 },
            unico: true,
            motivo: "un documento identifica a una sola cuenta (y se busca en cada login)",
        },
        Indice {
            coleccion: "respuesta",
            claves: doc! { "postulante_id": 1, "evaluacion._id": 1 },
            unico: true,
            motivo: "una evaluacion se asigna una sola vez a cada postulante",
        },
        Indice {
            coleccion: "respuesta",
            claves: doc! { "postulante_id": 1, "estado": 1 },
            unico: false,
            motivo: "hojas de un postulante por estado (GET /respuestas)",
        },
        Indice {
            coleccion: "respuesta",
            claves: doc! { "estado": 1, "fecha_tiempo_fin": -1 },
            unico: false,
            motivo: "hojas finalizadas para revisar (GET /revisiones)",
        },
    ]
}

/// Crea los índices de la aplicación. Los no únicos primero: no fallan por datos existentes.
pub async fn crear_indices(db: &Database) {
    let mut indices = indices();
    indices.sort_by_key(|indice| indice.unico);

    for indice in indices {
        let modelo = IndexModel::builder()
            .keys(indice.claves.clone())
            .options(IndexOptions::builder().unique(indice.unico).build())
            .build();
        match db
            .collection::<Document>(indice.coleccion)
            .create_index(modelo)
            .await
        {
            Ok(_) => info!("indice {} {} listo", indice.coleccion, indice.claves),
            Err(e) => error!(
                "no se pudo crear el indice {} {} ({}): {e}",
                indice.coleccion, indice.claves, indice.motivo
            ),
        }
    }
}
