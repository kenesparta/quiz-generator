//! # Evaluación Respuesta
//!
//! **Tipo de Subdominio:** Soporte
//!
//! Este crate gestiona las respuestas de los postulantes a las evaluaciones.
//! Conecta el subdominio de postulantes con el core de creación de evaluaciones.
//!
//! ## Conceptos de Dominio
//!
//! - **Respuesta**: Registro que vincula un postulante con una evaluación asignada
//! - **Estado**: Máquina de estados (Creado -> EnProceso -> Finalizado)
//! - **Revisión**: Proceso de calificación de respuestas finalizadas
//!
//! ## Ciclo de Vida de una Respuesta
//!
//! 1. **Creado**: Evaluación asignada al postulante
//! 2. **EnProceso**: Postulante inició el examen (`fecha_tiempo_inicio` establecida)
//! 3. **Finalizado**: Postulante completó el examen (`fecha_tiempo_fin` establecida)
//!
//! ## Responsabilidades
//!
//! - Asignación de evaluaciones a postulantes
//! - Control del flujo de inicio/fin de exámenes
//! - Almacenamiento de respuestas individuales
//! - Soporte para el proceso de revisión y calificación
