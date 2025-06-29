use std::env;


use axum::extract::State;
use axum::{http::StatusCode, response::IntoResponse, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use utoipa::OpenApi;

use crate::middleware::auth::Claims;
use crate::models::user::RegisterRequest;
use crate::models::user::RegisterResponse;
use crate::models::{LoginRequest, LoginResponse, Role, User};
use crate::AppState;
use dotenv::dotenv;
use uuid::Uuid;

#[derive(OpenApi)]
#[openapi(
    paths(login, register),
    components(schemas(LoginRequest, LoginResponse, RegisterRequest, RegisterResponse))
)]
pub struct AuthApi;

// Add a Registration route with email, password, confirm password. Newly register users role should be User
// and password should be hashed before storing in the database.

#[utoipa::path(
    post,
    path = "/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully"),
        (status = 400, description = "Bad request")
    )
)]

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    if payload.email.is_empty()
        || payload.password.is_empty()
        || payload.first_name.is_empty()
        || payload.last_name.is_empty()
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid registration data"})),
        )
            .into_response();
    }
    let mut users = state.users.lock().unwrap();

    // If validation passes, hash the password and store the user.
    let hashed_password = bcrypt::hash(payload.password, 4).unwrap();
    let user = User {
        id: Uuid::new_v4().to_string(),
        first_name: payload.first_name,
        last_name: payload.last_name,
        email: payload.email,
        password: hashed_password,
        role: Role::User,
    };

    users.push(user.clone()); // Store the user in the state (in-memory for this example).
    (
        StatusCode::CREATED,
        Json(RegisterResponse {
            id: user.id.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            email: user.email.clone(),
        }),
    )
        .into_response()
}

#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials")
    )
)]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    dotenv().ok();

    let users = state.users.lock().unwrap();
    let user = users.iter().find(|u| u.email == payload.email);

    if user.is_none()
        || bcrypt::verify(payload.password, &user.unwrap().password).ok() != Some(true)
    {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid credentials"})),
        )
            .into_response();
    }

    let claims = Claims {
        sub: payload.email.clone(),
        role: user.unwrap().role.clone(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };
    let config = state.config.clone();
    let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(
                config.jwt_secret.as_bytes(),
            ),
        )
        .unwrap();

    return (StatusCode::OK, Json(LoginResponse { token })).into_response();

    //   (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid credentials"}))).into_response()
}
