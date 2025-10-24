use axum::Router;
use axum::routing::get;

pub mod openapi;
mod projects;
mod requirements;

pub fn create_app() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route(
            "/artemis/projects",
            get(projects::handlers::list_projects).post(projects::handlers::post_project),
        )
        .route(
            "/artemis/projects/{id}",
            get(projects::handlers::get_project).delete(projects::handlers::delete_project),
        )
        .route(
            "/artemis/requirements",
            get(requirements::handlers::list_requirements)
                .post(requirements::handlers::post_requirement),
        )
        .route(
            "/artemis/requirements/{id}",
            get(requirements::handlers::get_requirement)
                .delete(requirements::handlers::delete_requirement),
        )
}
