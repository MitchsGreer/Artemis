use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::api::projects::models::ProjectId;

#[derive(ToSchema, Serialize, Deserialize, Debug)]
pub struct CreateRequirementDetails {
    project_id: ProjectId,
    description: String,
}

#[derive(ToSchema, Serialize)]
pub struct Requirement {
    pub id: RequirementId,
    pub project_id: ProjectId,
    pub description: String,
}

#[derive(ToSchema, Serialize, Deserialize, Debug)]
pub struct RequirementId(pub u64);
