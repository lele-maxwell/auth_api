use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use tower_http::cors::CorsLayer;
use std::sync::Mutex;
pub mod models;
pub mod routes;
pub mod middleware;

use crate::{routes::{auth, protected}, middleware::auth::auth_middleware};


#[derive(Debug, Clone)]
pub struct AppState {
    pub users: Arc<Mutex<Vec<models::User>>>, 
}
#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
        info(title = "Auth API", description = "A simple auth API"),
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
    let state = AppState {users: Arc::new(Mutex::new(vec![]))};

    let app = Router::new()
        .route("/admin", get(protected::admin_route))
        .route("/user", get(protected::user_route))

        .layer(axum::middleware::from_fn(auth_middleware)) 
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
