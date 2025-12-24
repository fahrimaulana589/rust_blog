use crate::app::features::profile::domain::repository::ProfileRepository;
use crate::app::features::profile::interface::dto::{LanguageDto, ProfileResponseDto};
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    repository: Arc<dyn ProfileRepository>,
}

impl Execute {
    pub fn new(repository: Arc<dyn ProfileRepository>) -> Self {
        Self { repository }
    }

    pub fn execute(&self) -> Result<Option<ProfileResponseDto>, String> {
        let result = self.repository.get_profile().map_err(|e| e.to_string())?;

        match result {
            Some((profile, specs, tech, langs)) => {
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
                    specializations: specs.into_iter().map(|s| s.specialization).collect(),
                    tech_focus: tech.into_iter().map(|t| t.tech_focus).collect(),
                    languages: langs
                        .into_iter()
                        .map(|l| LanguageDto {
                            name: l.name,
                            level: l.level,
                        })
                        .collect(),
                };
                Ok(Some(response))
            }
            None => Ok(None),
        }
    }
}
