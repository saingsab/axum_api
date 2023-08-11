use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use sqlx::{postgres::PgPoolOptions, pool};

#[tokio::main]
async fn main() {
    let durl = std::env::var("DATABASE_URL").expect("set DATABASE_URL env vairable");

    // initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "axum_api=debug".into()),
        ))
    .with(tracing_subscriber::fmt::layer())
    .init();

    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/", get(|| async {"Hi, ayoung"}))
        .layer(cors)
        .layer(Extension(durl));

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}