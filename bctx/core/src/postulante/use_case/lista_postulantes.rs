use crate::postulante::domain::entity::postulante::Postulante;
use crate::postulante::domain::error::postulante::PostulanteError;
use crate::postulante::provider::repositorio::RepositorioPostulanteLectura;

pub struct OutputData {
    pub id: String,
    pub documento: String,
    pub nombre: String,
    pub primer_apellido: String,
    pub segundo_apellido: String,
    pub nombre_completo: String,
    pub fecha_nacimiento: String,
    pub grado_instruccion: String,
    pub genero: String,
    pub fecha_registro: String,
}

pub struct ListaOutput {
    pub postulantes: Vec<OutputData>,
}

impl From<Postulante> for OutputData {
    fn from(p: Postulante) -> Self {
        Self {
            id: p.id.to_string(),
            documento: p.documento.to_string(),
            nombre: p.nombre_completo.nombre().to_string(),
            primer_apellido: p.nombre_completo.primer_apellido().to_string(),
            segundo_apellido: p.nombre_completo.segundo_apellido().to_string(),
            nombre_completo: p.nombre_completo.nombre_completo().to_string(),
            fecha_nacimiento: p.fecha_nacimiento.to_string(),
            grado_instruccion: p.grado_instruccion.to_string(),
            genero: p.genero.to_string(),
            fecha_registro: p.fecha_registro.to_string(),
        }
    }
}

pub struct ObtenerListaDePostulantes<R> {
    repositorio: R,
}

impl<R: RepositorioPostulanteLectura> ObtenerListaDePostulantes<R> {
    pub fn new(repositorio: R) -> Self {
        Self { repositorio }
    }

    pub async fn ejecutar(&self) -> Result<ListaOutput, PostulanteError> {
        let lista_de_postulantes = self.repositorio.obtener_lista_de_postulantes().await?;
        Ok(ListaOutput {
            postulantes: lista_de_postulantes.into_iter().map(|p| p.into()).collect(),
        })
    }
}
