use axum::{
    body::Body, extract::State, http::{Request, StatusCode}, middleware::Next, response::{IntoResponse, Response}
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::{env, sync::Arc};
use crate::{models::{Role, User}, AppState};
use dotenv::dotenv;

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub role: Role,
    pub exp: usize,
}

pub async fn auth_middleware(
    State(state): State<AppState>, 
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

        dotenv().ok(); 

        let config = state.config.clone();
        
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let key = DecodingKey::from_secret(
        config.jwt_secret.as_bytes()
    );


    let token_data = decode::<Claims>(token, &key, &Validation::default())
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user = User {
        id: Uuid::new_v4().to_string(), // In production, fetch from DB
        email: token_data.claims.sub,
        first_name: String::new(), 
        last_name: String::new(),  
        password: String::new(),
        role: token_data.claims.role,
    };

    let mut request = req;
    request.extensions_mut().insert(Arc::new(user));
    Ok(next.run(request).await)
}
