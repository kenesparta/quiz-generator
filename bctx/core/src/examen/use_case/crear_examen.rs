use crate::examen::domain::entity::examen::Examen;
use crate::examen::domain::error::examen::ExamenError;
use crate::examen::provider::repositorio::RepositorioExamenEscritura;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;
use std::str::FromStr;

pub struct InputData {
    pub id: String,
    pub titulo: String,
    pub descripcion: String,
    pub duracion_minutos: u32,
    pub puntos_totales: u32,
    pub categoria: String,
    pub nivel_dificultad: String,
    pub preguntas_ids: Vec<String>,
    pub activo: bool,
}

pub struct OutputData {
    pub id: String,
}

pub struct CrearExamen<RepoErr> {
    repositorio: Box<dyn RepositorioExamenEscritura<RepoErr>>,
}

impl<RepoErr> CrearExamen<RepoErr> {
    pub fn new(repositorio: Box<dyn RepositorioExamenEscritura<RepoErr>>) -> CrearExamen<RepoErr> {
        Self { repositorio }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, OutputData, ExamenError> for CrearExamen<RepoErr>
where
    ExamenError: From<RepoErr>,
{
    async fn ejecutar(&self, in_: InputData) -> Result<OutputData, ExamenError> {
        // Validar entrada de datos
        if in_.puntos_totales == 0 {
            return Err(ExamenError::PuntosTotalesInvalidos);
        }

        if in_.duracion_minutos < 10 || in_.duracion_minutos > 240 {
            return Err(ExamenError::DuracionInvalida);
        }

        if in_.preguntas_ids.is_empty() {
            return Err(ExamenError::SinPreguntas);
        }

        // Convertir string a enum NivelDificultad (asumiendo que existe este enum)
        let nivel_dificultad = match in_.nivel_dificultad.as_str() {
            "FACIL" => "FACIL",
            "MEDIO" => "MEDIO",
            "DIFICIL" => "DIFICIL",
            _ => return Err(ExamenError::NivelDificultadInvalido),
        };

        // Crear entidad de examen
        let examen = Examen::new(
            in_.id.clone(),
            in_.titulo,
            in_.descripcion,
            in_.duracion_minutos,
            in_.puntos_totales,
            in_.categoria,
            nivel_dificultad.to_string(),
            in_.preguntas_ids,
            in_.activo,
        )?;

        // Guardar en el repositorio
        self.repositorio.guardar_examen(examen).await?;

        Ok(OutputData { id: in_.id })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::examen::domain::entity::examen::Examen;
    use crate::examen::domain::value_object::id::ExamenID;
    use async_trait::async_trait;
    use std::sync::Mutex;

    struct MockRepositorioExamen {
        _examen: Mutex<Option<Examen>>,
        _result: Result<(), ExamenError>,
    }

    #[async_trait]
    impl RepositorioExamenEscritura<ExamenError> for MockRepositorioExamen {
        async fn guardar_examen(&self, examen: Examen) -> Result<(), ExamenError> {
            let mut data = self._examen.lock().unwrap();
            *data = Some(examen);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_crear_examen_success() {
        let repositorio = Box::new(MockRepositorioExamen {
            _examen: Mutex::new(None),
            _result: Ok(()),
        });

        let use_case = CrearExamen::new(repositorio);

        let result = use_case
            .ejecutar(InputData {
                id: "22d1adea-d489-486b-badf-8e0580ddd0c3".to_string(),
                titulo: "Examen de Matemáticas".to_string(),
                descripcion: "Examen parcial de matemáticas".to_string(),
                duracion_minutos: 60,
                puntos_totales: 100,
                categoria: "MATEMATICAS".to_string(),
                nivel_dificultad: "MEDIO".to_string(),
                preguntas_ids: vec![
                    "33d1adea-d489-486b-badf-8e0580ddd0c3".to_string(),
                    "44d1adea-d489-486b-badf-8e0580ddd0c3".to_string(),
                ],
                activo: true,
            })
            .await;

        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.id, "22d1adea-d489-486b-badf-8e0580ddd0c3");
    }

    #[tokio::test]
    async fn test_crear_examen_sin_preguntas() {
        let repositorio = Box::new(MockRepositorioExamen {
            _examen: Mutex::new(None),
            _result: Ok(()),
        });

        let use_case = CrearExamen::new(repositorio);

        let result = use_case
            .ejecutar(InputData {
                id: "22d1adea-d489-486b-badf-8e0580ddd0c3".to_string(),
                titulo: "Examen de Matemáticas".to_string(),
                descripcion: "Examen parcial de matemáticas".to_string(),
                duracion_minutos: 60,
                puntos_totales: 100,
                categoria: "MATEMATICAS".to_string(),
                nivel_dificultad: "MEDIO".to_string(),
                preguntas_ids: vec![],
                activo: true,
            })
            .await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ExamenError::SinPreguntas));
    }

    #[tokio::test]
    async fn test_crear_examen_duracion_invalida() {
        let repositorio = Box::new(MockRepositorioExamen {
            _examen: Mutex::new(None),
            _result: Ok(()),
        });

        let use_case = CrearExamen::new(repositorio);

        let result = use_case
            .ejecutar(InputData {
                id: "22d1adea-d489-486b-badf-8e0580ddd0c3".to_string(),
                titulo: "Examen de Matemáticas".to_string(),
                descripcion: "Examen parcial de matemáticas".to_string(),
                duracion_minutos: 5, // Muy corto
                puntos_totales: 100,
                categoria: "MATEMATICAS".to_string(),
                nivel_dificultad: "MEDIO".to_string(),
                preguntas_ids: vec!["33d1adea-d489-486b-badf-8e0580ddd0c3".to_string()],
                activo: true,
            })
            .await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ExamenError::DuracionInvalida));
    }

    #[tokio::test]
    async fn test_crear_examen_puntos_invalidos() {
        let repositorio = Box::new(MockRepositorioExamen {
            _examen: Mutex::new(None),
            _result: Ok(()),
        });

        let use_case = CrearExamen::new(repositorio);

        let result = use_case
            .ejecutar(InputData {
                id: "22d1adea-d489-486b-badf-8e0580ddd0c3".to_string(),
                titulo: "Examen de Matemáticas".to_string(),
                descripcion: "Examen parcial de matemáticas".to_string(),
                duracion_minutos: 60,
                puntos_totales: 0, // Puntos inválidos
                categoria: "MATEMATICAS".to_string(),
                nivel_dificultad: "MEDIO".to_string(),
                preguntas_ids: vec!["33d1adea-d489-486b-badf-8e0580ddd0c3".to_string()],
                activo: true,
            })
            .await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ExamenError::PuntosTotalesInvalidos
        ));
    }

    #[tokio::test]
    async fn test_crear_examen_nivel_dificultad_invalido() {
        let repositorio = Box::new(MockRepositorioExamen {
            _examen: Mutex::new(None),
            _result: Ok(()),
        });

        let use_case = CrearExamen::new(repositorio);

        let result = use_case
            .ejecutar(InputData {
                id: "22d1adea-d489-486b-badf-8e0580ddd0c3".to_string(),
                titulo: "Examen de Matemáticas".to_string(),
                descripcion: "Examen parcial de matemáticas".to_string(),
                duracion_minutos: 60,
                puntos_totales: 100,
                categoria: "MATEMATICAS".to_string(),
                nivel_dificultad: "IMPOSIBLE".to_string(), // Nivel inválido
                preguntas_ids: vec!["33d1adea-d489-486b-badf-8e0580ddd0c3".to_string()],
                activo: true,
            })
            .await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ExamenError::NivelDificultadInvalido
        ));
    }
}
