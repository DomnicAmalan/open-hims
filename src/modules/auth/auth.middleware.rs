use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

use crate::modules::auth::{AuthService, AuthenticatedUser};

/// Authentication middleware
pub struct AuthMiddleware {
    auth_service: Arc<AuthService>,
}

impl AuthMiddleware {
    pub fn new(auth_service: Arc<AuthService>) -> Self {
        Self { auth_service }
    }

    /// Middleware function to validate authentication
    pub async fn authenticate(
        State(auth_service): State<Arc<AuthService>>,
        headers: HeaderMap,
        mut request: Request,
        next: Next,
    ) -> Result<Response, StatusCode> {
        // Extract Bearer token from Authorization header
        let token = headers
            .get("authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|auth_header| {
                if auth_header.starts_with("Bearer ") {
                    Some(&auth_header[7..])
                } else {
                    None
                }
            });

        if let Some(token) = token {
            match auth_service.validate_token(token).await {
                Ok(Some(user)) => {
                    // Add user to request extensions for use in handlers
                    request.extensions_mut().insert(user);
                    Ok(next.run(request).await)
                }
                Ok(None) => {
                    tracing::warn!("Invalid token provided");
                    Err(StatusCode::UNAUTHORIZED)
                }
                Err(e) => {
                    tracing::error!("Token validation error: {}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        } else {
            tracing::warn!("No authorization header provided");
            Err(StatusCode::UNAUTHORIZED)
        }
    }

    /// Middleware to check specific permissions
    pub async fn require_permission(
        permission: String,
    ) -> impl Fn(Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response, StatusCode>> + Send>>
    {
        move |request: Request, next: Next| {
            let perm = permission.clone();
            Box::pin(async move {
                // Get user from request extensions (set by authenticate middleware)
                if let Some(user) = request.extensions().get::<AuthenticatedUser>() {
                    if user.permissions.contains(&perm) {
                        Ok(next.run(request).await)
                    } else {
                        tracing::warn!(
                            "User {} does not have permission: {}",
                            user.username,
                            perm
                        );
                        Err(StatusCode::FORBIDDEN)
                    }
                } else {
                    tracing::error!("No authenticated user found in request");
                    Err(StatusCode::UNAUTHORIZED)
                }
            })
        }
    }
}