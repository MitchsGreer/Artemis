use super::models::*;
use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;

#[utoipa::path(
    get,
    path = "/artemis/projects",
    responses(
        (status = 200, description = "Successfully listed projects.", body = Vec<ProjectDetails>),
    ),
)]
pub async fn list_projects() -> impl IntoResponse {
    // TODO
    let projects: Vec<ProjectDetails> = vec![];
    Json(projects)
}

#[utoipa::path(
    post,
    path = "/artemis/projects",
    responses(
        (status = 201, description = "Successfully created new project."),
    ),
)]
pub async fn post_project() -> StatusCode {
    // TODO
    StatusCode::CREATED
}

#[utoipa::path(
    get,
    path = "/artemis/projects/{id}",
    params(
        ("id" = ProjectId, Path, description = "The unique identifier of the project to retrieve."),
    ),
    responses(
        (status = 200, description = "Project was successfully found.", body = ProjectDetails),
        (status = 404, description = "Project not found."),
    ),
)]
pub async fn get_project(Path(id): Path<ProjectId>) -> impl IntoResponse {
    // TODO
    let project = ProjectDetails {
        id: id,
        title: String::from("test"),
        status: ProjectStatus::Todo,
    };
    Json(project)
}

#[utoipa::path(
    delete,
    path = "/artemis/projects/{id}",
    params(
        ("id" = ProjectId, Path, description = "The unique identifier of the project to delete."),
    ),
    responses(
        (status = 204, description = "Project was successfully deleted."),
        (status = 404, description = "Project not found."),
        (status = 409, description = "Unable to delete project due to conflict."),
    ),
)]
pub async fn delete_project(Path(id): Path<ProjectId>) -> impl IntoResponse {
    // TODO
    println!("Deleting id:{:?}", id);
    StatusCode::NO_CONTENT
}
