use super::projects::handlers::*;
use super::projects::models::*;
use super::requirements::handlers::*;
use super::requirements::models::*;
use utoipa::OpenApi;

const OPEN_API_SPEC_PATH: &str = "../../open_api_spec.yaml";

pub fn update_api_spec() {
    let spec_yaml = ApiDoc::openapi()
        .to_yaml()
        .expect("Failed to convert server code to yaml spec.");
    std::fs::write(OPEN_API_SPEC_PATH, spec_yaml).expect("Failed to save open api spec.");
    println!("Updated OpenApi spec at {OPEN_API_SPEC_PATH}");
}

#[derive(OpenApi)]
#[openapi(
    paths(
        list_projects,
        post_project,
        get_project,
        delete_project,
        list_requirements,
        post_requirement,
        get_requirement,
        delete_requirement,
    ),
    components(schemas(
        ProjectId,
        ProjectDetails,
        ProjectStatus,
        RequirementId,
        Requirement,
        CreateRequirementDetails,
    )),
    info(
        title = "Artemis Server API",
        version = "1.0.0",
        description = "Server-side Project and Requirements API",
        license(name = "MIT", url = "https://opensource.org/licenses/MIT"),
    )
)]
struct ApiDoc;
