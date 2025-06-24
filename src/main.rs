use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Mutex;
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod utils;
use std::fs;

use crate::{
    middleware::auth::auth_middleware,
    routes::{auth, protected},
    utils::load_env,
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Arc<utils::Config>,
    pub users: Arc<Mutex<Vec<models::User>>>,
}

#[derive(OpenApi)]
#[openapi(
    info(title = "Auth API", description = "A simple auth API", license(name = "MIT", identifier = "MIT")),
    paths(
        auth::register,
        auth::login,
        protected::admin_route,
        protected::user_route
    ),
    components(schemas(
        models::User,
        models::Role,
        models::LoginRequest,
        models::LoginResponse,
        models::RegisterRequest,
        models::RegisterResponse
    ))
)]

struct ApiDoc;
// Write OpenAPI JSON to a file
fn write_openapi_json() {
    let openapi = ApiDoc::openapi();
    let json = openapi.to_json().unwrap();
    fs::write("openapi.json", json).expect("Failed to write openapi.json");
}

#[tokio::main]
async fn main() {
    write_openapi_json();

    let state = AppState {
        config: Arc::new(load_env()),
        users: Arc::new(Mutex::new(vec![])),
    };

    let app = Router::new()
        .route("/admin", get(protected::admin_route))
        .route("/user", get(protected::user_route))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
