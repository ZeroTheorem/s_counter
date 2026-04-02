mod database;
mod handlers;
mod parser;
mod query_params;
mod requests_bodies;
mod responses;
mod utc;

use axum::{
    Router,
    routing::{delete, get, post},
};
use dotenv::dotenv;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    database::Database,
    handlers::{create_record_handler, delete_record_handler, get_entries, get_stats_handler},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("warn"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let http_log = TraceLayer::new_for_http();
    let storage = Database::builder().await?;

    let router = Router::new()
        .route("/api/stats", get(get_stats_handler))
        .route("/api/entries", post(create_record_handler))
        .route("/api/entries/{record_id}", delete(delete_record_handler))
        .route("/api/entries", get(get_entries))
        .with_state(storage)
        .layer(http_log)
        .fallback_service(ServeDir::new("dist"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await?;
    axum::serve(listener, router).await?;
    Ok(())
}
