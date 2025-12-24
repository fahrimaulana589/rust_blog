use crate::app::features::profile::domain::entity::{
    NewProfile, NewProfileLanguage, NewProfileSpecialization, NewProfileTechFocus, Profile,
    ProfileLanguage, ProfileSpecialization, ProfileTechFocus,
};
use crate::app::features::profile::domain::repository::ProfileRepository;
use crate::schema::{profile_languages, profile_specializations, profile_tech_focus, profiles};
use crate::utils::db::DbPool;
// use anyhow::Result;
use diesel::prelude::*;

#[derive(Clone)]
pub struct ProfileRepositoryImpl {
    pub pool: DbPool,
}

impl ProfileRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl ProfileRepository for ProfileRepositoryImpl {
    fn get_profile(
        &self,
    ) -> QueryResult<
        Option<(
            Profile,
            Vec<ProfileSpecialization>,
            Vec<ProfileTechFocus>,
            Vec<ProfileLanguage>,
        )>,
    > {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");

        let profile = profiles::table.first::<Profile>(&mut conn).optional()?;

        match profile {
            Some(p) => {
                let specializations = profile_specializations::table
                    .filter(profile_specializations::profile_id.eq(p.id))
                    .load::<ProfileSpecialization>(&mut conn)?;
                let tech_focus = profile_tech_focus::table
                    .filter(profile_tech_focus::profile_id.eq(p.id))
                    .load::<ProfileTechFocus>(&mut conn)?;
                let languages = profile_languages::table
                    .filter(profile_languages::profile_id.eq(p.id))
                    .load::<ProfileLanguage>(&mut conn)?;
                Ok(Some((p, specializations, tech_focus, languages)))
            }
            None => Ok(None),
        }
    }

    fn upsert_profile(
        &self,
        profile_data: NewProfile,
        specializations: Vec<String>,
        tech_focus: Vec<String>,
        languages: Vec<(String, String)>,
    ) -> QueryResult<Profile> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            // Check if profile exists
            let existing_profile = profiles::table.first::<Profile>(conn).optional()?;

            let profile = match existing_profile {
                Some(p) => diesel::update(profiles::table.find(p.id))
                    .set(&profile_data)
                    .get_result::<Profile>(conn)?,
                None => diesel::insert_into(profiles::table)
                    .values(&profile_data)
                    .get_result::<Profile>(conn)?,
            };

            // Delete existing relations
            diesel::delete(
                profile_specializations::table
                    .filter(profile_specializations::profile_id.eq(profile.id)),
            )
            .execute(conn)?;
            diesel::delete(
                profile_tech_focus::table.filter(profile_tech_focus::profile_id.eq(profile.id)),
            )
            .execute(conn)?;
            diesel::delete(
                profile_languages::table.filter(profile_languages::profile_id.eq(profile.id)),
            )
            .execute(conn)?;

            // Insert new relations
            let new_specializations: Vec<NewProfileSpecialization> = specializations
                .into_iter()
                .map(|s| NewProfileSpecialization {
                    profile_id: profile.id,
                    specialization: s,
                })
                .collect();
            if !new_specializations.is_empty() {
                diesel::insert_into(profile_specializations::table)
                    .values(&new_specializations)
                    .execute(conn)?;
            }

            let new_tech_focus: Vec<NewProfileTechFocus> = tech_focus
                .into_iter()
                .map(|s| NewProfileTechFocus {
                    profile_id: profile.id,
                    tech_focus: s,
                })
                .collect();
            if !new_tech_focus.is_empty() {
                diesel::insert_into(profile_tech_focus::table)
                    .values(&new_tech_focus)
                    .execute(conn)?;
            }

            let new_languages: Vec<NewProfileLanguage> = languages
                .into_iter()
                .map(|(name, level)| NewProfileLanguage {
                    profile_id: profile.id,
                    name,
                    level,
                })
                .collect();
            if !new_languages.is_empty() {
                diesel::insert_into(profile_languages::table)
                    .values(&new_languages)
                    .execute(conn)?;
            }

            Ok(profile)
        })
    }
}
