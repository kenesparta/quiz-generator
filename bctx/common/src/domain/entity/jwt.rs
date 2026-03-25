#[derive(Debug, Clone)]
pub struct JwtObject {
    pub key: String,
    pub value: String,
    pub expiration: u64,
    pub rol: Option<String>,
}
