mod error;

use std::time::Duration;

use axum::{http::StatusCode, routing::get, Extension, Router};
use sqlx::PgPool;
use tokio::{net::TcpListener, signal};
use tower::ServiceBuilder;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::PAPERCLIP;

pub type Database = Extension<PgPool>;

pub async fn serve(pool: PgPool) -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "paperclip=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().compact().without_time())
        .init();

    let app = Router::new().merge(api_routes()).layer(
        ServiceBuilder::new()
            .layer((
                TraceLayer::new_for_http(),
                TimeoutLayer::new(Duration::from_secs(10)),
            ))
            .layer(Extension(pool)),
    );

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    tracing::info!(
        "{} listening on {}",
        PAPERCLIP,
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

async fn root() -> StatusCode {
    StatusCode::OK
}

fn api_routes() -> Router {
    Router::new().route("/", get(root))
}
