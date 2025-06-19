use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use utoipa::OpenApi;

use crate::models::{LoginRequest, LoginResponse, Role, User};
use crate::middleware::auth::Claims;

#[derive(OpenApi)]
#[openapi(
    paths(login),
    components(schemas(LoginRequest, LoginResponse))
)]
pub struct AuthApi;

#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials")
    )
)]
pub async fn login(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    // In production, verify against a database
    if payload.username == "admin" && payload.password == "password" {
        let claims = Claims {
            sub: payload.username.clone(),
            role: Role::Admin,
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("your-secret-key".as_ref()),
        )
        .unwrap();

        return (
            StatusCode::OK,
            Json(LoginResponse { token }),
        ).into_response();
    }

    (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid credentials"}))).into_response()
}
