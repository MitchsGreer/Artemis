use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Serialize, Deserialize, Debug)]
pub struct ProjectId(pub u64);

#[derive(ToSchema, Serialize)]
pub struct ProjectDetails {
    pub id: ProjectId,
    pub title: String,
    pub status: ProjectStatus,
}

#[derive(ToSchema, Serialize)]
#[allow(dead_code)]
pub enum ProjectStatus {
    Todo,
    InProgress,
    InValidation,
    Complete,
}
