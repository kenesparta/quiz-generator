use async_trait::async_trait;
use bcrypt::{DEFAULT_COST, hash};
use quizz_core::admin::domain::error::admin::AdminError;
use quizz_core::admin::provider::password::SeguridadPasswordAdmin;

pub struct CifradoAdmin;

#[async_trait]
impl SeguridadPasswordAdmin<AdminError> for CifradoAdmin {
    async fn cifrar(&self, password: String) -> Result<String, AdminError> {
        hash(password, DEFAULT_COST).map_err(|_| AdminError::PasswordVacio)
    }
}
