pub trait PasswordCrypto<E> {
    fn cifrar(&self, password: String) -> Result<String, E>;
    fn comparar(&self, password: String, hashed: String) -> Result<bool, E>;
}
