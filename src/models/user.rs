use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct User {
    pub id: String, // Use Uuid for unique user identification
    pub username: String,
    pub password: String, // Hashed in production
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub enum Role {
    Admin,
    User,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub confirm_password: String,
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegisterResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
}
