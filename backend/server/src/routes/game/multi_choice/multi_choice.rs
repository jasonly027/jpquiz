use axum::{Json, extract::State};
use axum_extra::extract::Query;
use dictionary::{WordPair, filters};
use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};
use server_derive::ResponseError;
use std::collections::HashSet;
use thiserror::Error;
use utoipa::{IntoParams, OpenApi, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};

use super::model::{
    CreateQuestionError, CreateQuestionsError, GameMode, GameQuestion, create_questions,
};

use crate::{
    application::AppState,
    routes::dto::{NLevelDto, PartOfSpeechCategoryDto},
};

#[derive(OpenApi)]
#[openapi(components(schemas(GameMode)))]
struct Api;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::with_openapi(Api::openapi()).routes(routes!(get_game))
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
struct GetGameQuery {
    mode: GameMode,
    levels: HashSet<NLevelDto>,
    pos: HashSet<PartOfSpeechCategoryDto>,
}

#[derive(Debug, Error, ResponseError)]
enum GetGameError {
    #[error("not enough pairs to create questions with provided filter(s)")]
    #[response(status = NOT_FOUND, log = false)]
    BadQueryCombination,

    #[error("failed to create questions, at least one failed")]
    #[response(status = INTERNAL_SERVER_ERROR, log = true)]
    QuestionConstruction(#[source] CreateQuestionError),
}

#[derive(Debug, Serialize, ToSchema)]
struct GetGameResponse {
    data: Vec<GameQuestion>,
}

#[utoipa::path(
    get,
    path = "/",
    params(GetGameQuery),
    responses(
        (status = OK, body = inline(GetGameResponse)),
        (status = NOT_FOUND, description = "No questions matching filters", body = str),
    )
)]
#[tracing::instrument(name = "Create multiple choice game", skip(ctx))]
async fn get_game(
    State(ctx): State<AppState>,
    Query(query): Query<GetGameQuery>,
) -> Result<Json<GetGameResponse>, GetGameError> {
    let pairs: Vec<WordPair> = ctx
        .dictionary
        .pairs()
        .filter({
            // Require kanji if necessary
            use GameMode as GM;
            let f = match query.mode {
                GM::KanjiToEng | GM::KanjiToKana | GM::EngToKanji | GM::KanaToKanji => {
                    filters::word_pair::has_kanji
                }
                _ => filters::word_pair::passthrough,
            };
            move |p| f(p)
        })
        .filter({
            // Match NLevel(s)
            let levels = query.levels.into_iter().map(Into::into);
            let f = filters::word_pair::levels(levels);
            move |p| f(p)
        })
        .filter_map({
            // Match Part of Speech Category(s)
            let categories = query.pos.into_iter().map(Into::into);
            let f = filters::word_pair::categories(categories);
            move |p| f(p)
        })
        .sample(&mut rand::rng(), 50);

    let questions = create_questions(&pairs, query.mode).map_err(|e| match e {
        CreateQuestionsError::InsufficientPairs => GetGameError::BadQueryCombination,
        CreateQuestionsError::QuestionConstruction(e) => GetGameError::QuestionConstruction(e),
    })?;

    Ok(Json(GetGameResponse { data: questions }))
}
