use axum::Router;
use axum::routing::get;

mod projects;
pub mod openapi;

pub fn create_app() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/artemis/projects", get(projects::handlers::list_projects).post(projects::handlers::post_project))
        .route("/artemis/projects/{id}", get(projects::handlers::get_project).delete(projects::handlers::delete_project))
}