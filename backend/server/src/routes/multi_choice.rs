use axum::extract::Query;
use serde::Deserialize;
use utoipa::IntoParams;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::application::AppState;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(test))
}

#[derive(Deserialize, IntoParams)]
struct TestQuery {
    value: String,
}

#[utoipa::path(get, path = "/", params(TestQuery))]
async fn test(query: Query<TestQuery>) -> String {
    query.0.value
}
