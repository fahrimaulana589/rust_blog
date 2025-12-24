use crate::app::features::profile::domain::entity::{
    Profile, ProfileLanguage, ProfileSpecialization, ProfileTechFocus,
};
use diesel::QueryResult;

pub trait ProfileRepository: Send + Sync {
    fn get_profile(
        &self,
    ) -> QueryResult<
        Option<(
            Profile,
            Vec<ProfileSpecialization>,
            Vec<ProfileTechFocus>,
            Vec<ProfileLanguage>,
        )>,
    >;
    fn upsert_profile(
        &self,
        profile_data: super::entity::NewProfile,
        specializations: Vec<String>,
        tech_focus: Vec<String>,
        languages: Vec<(String, String)>, // name, level
    ) -> QueryResult<Profile>;
}
