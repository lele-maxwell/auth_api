use axum::{
    body::Body, http::{Request, StatusCode}, middleware::Next, response::{IntoResponse, Response}
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::{env, sync::Arc};
use crate::models::{Role, User};
use dotenv::dotenv;

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub role: Role,
    pub exp: usize,
}

pub async fn auth_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

        dotenv().ok(); 
        
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let key = DecodingKey::from_secret(
        env::var("JWT_SECRET").expect("JWT_SECRET must be set").as_ref(), );


    let token_data = decode::<Claims>(token, &key, &Validation::default())
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user = User {
        id: Uuid::new_v4().to_string(), // In production, fetch from DB
        username: token_data.claims.sub,
        password: String::new(),
        role: token_data.claims.role,
    };

    let mut request = req;
    request.extensions_mut().insert(Arc::new(user));
    Ok(next.run(request).await)
}
