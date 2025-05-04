pub struct Password {
    value: String,
}

impl Password {
    pub fn new() -> Self {
        let mut pass = Password {
            value: String::new(),
        };
        pass.generate_password();
        pass
    }

    fn generate_password(&mut self) {
        self.value = String::new();
    }

    pub fn value(self) -> String {
        self.value
    }

    fn generate_password_from_document(&mut self, document: &str) {
        self.value = String::new();
    }
}
