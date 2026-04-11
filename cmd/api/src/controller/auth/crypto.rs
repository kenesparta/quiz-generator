use async_trait::async_trait;
use bcrypt::verify;
use quizz_auth::universal::domain::error::login_universal::LoginUniversalError;
use quizz_common::provider::seguridad::SeguridadComparar;

pub struct CifradoPorDefecto;

#[async_trait]
impl SeguridadComparar<LoginUniversalError> for CifradoPorDefecto {
    async fn comparar(
        &self,
        password: String,
        hashed: String,
    ) -> Result<(), LoginUniversalError> {
        match verify(password, &hashed) {
            Ok(true) => Ok(()),
            _ => Err(LoginUniversalError::PasswordIncorrecto),
        }
    }
}
