use std::sync::Arc;

use anyhow::Result;
use axum::{Router, routing::get};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::{configuration::Settings, database, telemetry::RequestSpan};

pub struct Application {
    router: Router,
    listener: TcpListener,
    port: u16,
}

impl Application {
    pub async fn build(config: &Settings) -> Result<Self> {
        let listener = {
            let addr = format!("{}:{}", config.application.host, config.application.port);
            TcpListener::bind(addr).await?
        };

        let address = listener.local_addr()?;
        let port = address.port();

        info!("Serving at http://{}", address);

        let db_pool = database::create_pool(&config.database);
        let router = router(db_pool);

        Ok(Self {
            router,
            listener,
            port,
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        axum::serve(self.listener, self.router).await
    }
}

pub fn router(db_pool: PgPool) -> Router {
    let middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http().make_span_with(RequestSpan::default()));

    let shared_state = Arc::new(AppState { db_pool });

    let router = Router::new()
        .route("/", get(root))
        .layer(middleware)
        .with_state(shared_state);

    router
}

pub struct AppState {
    pub db_pool: PgPool,
}

async fn root() -> &'static str {
    "OK"
}
