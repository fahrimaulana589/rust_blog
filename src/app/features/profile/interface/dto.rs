use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub struct LanguageDto {
    pub name: String,
    pub level: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub struct ProfileResponseDto {
    pub full_name: String,
    pub headline: String,
    pub summary: String,
    pub role: String,
    pub location: String,
    pub profile_image: String,
    pub availability: String,
    pub years_of_experience: i32,
    pub resume_url: String,
    pub email: String,
    pub work_philosophy: String,
    pub timezone: String,
    pub specializations: Vec<String>,
    pub tech_focus: Vec<String>,
    pub languages: Vec<LanguageDto>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate, Clone)]
pub struct UpsertProfileRequestDto {
    #[validate(length(min = 1, message = "Full name is required"))]
    pub full_name: String,
    pub headline: String,
    pub summary: String,
    pub role: String,
    pub location: String,
    pub profile_image: String,
    pub availability: String,
    pub years_of_experience: i32,
    pub resume_url: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    pub work_philosophy: String,
    pub timezone: String,
    pub specializations: Vec<String>,
    pub tech_focus: Vec<String>,
    pub languages: Vec<LanguageDto>,
}
