use crate::schema::{project_stack, projects, stacks};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// --- Project ---
#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = projects)]
pub struct Project {
    pub id: i32,
    pub nama_projek: String,
    pub deskripsi: String,
    pub status: String,
    pub progress: i32,
    pub link_demo: Option<String>,
    pub repository: Option<String>,
    pub tanggal_mulai: chrono::NaiveDate,
    pub tanggal_selesai: Option<chrono::NaiveDate>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize, Serialize, Debug, Clone)]
#[diesel(table_name = projects)]
pub struct NewProject {
    pub nama_projek: String,
    pub deskripsi: String,
    pub status: String,
    pub progress: i32,
    pub link_demo: Option<String>,
    pub repository: Option<String>,
    pub tanggal_mulai: chrono::NaiveDate,
    pub tanggal_selesai: Option<chrono::NaiveDate>,
}

// --- Stack ---
#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = stacks)]
pub struct Stack {
    pub id: i32,
    pub nama_stack: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize, Serialize, Debug, Clone)]
#[diesel(table_name = stacks)]
pub struct NewStack {
    pub nama_stack: String,
}

#[derive(Insertable, Deserialize, Serialize, Debug, Clone)]
#[diesel(table_name = project_stack)]
pub struct NewProjectStack {
    pub project_id: i32,
    pub stack_id: i32,
}
