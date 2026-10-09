//! bcrypt fuera del ejecutor async.
//!
//! bcrypt es lento a propósito (~240 ms con el coste por defecto, 12). Cada worker de actix-web
//! es un runtime de un solo hilo: calcular el hash dentro de un `async fn` congela todas las
//! peticiones de ese worker (se midieron esperas de 1,85 s y 500 espurios bajo carga).
//! `spawn_blocking` lo lleva al pool de hilos bloqueantes de Tokio. Este es el único sitio
//! que llama a bcrypt, así ningún llamador puede olvidarlo.
use quizz_common::provider::seguridad::{CifradoError, Cifrador};
use tracing::error;

/// Hash de coste 12 (el de producción) de una contraseña que no usa nadie: verificar contra él
/// cuesta lo mismo que verificar contra el hash de un usuario real.
const HASH_FICTICIO: &str = "$2y$12$nb2ULX2HIe/EQvAag7YZi.5T4Qm1a9gXGz38mJ8J.h4ivAu5IUTj.";

#[derive(Clone, Copy)]
pub struct Bcrypt {
    coste: u32,
}

impl Default for Bcrypt {
    fn default() -> Self {
        Self {
            coste: bcrypt::DEFAULT_COST,
        }
    }
}

impl Bcrypt {
    /// Un coste distinto del de producción; solo tiene sentido en tests.
    #[cfg(test)]
    pub fn con_coste(coste: u32) -> Self {
        Self { coste }
    }
}

fn tarea_fallida(e: tokio::task::JoinError) -> CifradoError {
    error!("bcrypt, la tarea bloqueante no termino: {e}");
    CifradoError::Tarea
}

impl Cifrador for Bcrypt {
    async fn cifrar(&self, password: String) -> Result<String, CifradoError> {
        let coste = self.coste;
        tokio::task::spawn_blocking(move || bcrypt::hash(password, coste))
            .await
            .map_err(tarea_fallida)?
            .map_err(|e| {
                error!("bcrypt, calcular el hash: {e}");
                CifradoError::Hash
            })
    }

    async fn verificar(&self, password: String, hash: String) -> Result<bool, CifradoError> {
        tokio::task::spawn_blocking(move || bcrypt::verify(password, &hash))
            .await
            .map_err(tarea_fallida)?
            // Sin el detalle: el error de un hash ilegible lo incluye, y un registro antiguo
            // podría guardar ahí la contraseña en claro.
            .map_err(|_| {
                error!("bcrypt, el hash guardado no es valido");
                CifradoError::HashNoValido
            })
    }

    async fn simular_verificacion(&self, password: String) {
        // El resultado no importa: solo se iguala el tiempo de respuesta.
        let _ = self.verificar(password, HASH_FICTICIO.to_string()).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[tokio::test]
    async fn un_hash_verifica_su_contraseña_y_no_otra() {
        let bcrypt = Bcrypt::con_coste(4);
        let hash = bcrypt.cifrar("clave-correcta".to_string()).await.unwrap();
        assert!(hash.starts_with("$2"));
        assert!(
            bcrypt
                .verificar("clave-correcta".to_string(), hash.clone())
                .await
                .unwrap()
        );
        assert!(
            !bcrypt
                .verificar("otra-clave".to_string(), hash)
                .await
                .unwrap()
        );
    }

    #[tokio::test]
    async fn el_hash_ficticio_es_un_bcrypt_de_coste_de_produccion() {
        let partes: Vec<&str> = HASH_FICTICIO.split('$').collect();
        assert_eq!(partes[2], bcrypt::DEFAULT_COST.to_string());
        let verificado = Bcrypt::default()
            .verificar("cualquiera".to_string(), HASH_FICTICIO.to_string())
            .await;
        assert!(matches!(verificado, Ok(false)));
    }

    #[tokio::test]
    async fn un_hash_guardado_no_valido_es_un_error() {
        let resultado = Bcrypt::con_coste(4)
            .verificar("clave".to_string(), "no-es-un-hash".to_string())
            .await;
        assert!(matches!(resultado, Err(CifradoError::HashNoValido)));
    }

    /// En un runtime de un solo hilo, como el de cada worker de actix, otra tarea debe poder
    /// avanzar mientras se calcula el hash. Con bcrypt dentro del `async fn` este contador se
    /// queda en 0.
    #[tokio::test(flavor = "current_thread")]
    async fn el_hash_no_bloquea_el_ejecutor() {
        let avances = Arc::new(AtomicUsize::new(0));
        let contador = Arc::clone(&avances);
        let otra_tarea = tokio::spawn(async move {
            loop {
                contador.fetch_add(1, Ordering::Relaxed);
                tokio::task::yield_now().await;
            }
        });

        Bcrypt::con_coste(8)
            .cifrar("una-clave".to_string())
            .await
            .unwrap();
        otra_tarea.abort();

        assert!(avances.load(Ordering::Relaxed) > 0);
    }
}
