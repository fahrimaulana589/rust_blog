use crate::schema::portfolios;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable, AsChangeset,
)]
#[diesel(table_name = portfolios)]
pub struct Portfolio {
    pub id: i32,
    pub project_id: i32,
    pub judul: String,
    pub deskripsi: Option<String>,
    pub is_active: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = portfolios)]
pub struct NewPortfolio {
    pub project_id: i32,
    pub judul: String,
    pub deskripsi: Option<String>,
    pub is_active: bool,
}
