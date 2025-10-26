use axum::{Extension, Router};
use axum::routing;
use axum::serve;

use std::net::SocketAddr;
use dotenv::dotenv;
use tower_http::cors::CorsLayer;
use tracing::info;
use std::sync::Arc;
use sqlx::Any;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt::Subscriber::builder().with_max_level(tracing::Level::TRACE).finish();
    tracing::subscriber::set_global_default(subscriber).expect("Setting global subscriber failed");
    dotenv().ok();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let cfg= config::Config::from_env().map_err(|e| tracing::error!("Cannot loading the Config: {:?}",e.to_string())).unwrap();
    let db = PgPoolOptions::new().max_connections(cfg.postgres.conn).connect(&cfg.postgres.dsn).await.unwrap();

    let app = Router::new()
        .route("/", routing::get(|| async { "Hello, world!" }))
        .layer(Extension(CorsLayer::new().allow_headers(Any).allow_methods(Any).allow_origin(Any)))
        .layer(Extension(Arc::new(())))
        .layer(Extension(Arc::new(db)));

    info!("Database is Connected");
    info!("🚀 Server running at http://{}", addr);

    serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
