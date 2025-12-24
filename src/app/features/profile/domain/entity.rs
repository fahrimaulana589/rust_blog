use crate::schema::{profile_languages, profile_specializations, profile_tech_focus, profiles};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Debug, Serialize, Deserialize, Clone)]
#[diesel(table_name = profiles)]
pub struct Profile {
    pub id: i32,
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
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Debug, Clone)]
#[diesel(table_name = profiles)]
pub struct NewProfile {
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
}

#[derive(
    Queryable, Selectable, Identifiable, Associations, Debug, Serialize, Deserialize, Clone,
)]
#[diesel(belongs_to(Profile))]
#[diesel(table_name = profile_specializations)]
pub struct ProfileSpecialization {
    pub id: i32,
    pub profile_id: i32,
    pub specialization: String,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = profile_specializations)]
pub struct NewProfileSpecialization {
    pub profile_id: i32,
    pub specialization: String,
}

#[derive(
    Queryable, Selectable, Identifiable, Associations, Debug, Serialize, Deserialize, Clone,
)]
#[diesel(belongs_to(Profile))]
#[diesel(table_name = profile_tech_focus)]
pub struct ProfileTechFocus {
    pub id: i32,
    pub profile_id: i32,
    pub tech_focus: String,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = profile_tech_focus)]
pub struct NewProfileTechFocus {
    pub profile_id: i32,
    pub tech_focus: String,
}

#[derive(
    Queryable, Selectable, Identifiable, Associations, Debug, Serialize, Deserialize, Clone,
)]
#[diesel(belongs_to(Profile))]
#[diesel(table_name = profile_languages)]
pub struct ProfileLanguage {
    pub id: i32,
    pub profile_id: i32,
    pub name: String,
    pub level: String,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = profile_languages)]
pub struct NewProfileLanguage {
    pub profile_id: i32,
    pub name: String,
    pub level: String,
}
