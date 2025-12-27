use validator::ValidationErrors;

#[derive(Debug)]
pub enum PortfolioError {
    NotFound(String),
    Validation(ValidationErrors),
    System(String),
}

impl From<String> for PortfolioError {
    fn from(s: String) -> Self {
        PortfolioError::System(s)
    }
}
