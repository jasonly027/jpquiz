use std::{fmt::Display, sync::Arc};

use axum::{extract::Request, middleware::Next, response::Response};
use tower_http::trace::MakeSpan;
use tracing::{Level, error, subscriber::set_global_default};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};
use uuid::Uuid;

use crate::configuration::APP_NAME;

pub fn init(env_filter_var: &str) {
    let subscriber = Registry::default()
        .with(EnvFilter::from_env(env_filter_var))
        .with(JsonStorageLayer)
        .with(
            BunyanFormattingLayer::new(APP_NAME.to_string(), std::io::stdout)
                .skip_fields(["target", "line", "file"].into_iter())
                .expect("Failed to set formatting layer"),
        );

    set_global_default(subscriber).expect("Failed to set subscriber");
}

#[derive(Debug, Clone)]
pub struct RequestSpan {
    level: Level,
}

impl RequestSpan {
    pub fn new() -> Self {
        Self {
            level: DEFAULT_MESSAGE_LEVEL,
        }
    }

    pub fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }
}

const DEFAULT_MESSAGE_LEVEL: Level = Level::DEBUG;

impl Default for RequestSpan {
    fn default() -> Self {
        Self::new()
    }
}

impl<B> MakeSpan<B> for RequestSpan {
    fn make_span(&mut self, request: &axum::http::Request<B>) -> tracing::Span {
        // Copied from tower-http::trace::DefaultMakeSpan

        macro_rules! make_span {
            ($level:expr) => {
                    tracing::span!(
                        $level,
                        "request",
                        method = %request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                        requestId = %RequestId::new()
                    )
            }
        }

        match self.level {
            Level::ERROR => make_span!(Level::ERROR),
            Level::WARN => make_span!(Level::WARN),
            Level::INFO => make_span!(Level::INFO),
            Level::DEBUG => make_span!(Level::DEBUG),
            Level::TRACE => make_span!(Level::TRACE),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RequestId(pub Uuid);

impl RequestId {
    pub fn new() -> Self {
        RequestId(Uuid::now_v7())
    }
}

impl Display for RequestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.hyphenated().encode_lower(&mut Uuid::encode_buffer())
        )
    }
}

pub async fn log_server_failures(request: Request, next: Next) -> Response {
    let response = next.run(request).await;
    if let Some(err) = response.extensions().get::<Arc<anyhow::Error>>() {
        error!(?err, "unexpected error occured in handler");
    }
    response
}
