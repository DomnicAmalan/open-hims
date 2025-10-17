use anyhow::Result;
use axum::{http::StatusCode, response::Json, routing::get, Router};
use serde::Serialize;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use hims_core_sdk::{
    database::connection::Database,
    modules::AppModules,
};

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    timestamp: String,
}

/// Health check endpoint
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

/// Create the main application router with all modules
async fn create_app() -> Result<Router> {
    // Initialize database connection
    let database = Database::new().await?;
    let db_pool = database.get_pool();
    
    // Initialize all application modules
    let app_modules = Arc::new(AppModules::new(db_pool));
    
    // Create the main router
    let app = Router::new()
        .route("/health", get(health))
        .merge(app_modules.routes())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()),
        );

    Ok(app)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    tracing::info!("🏥 HIMS Core API Server starting...");

    // Create the application
    let app = create_app().await?;

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("🚀 Server listening on http://0.0.0.0:3000");

    axum::serve(listener, app).await?;

    Ok(())
}