use axum::{Json, Router, routing::get};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use database::envs;
use utoipa::{OpenApi, ToSchema};
use serde::{Deserialize, Serialize};

const OPEN_API_SPEC_PATH: &str = "../../open_api_spec.yaml";

#[tokio::main]
async fn main() {
    save_spec_updates();

    // build app and add single route
    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/artemis/projects", get(list_projects).post(post_project))
        .route("/artemis/projects/{id}", get(get_project).delete(delete_project));

    let address = envs::db_address();
    let port = envs::db_port();
    println!("Now Listening on {address}:{port}...");
    let listener = tokio::net::TcpListener::bind(format!("{address}:{port}")).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub fn save_spec_updates() {
    let spec_yaml = ApiDoc::openapi()
        .to_yaml()
        .expect("Failed to convert server code to yaml spec.");
    std::fs::write(OPEN_API_SPEC_PATH, spec_yaml)
        .expect("Failed to save open api spec.");
    println!("Updated OpenApi spec at {OPEN_API_SPEC_PATH}");
}

#[derive(OpenApi)]
#[openapi(
    paths(list_projects, post_project, get_project, delete_project),
    components(
        schemas(ProjectId, ProjectDetails, ProjectStatus),
    ),
    info(
        title = "Artemis Server API",
        version = "1.0.0",
        description = "Server-side Project and Requirements API"
    ),
)]
struct ApiDoc;

#[derive(ToSchema, Serialize, Deserialize, Debug)]
struct ProjectId(u64);

#[derive(ToSchema, Serialize)]
struct ProjectDetails {
    id: ProjectId,
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
    path = "/artemis/projects",
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
    path = "/artemis/projects",
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
    path = "/artemis/projects/{id}",
    params(
        ("id" = ProjectId, Path, description = "The unique identifier of the project to retrieve."),
    ),
    responses(
        (status = 200, description = "Project was successfully found.", body = ProjectDetails),
        (status = 404, description = "Project not found."),
    ),
)]
async fn get_project(Path(id): Path<ProjectId>) -> impl IntoResponse {
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
async fn delete_project(Path(id): Path<ProjectId>) -> impl IntoResponse {
    // TODO
    println!("Deleting id:{:?}", id);
    StatusCode::NO_CONTENT
}
