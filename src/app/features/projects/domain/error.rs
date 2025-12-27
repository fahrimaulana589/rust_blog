use validator::ValidationErrors;

#[derive(Debug)]
pub enum ProjectError {
    Validation(ValidationErrors),
    System(String),
    NotFound(String),
}

impl From<String> for ProjectError {
    fn from(s: String) -> Self {
        ProjectError::System(s)
    }
}
