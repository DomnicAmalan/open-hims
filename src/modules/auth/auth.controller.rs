use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::modules::auth::{AuthService, AuthenticatedUser};

/// Authentication controller
pub struct AuthController {
    auth_service: Arc<AuthService>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user: AuthenticatedUser,
    pub token: String,
    pub expires_in: u64,
}

#[derive(Debug, Deserialize)]
pub struct TokenValidationRequest {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct TokenValidationResponse {
    pub valid: bool,
    pub user: Option<AuthenticatedUser>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

impl AuthController {
    /// Create new controller with injected service
    pub fn new(auth_service: Arc<AuthService>) -> Self {
        Self { auth_service }
    }

    /// Create router with dependency injection
    pub fn routes(&self) -> Router {
        Router::new()
            .route("/login", post(Self::login))
            .route("/validate", post(Self::validate_token))
            .with_state(self.auth_service.clone())
    }

    /// User login endpoint
    pub async fn login(
        State(auth_service): State<Arc<AuthService>>,
        Json(payload): Json<LoginRequest>,
    ) -> Result<Json<LoginResponse>, (StatusCode, Json<ErrorResponse>)> {
        tracing::info!("Login attempt for user: {}", payload.username);
        
        match auth_service
            .authenticate(&payload.username, &payload.password)
            .await
        {
            Ok(Some(user)) => {
                match auth_service.generate_token(&user).await {
                    Ok(token) => {
                        tracing::info!("User {} logged in successfully", user.username);
                        Ok(Json(LoginResponse {
                            user,
                            token,
                            expires_in: 3600, // 1 hour
                        }))
                    }
                    Err(e) => {
                        tracing::error!("Failed to generate token: {}", e);
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(ErrorResponse {
                                error: "Token generation failed".to_string(),
                                message: e.to_string(),
                            }),
                        ))
                    }
                }
            }
            Ok(None) => {
                tracing::warn!("Invalid login attempt for user: {}", payload.username);
                Err((
                    StatusCode::UNAUTHORIZED,
                    Json(ErrorResponse {
                        error: "Invalid credentials".to_string(),
                        message: "Username or password is incorrect".to_string(),
                    }),
                ))
            }
            Err(e) => {
                tracing::error!("Authentication error for user {}: {}", payload.username, e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "Authentication failed".to_string(),
                        message: e.to_string(),
                    }),
                ))
            }
        }
    }

    /// Token validation endpoint
    pub async fn validate_token(
        State(auth_service): State<Arc<AuthService>>,
        Json(payload): Json<TokenValidationRequest>,
    ) -> Result<Json<TokenValidationResponse>, (StatusCode, Json<ErrorResponse>)> {
        tracing::info!("Token validation request");
        
        match auth_service.validate_token(&payload.token).await {
            Ok(Some(user)) => {
                tracing::info!("Token validation successful for user: {}", user.username);
                Ok(Json(TokenValidationResponse {
                    valid: true,
                    user: Some(user),
                }))
            }
            Ok(None) => {
                tracing::warn!("Invalid token provided");
                Ok(Json(TokenValidationResponse {
                    valid: false,
                    user: None,
                }))
            }
            Err(e) => {
                tracing::error!("Token validation error: {}", e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "Token validation failed".to_string(),
                        message: e.to_string(),
                    }),
                ))
            }
        }
    }
}