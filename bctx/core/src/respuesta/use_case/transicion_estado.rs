//! Fakes compartidos por los tests de las transiciones de estado.
#![cfg(test)]

use crate::postulante::domain::value_object::id::PostulanteID;
use crate::respuesta::domain::entity::respuesta::Estado;
use crate::respuesta::domain::error::respuesta::RespuestaError;
use crate::respuesta::domain::value_object::id::RespuestaID;
use crate::respuesta::provider::repositorio::RepositorioEstadoRespuesta;
use async_trait::async_trait;
use chrono::{DateTime, FixedOffset};
use std::sync::{Arc, Mutex};

pub const HOJA: &str = "0b9f1a52-3d4e-4f60-8a7b-9c0d1e2f3a4b";
pub const DUENO: &str = "6f1c2a3b-4d5e-4f60-8a7b-9c0d1e2f3a4b";
pub const OTRO: &str = "d4e92ac2-882a-4e46-ae56-b2361ac5327e";

pub fn ahora() -> DateTime<FixedOffset> {
    DateTime::parse_from_rfc3339("2026-10-08T10:00:00-05:00").unwrap()
}

/// Una hoja de `DUENO` en memoria, con compare-and-set real. Los clones comparten la hoja,
/// así el test conserva uno para inspeccionarla. `perder_carrera` simula que otra petición
/// cambió el estado justo antes de la escritura.
#[derive(Clone)]
pub struct HojaEnMemoria {
    pub estado: Arc<Mutex<Estado>>,
    pub fecha: Arc<Mutex<Option<DateTime<FixedOffset>>>>,
    pub perder_carrera: Option<Estado>,
}

impl HojaEnMemoria {
    pub fn en(estado: Estado) -> Self {
        Self {
            estado: Arc::new(Mutex::new(estado)),
            fecha: Arc::new(Mutex::new(None)),
            perder_carrera: None,
        }
    }

    pub fn estado(&self) -> Estado {
        *self.estado.lock().unwrap()
    }

    pub fn fecha(&self) -> Option<DateTime<FixedOffset>> {
        *self.fecha.lock().unwrap()
    }
}

#[async_trait]
impl RepositorioEstadoRespuesta<RespuestaError> for HojaEnMemoria {
    async fn obtener_estado(
        &self,
        id: &RespuestaID,
        postulante_id: &PostulanteID,
    ) -> Result<Option<Estado>, RespuestaError> {
        let es_la_hoja = id.to_string() == HOJA && postulante_id.to_string() == DUENO;
        Ok(es_la_hoja.then(|| *self.estado.lock().unwrap()))
    }

    async fn transicionar(
        &self,
        id: &RespuestaID,
        postulante_id: &PostulanteID,
        desde: Estado,
        hacia: Estado,
        fecha: DateTime<FixedOffset>,
    ) -> Result<bool, RespuestaError> {
        if let Some(otro) = self.perder_carrera {
            *self.estado.lock().unwrap() = otro;
        }
        let mut estado = self.estado.lock().unwrap();
        if id.to_string() != HOJA || postulante_id.to_string() != DUENO || *estado != desde {
            return Ok(false);
        }
        *estado = hacia;
        *self.fecha.lock().unwrap() = Some(fecha);
        Ok(true)
    }
}
