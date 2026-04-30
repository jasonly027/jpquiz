use std::sync::Arc;

use anyhow::Result;
use axum::Router;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::{
    configuration::{APP_NAME, Settings},
    database, routes,
    telemetry::RequestSpan,
};

pub struct Application {
    state: AppState,
    router: Router<Arc<AppStateInternal>>,
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

        let (router, _) = router().split_for_parts();

        let state = Arc::new(AppStateInternal {
            db_pool: database::create_pool(&config.database),
        });

        Ok(Self {
            state,
            router,
            listener,
            port,
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        axum::serve(self.listener, self.router.with_state(self.state)).await
    }
}

#[derive(OpenApi)]
#[openapi(info(title = APP_NAME))]
struct Api;

pub fn router() -> OpenApiRouter<AppState> {
    let middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http().make_span_with(RequestSpan::default()));

    let router = OpenApiRouter::with_openapi(Api::openapi())
        .routes(utoipa_axum::routes!(routes::health_check))
        .nest("/game/multi_choice", routes::multi_choice::router())
        .layer(middleware);

    router
}

pub type AppState = Arc<AppStateInternal>;

pub struct AppStateInternal {
    pub db_pool: PgPool,
}
