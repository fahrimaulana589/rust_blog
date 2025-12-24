use crate::app::features::profile::domain::entity::NewProfile;
use crate::app::features::profile::domain::repository::ProfileRepository;
use crate::app::features::profile::interface::dto::{ProfileResponseDto, UpsertProfileRequestDto};
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    repository: Arc<dyn ProfileRepository>,
}

impl Execute {
    pub fn new(repository: Arc<dyn ProfileRepository>) -> Self {
        Self { repository }
    }

    pub fn execute(&self, request: UpsertProfileRequestDto) -> Result<ProfileResponseDto, String> {
        let profile_data = NewProfile {
            full_name: request.full_name,
            headline: request.headline,
            summary: request.summary,
            role: request.role,
            location: request.location,
            profile_image: request.profile_image,
            availability: request.availability,
            years_of_experience: request.years_of_experience,
            resume_url: request.resume_url,
            email: request.email,
            work_philosophy: request.work_philosophy,
            timezone: request.timezone,
        };

        let languages_tuple: Vec<(String, String)> = request
            .languages
            .iter()
            .map(|l| (l.name.clone(), l.level.clone()))
            .collect();

        // Transaction is handled in repository
        let profile = self
            .repository
            .upsert_profile(
                profile_data,
                request.specializations.clone(),
                request.tech_focus.clone(),
                languages_tuple,
            )
            .map_err(|e| e.to_string())?;

        // Reconstruct response (or fetch again if needed, but we can just use the input + id if we trust it, but repository returns Profile)
        // Since we replaced the relations, we can just return what we passed in, or query again.
        // Repository returns Just Profile entity.
        // But the user expects the full ProfileResponse.
        // Ideally we should just return what we sent, because we know it succeeded.

        let response = ProfileResponseDto {
            full_name: profile.full_name,
            headline: profile.headline,
            summary: profile.summary,
            role: profile.role,
            location: profile.location,
            profile_image: profile.profile_image,
            availability: profile.availability,
            years_of_experience: profile.years_of_experience,
            resume_url: profile.resume_url,
            email: profile.email,
            work_philosophy: profile.work_philosophy,
            timezone: profile.timezone,
            specializations: request.specializations,
            tech_focus: request.tech_focus,
            languages: request.languages,
        };

        Ok(response)
    }
}
