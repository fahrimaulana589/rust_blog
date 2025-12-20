use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResponse<T> {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> SuccessResponse<T> {
    pub fn new(message: String, data: Option<T>) -> Self {
        Self { message, data }
    }
}

pub fn map_success_response(message: String) -> SuccessResponse<()> {
    SuccessResponse::new(message, None)
}

pub fn map_success_with_data<T>(message: String, data: T) -> SuccessResponse<T> {
    SuccessResponse::new(message, Some(data))
}
