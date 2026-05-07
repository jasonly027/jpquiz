use std::{io::Cursor, sync::Arc};

use anyhow::Result;
use axum::{Router, middleware};
use dictionary::Dictionary;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::{Level, info};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::{
    configuration::{APP_NAME, Settings},
    database, routes,
    telemetry::{self, RequestSpan},
    util,
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

        let state = {
            let db_pool = database::create_pool(&config.database);

            let dict = include_str!("../../static/dictionary.json");
            let dictionary = Arc::new(Dictionary::load(Cursor::new(dict))?);

            Arc::new(AppStateInternal {
                db_pool,
                dictionary,
            })
        };

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
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(RequestSpan::new().level(Level::INFO))
                // BunyanFormattingLayer already logs on span enter/exit
                .on_request(())
                .on_response(())
                .on_failure(()),
        )
        .layer(middleware::from_fn(telemetry::log_server_failures))
        .layer(middleware::map_response(util::obfuscate_client_failures));

    let router = OpenApiRouter::with_openapi(Api::openapi())
        .merge(routes::router())
        .layer(middleware);

    router
}

pub type AppState = Arc<AppStateInternal>;

pub struct AppStateInternal {
    pub db_pool: PgPool,
    pub dictionary: Arc<Dictionary>,
}
