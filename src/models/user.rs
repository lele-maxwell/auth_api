use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct User {
    pub id: String, // Use Uuid for unique user identification
    pub first_name: String,
    pub email: String,
    pub password: String, // Hashed in production
    pub last_name: String,
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub enum Role {
    Admin,
    User,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub first_name: String,
    pub last_name: String, 
    pub email: String,
    pub password: String,
    
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegisterResponse {
    pub id: String, // Use Uuid for unique user identification
    pub first_name: String,
    pub last_name: String, 
    pub email: String,

}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
}
