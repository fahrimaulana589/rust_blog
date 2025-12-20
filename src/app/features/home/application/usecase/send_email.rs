use crate::utils::email::Email;

#[derive(Clone)]
pub struct Execute {
    pub email: Email,
}

impl Execute {
    pub fn new(email: Email) -> Self {
        Self { email }
    }

    pub fn send(&self) -> Result<String, String> {
        match self.email.send_test_email() {
            Ok(message) => Ok(message),
            Err(e) => Err(e),
        }
    }
}
