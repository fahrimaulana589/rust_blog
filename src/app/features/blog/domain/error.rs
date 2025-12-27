use validator::ValidationErrors;

#[derive(Debug)]
pub enum BlogError {
    Validation(ValidationErrors),
    System(String),
    NotFound(String),
}

impl From<String> for BlogError {
    fn from(s: String) -> Self {
        BlogError::System(s)
    }
}
