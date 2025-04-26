use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use chrono::Utc;
use entities::user::URol;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tracing::error;
use use_cases::user_service::LogInResponse;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: Uuid, // User id,
    exp: usize,
    iat: usize,
    user_rol: URol,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfoAuth {
    pub user_id: Uuid, // User id,
    pub user_rol: URol,
}

pub fn generate_jwt(
    log_in_response: &LogInResponse,
    token_key: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now().timestamp() as usize;

    let my_claims = Claims {
        sub: log_in_response.user_id,
        exp: now + 3600,
        iat: now,
        user_rol: log_in_response.user_rol.clone(),
    };

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(token_key.as_ref()),
    );

    token
}

fn decode_jwt(token_key: &str, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let my_claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(token_key.as_ref()),
        &Validation::default(),
    )?;

    Ok(my_claims.claims)
}

pub async fn auth_middleware(
    State(jwt_secret): State<String>,
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    let jwt_token_string = match auth_header {
        Some(header_value) if header_value.starts_with("Bearer ") => {
            &header_value[7..] // Remove "Bearer " prefix
        }
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    let claims = decode_jwt(&jwt_secret, jwt_token_string).map_err(|err| {
        error!("Error in token verification: {err}");
        StatusCode::UNAUTHORIZED
    })?;

    request.extensions_mut().insert(UserInfoAuth {
        user_id: claims.sub,
        user_rol: claims.user_rol,
    });

    let response = next.run(request).await;

    Ok(response)
}
