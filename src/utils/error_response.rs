use serde::Serialize;
use std::collections::HashMap;
use validator::ValidationErrors;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
    pub errors: Option<HashMap<String, String>>, // Field -> Message
}

impl ErrorResponse {
    pub fn new(message: String) -> Self {
        Self {
            message,
            errors: None,
        }
    }
}

pub fn map_string_error(e: String) -> ErrorResponse {
    ErrorResponse::new(e)
}

pub fn map_validation_error(e: ValidationErrors) -> ErrorResponse {
    let mut errors = HashMap::new();
    for (field, errs) in e.field_errors() {
        if let Some(first_err) = errs.first() {
            // Use code if message is not available, or default
            let msg = first_err
                .message
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or_else(|| first_err.code.to_string());
            errors.insert(field.to_string(), msg);
        }
    }

    ErrorResponse {
        message: "Validation Error".to_string(),
        errors: Some(errors),
    }
}

use actix_web::{HttpRequest, HttpResponse, error};

pub fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    let response = ErrorResponse {
        message: "Invalid Input".to_string(),
        errors: Some(HashMap::from([("json".to_string(), err.to_string())])),
    };
    error::InternalError::from_response(err, HttpResponse::BadRequest().json(response)).into()
}
