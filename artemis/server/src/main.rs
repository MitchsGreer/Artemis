use axum::{Json, Router, routing::get};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use database::envs;
use utoipa::ToSchema;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // build app and add single route
    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/projects", get(list_projects).post(post_project))
        .route("/projects/{id}", get(get_project).delete(delete_project));

    let address = envs::db_address();
    let port = envs::db_port();
    println!("Now Listening on {address}:{port}...");
    let listener = tokio::net::TcpListener::bind(format!("{address}:{port}")).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(ToSchema, Serialize, Deserialize, Debug)]
struct UserId(u64);

#[derive(ToSchema, Serialize)]
struct ProjectDetails {
    id: UserId,
    title: String,
    status: ProjectStatus
}

#[derive(ToSchema, Serialize)]
#[allow(dead_code)]
enum ProjectStatus {
    Todo,
    InProgress,
    InValidation,
    Complete
}

#[utoipa::path(
    get,
    path = "/projects",
    responses(
        (status = 200, description = "Successfully listed projects.", body = Vec<ProjectDetails>),
    ),
)]
async fn list_projects() -> impl IntoResponse {
    // TODO
    let projects: Vec<ProjectDetails> = vec![];
    Json(projects)
}

#[utoipa::path(
    post,
    path = "/projects",
    responses(
        (status = 201, description = "Successfully created new project."),
    ),
)]
async fn post_project() -> StatusCode {
    // TODO
    StatusCode::CREATED
}

#[utoipa::path(
    get,
    path = "/projects/{id}",
    params(
        ("id" = UserId, Path, description = "The unique identifier of the project to retrieve."),
    ),
    responses(
        (status = 200, description = "Project was successfully found.", body = ProjectDetails),
        (status = 404, description = "Project not found."),
    ),
)]
async fn get_project(Path(id): Path<UserId>) -> impl IntoResponse {
    // TODO
    let project = ProjectDetails{
        id: id,
        title: String::from("test"),
        status: ProjectStatus::Todo,
    };
    Json(project)
}

#[utoipa::path(
    delete,
    path = "/projects/{id}",
    params(
        ("id" = UserId, Path, description = "The unique identifier of the project to retrieve."),
    ),
    responses(
        (status = 204, description = "Project was successfully deleted."),
        (status = 404, description = "Project not found."),
    ),
)]
async fn delete_project(Path(id): Path<UserId>) -> impl IntoResponse {
    // TODO
    println!("Deleting id:{:?}", id);
    StatusCode::NO_CONTENT
}
