use super::models::*;
use crate::api::projects::models::ProjectId;
use axum::{
    extract::{self, Path, Query},
    http::StatusCode,
    response::IntoResponse,
};

#[utoipa::path(
    get,
    path = "/artemis/requirements",
    params(
        ("projectId" = ProjectId, Query, description = "The unique identifier of the project to retrieve all requirements for."),
    ),
    responses(
        (status = 200, description = "Successfully listed requirements."),
    )
)]
pub async fn list_requirements(Query(project_id): Query<ProjectId>) -> impl IntoResponse {
    // TODO
    println!("Listing requirements for projectId {:?}", project_id);
    StatusCode::INTERNAL_SERVER_ERROR
}

#[utoipa::path(
    post,
    path = "/artemis/requirements",
    request_body = CreateRequirementDetails,
    responses(
        (status = 201, description = "Successfully created new requirement."),
    )
)]
pub async fn post_requirement(
    extract::Json(create_details): extract::Json<CreateRequirementDetails>,
) -> impl IntoResponse {
    // TODO
    println!("Received create_details: {:?}", create_details);
    StatusCode::NOT_IMPLEMENTED
}

#[utoipa::path(
    get,
    path = "/artemis/requirements/{id}",
    params(
        ("id" = RequirementId, description = "RequirementId to retrieve details for."),
    ),
    responses(
        (status = 200, description = "Successfully retrieved requirement."),
        (status = 404, description = "Requirement not found."),
    )
)]
pub async fn get_requirement(Path(id): Path<RequirementId>) -> impl IntoResponse {
    let requirement = Requirement {
        id: id,
        project_id: ProjectId(0),
        description: String::from("test description"),
    };
    axum::Json(requirement)
}

#[utoipa::path(
    delete,
    path = "/artemis/requirements/{id}",
    params(
        ("id" = RequirementId, description = "RequirementId to delete."),
    ),
    responses(
        (status = 204, description = "Requirement was successfully deleted."),
        (status = 404, description = "Requirement not found."),
        (status = 409, description = "Unable to delete requirment due to conflict."),
    )
)]
pub async fn delete_requirement(Path(id): Path<RequirementId>) -> impl IntoResponse {
    println!("Deleting requirement {:?}", id);
    StatusCode::NOT_IMPLEMENTED
}
