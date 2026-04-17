use crate::postulante::domain::entity::postulante::Postulante;
use crate::postulante::domain::error::postulante::PostulanteError;
use crate::postulante::domain::value_object::genero::Genero;
use crate::postulante::domain::value_object::grado_instruccion::GradoInstruccion;
use crate::postulante::domain::value_object::id::PostulanteID;
use crate::postulante::domain::value_object::nombre::Nombre;
use crate::postulante::provider::repositorio::{
    RepositorioPostulanteEscritura, RepositorioPostulanteLectura,
};
use async_trait::async_trait;
use quizz_common::domain::value_objects::fecha_nacimiento::FechaNacimiento;
use quizz_common::use_case::CasoDeUso;
use std::str::FromStr;

pub struct InputData {
    pub id: String,
    pub documento: String,
    pub nombre: String,
    pub primer_apellido: String,
    pub segundo_apellido: String,
    pub fecha_nacimiento: String,
    pub grado_instruccion: String,
    pub genero: String,
}

pub struct ActualizarPostulante<ReadErr, WriteErr> {
    repositorio_lectura: Box<dyn RepositorioPostulanteLectura<ReadErr>>,
    repositorio_escritura: Box<dyn RepositorioPostulanteEscritura<WriteErr>>,
}

impl<ReadErr, WriteErr> ActualizarPostulante<ReadErr, WriteErr> {
    pub fn new(
        repositorio_lectura: Box<dyn RepositorioPostulanteLectura<ReadErr>>,
        repositorio_escritura: Box<dyn RepositorioPostulanteEscritura<WriteErr>>,
    ) -> Self {
        Self {
            repositorio_lectura,
            repositorio_escritura,
        }
    }
}

#[async_trait]
impl<ReadErr, WriteErr> CasoDeUso<InputData, (), PostulanteError>
    for ActualizarPostulante<ReadErr, WriteErr>
