use validator::ValidationErrors;

#[derive(Debug)]
pub enum PortofolioError {
    NotFound(String),
    Validation(ValidationErrors),
    System(String),
}

impl From<String> for PortofolioError {
    fn from(s: String) -> Self {
        PortofolioError::System(s)
    }
}
