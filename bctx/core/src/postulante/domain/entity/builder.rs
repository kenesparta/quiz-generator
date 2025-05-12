use crate::postulante::domain::entity::postulante::Postulante;
use crate::postulante::domain::value_object::documento::Documento;
use crate::postulante::domain::value_object::genero::Genero;
use crate::postulante::domain::value_object::grado_instruccion::GradoInstruccion;
use crate::postulante::domain::value_object::id::PostulanteID;
use crate::postulante::domain::value_object::nombre::Nombre;
use crate::postulante::domain::value_object::password::Password;
use quizz_common::domain::value_objects::fecha_nacimiento::FechaNacimiento;

#[derive(Debug)]
pub struct PostulanteBuilder {
    id: Option<PostulanteID>,
    documento: Option<Documento>,
    nombre_completo: Option<Nombre>,
    fecha_nacimiento: Option<FechaNacimiento>,
    grado_instruccion: Option<GradoInstruccion>,
    genero: Option<Genero>,
    password: Option<Password>,
}

impl PostulanteBuilder {
    // Constructor
    pub fn new() -> Self {
        Self {
            id: None,
            documento: None,
            nombre_completo: None,
            fecha_nacimiento: None,
            grado_instruccion: None,
            genero: None,
            password: None,
        }
    }

    pub fn with_id(mut self, id: PostulanteID) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_documento(mut self, documento: Documento) -> Self {
        self.documento = Some(documento);
        self
    }

    pub fn with_nombre_completo(mut self, nombre_completo: Nombre) -> Self {
        self.nombre_completo = Some(nombre_completo);
        self
    }

    pub fn with_fecha_nacimiento(mut self, fecha_nacimiento: FechaNacimiento) -> Self {
        self.fecha_nacimiento = Some(fecha_nacimiento);
        self
    }

    pub fn with_grado_instruccion(mut self, grado_instruccion: GradoInstruccion) -> Self {
        self.grado_instruccion = Some(grado_instruccion);
        self
    }

    pub fn with_genero(mut self, genero: Genero) -> Self {
        self.genero = Some(genero);
        self
    }

    pub fn with_password(mut self, password: Password) -> Self {
        self.password = Some(password);
        self
    }

    // // Build method
    // pub fn build(self) -> Result<Postulante, String> {
    //     let id = self.id.ok_or("Postulante ID is required")?;
    //     let documento = self.documento.ok_or("Documento is required")?;
    //     let nombre_completo = self.nombre_completo.ok_or("Nombre completo is required")?;
    //     let fecha_nacimiento = self.fecha_nacimiento.ok_or("Fecha de nacimiento is required")?;
    //     let grado_instruccion = self.grado_instruccion.ok_or("Grado de instrucción is required")?;
    //     let genero = self.genero.ok_or("Género is required")?;
    //     let password = self.password.ok_or("Password is required")?;
    //
    //     Ok(Postulante {
    //         id,
    //         documento,
    //         nombre_completo,
    //         fecha_nacimiento,
    //         grado_instruccion,
    //         genero,
    //         password,
    //     })
    // }
}