where
    PostulanteError: From<ReadErr>,
    PostulanteError: From<WriteErr>,
{
    async fn ejecutar(&self, in_: InputData) -> Result<(), PostulanteError> {
        let postulante_id = PostulanteID::new(&in_.id)?;
        let existente = self
            .repositorio_lectura
            .obtener_postulante_por_id(postulante_id)
            .await?;

        let grado_instruccion = GradoInstruccion::from_str(&in_.grado_instruccion)?;
        let genero = Genero::from_str(&in_.genero)?;
        let nombre_completo = Nombre::new(in_.nombre, in_.primer_apellido, in_.segundo_apellido)?;
        let fecha_nacimiento = FechaNacimiento::new(&in_.fecha_nacimiento)?;

        let postulante_actualizado = Postulante {
            id: existente.id,
            documento: existente.documento,
            nombre_completo,
            fecha_nacimiento,
            grado_instruccion,
            genero,
            password: existente.password,
            fecha_registro: existente.fecha_registro,
        };

        self.repositorio_escritura
            .actualizar_postulante(postulante_actualizado)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::postulante::domain::value_object::documento::Documento;
    use async_trait::async_trait;
    use quizz_common::domain::value_objects::fecha_registro::FechaRegistro;

    struct MockRepositorioLectura {
        postulante: Postulante,
    }

    #[async_trait]
    impl RepositorioPostulanteLectura<PostulanteError> for MockRepositorioLectura {
        async fn obtener_postulante_por_documento(
            &self,
            _documento: Documento,
        ) -> Result<Postulante, PostulanteError> {
            unimplemented!()
        }

        async fn obtener_postulante_por_id(
            &self,
            _postulante_id: PostulanteID,
        ) -> Result<Postulante, PostulanteError> {
            Ok(Postulante {
                id: PostulanteID::new(&self.postulante.id.value().uuid().to_string())?,
                documento: Documento::new(&self.postulante.documento.to_string())?,
                nombre_completo: Nombre::new(
                    self.postulante.nombre_completo.nombre().clone(),
                    self.postulante.nombre_completo.primer_apellido().clone(),
                    self.postulante.nombre_completo.segundo_apellido().clone(),
                )?,
                fecha_nacimiento: FechaNacimiento::new(
                    &self.postulante.fecha_nacimiento.to_string(),
                )?,
                grado_instruccion: self.postulante.grado_instruccion.clone(),
                genero: self.postulante.genero.clone(),
                password: None,
                fecha_registro: FechaRegistro::ahora(),
            })
        }

        async fn obtener_lista_de_postulantes(&self) -> Result<Vec<Postulante>, PostulanteError> {
            unimplemented!()
        }
    }

    struct MockRepositorioEscritura;

    #[async_trait]
    impl RepositorioPostulanteEscritura<PostulanteError> for MockRepositorioEscritura {
        async fn registrar_postulante(
            &self,
            _postulante: Postulante,
        ) -> Result<(), PostulanteError> {
            unimplemented!()
        }

        async fn actualizar_postulante(
            &self,
            _postulante: Postulante,
        ) -> Result<(), PostulanteError> {
            Ok(())
        }

        async fn eliminar_postulante(
            &self,
            _postulante_id: PostulanteID,
        ) -> Result<(), PostulanteError> {
            unimplemented!()
        }
    }

    fn crear_postulante_existente() -> Postulante {
        Postulante::new(
            "22d1adea-d489-486b-badf-8e0580ddd0c3".to_string(),
            "12345678".to_string(),
            "John".to_string(),
            "Doe".to_string(),
            "Smith".to_string(),
            "1990-01-01".to_string(),
            GradoInstruccion::Primaria,
            Genero::Masculino,
            "$2a$12$b0a7aabc6PcLyAMKifb3pOCSwi8zgqf0ylujb8DgF3I1r.xn.Mrn2".to_string(),
        )
        .unwrap()
    }

    #[tokio::test]
    async fn test_actualizar_postulante_success() {
        let postulante = crear_postulante_existente();

        let use_case = ActualizarPostulante::new(
            Box::new(MockRepositorioLectura { postulante }),
            Box::new(MockRepositorioEscritura),
        );

        let result = use_case
            .ejecutar(InputData {
                id: "22d1adea-d489-486b-badf-8e0580ddd0c3".to_string(),
                documento: "12345678".to_string(),
                nombre: "Carlos".to_string(),
                primer_apellido: "García".to_string(),
                segundo_apellido: "López".to_string(),
                fecha_nacimiento: "1991-05-15".to_string(),
                grado_instruccion: "SUPERIOR".to_string(),
                genero: "MASCULINO".to_string(),
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_actualizar_postulante_invalid_id() {
        let postulante = crear_postulante_existente();

        let use_case = ActualizarPostulante::new(
            Box::new(MockRepositorioLectura { postulante }),
            Box::new(MockRepositorioEscritura),
        );

        let result = use_case
            .ejecutar(InputData {
                id: "invalid-id".to_string(),
                documento: "12345678".to_string(),
                nombre: "Carlos".to_string(),
                primer_apellido: "García".to_string(),
                segundo_apellido: "López".to_string(),
                fecha_nacimiento: "1991-05-15".to_string(),
                grado_instruccion: "SUPERIOR".to_string(),
                genero: "MASCULINO".to_string(),
            })
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_actualizar_postulante_invalid_genero() {
        let postulante = crear_postulante_existente();

        let use_case = ActualizarPostulante::new(
            Box::new(MockRepositorioLectura { postulante }),
            Box::new(MockRepositorioEscritura),
        );

        let result = use_case
            .ejecutar(InputData {
                id: "22d1adea-d489-486b-badf-8e0580ddd0c3".to_string(),
                documento: "12345678".to_string(),
                nombre: "Carlos".to_string(),
                primer_apellido: "García".to_string(),
                segundo_apellido: "López".to_string(),
                fecha_nacimiento: "1991-05-15".to_string(),
                grado_instruccion: "SUPERIOR".to_string(),
                genero: "INVALID".to_string(),
            })
            .await;

        assert!(result.is_err());
    }
}
