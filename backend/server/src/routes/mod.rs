use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::application::AppState;

mod dto;
mod game;
mod health_check;

#[derive(OpenApi)]
#[openapi(components(schemas(dto::NLevelDto, dto::PartOfSpeechCategoryDto)))]
struct Api;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::with_openapi(Api::openapi())
        .routes(routes!(health_check::health_check))
        .nest("/game/multi-choice", game::multi_choice::router())
        .nest("/game/free-response", game::free_response::router())
}
