use axum::{Json, extract::State};
use axum_extra::extract::Query;
use dictionary::{WordPair, filters};
use rand::seq::{IndexedRandom, IteratorRandom, SliceRandom};
use serde::{Deserialize, Serialize};
use server_derive::ToResponseError;
use std::collections::HashSet;
use thiserror::Error;
use utoipa::{IntoParams, OpenApi, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};

use super::model::GameQuestion;

use crate::{
    application::AppState,
    routes::{
        dto::{NLevelDto, PartOfSpeechCategoryDto},
        game::model::GameMode,
    },
};

#[derive(OpenApi)]
#[openapi(components(schemas(GameMode)))]
struct Api;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::with_openapi(Api::openapi()).routes(routes!(get_free_response))
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
struct GetGameQuery {
    mode: GameMode,
    levels: HashSet<NLevelDto>,
    pos: HashSet<PartOfSpeechCategoryDto>,
}

#[derive(Debug, Serialize, ToSchema)]
struct GetGameResponse {
    data: Vec<GameQuestion>,
}

#[derive(Debug, Error, ToResponseError)]
enum GetGameError {
    #[error("no pairs to create questions with provided filter(s)")]
    #[response(status = UNPROCESSABLE_ENTITY, log = false)]
    BadQueryCombination,

    #[error("failed to create questions, at least one failed")]
    #[response(status = INTERNAL_SERVER_ERROR, log = true)]
    QuestionConstruction(#[source] CreateQuestionError),
}

#[utoipa::path(
    get,
    path = "/",
    params(GetGameQuery),
    responses(
        (status = OK, body = inline(GetGameResponse)),
        (status = UNPROCESSABLE_ENTITY, body = str),
        (status = INTERNAL_SERVER_ERROR, body = str),
    )
)]
#[tracing::instrument(name = "Create free response game", skip(ctx))]
async fn get_free_response(
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
        .sample(&mut rand::rng(), 100);

    if pairs.is_empty() {
        return Err(GetGameError::BadQueryCombination);
    }

    let mut questions = create_questions(&pairs, query.mode).map_err(|e| match e {
        CreateQuestionsError::QuestionConstruction(e) => GetGameError::QuestionConstruction(e),
    })?;

    questions.shuffle(&mut rand::rng());

    Ok(Json(GetGameResponse { data: questions }))
}

#[derive(Debug, Clone, Error)]
enum CreateQuestionsError {
    #[error(transparent)]
    QuestionConstruction(#[from] CreateQuestionError),
}

fn create_questions(
    pairs: &[WordPair],
    mode: GameMode,
) -> Result<Vec<GameQuestion>, CreateQuestionsError> {
    pairs
        .iter()
        .map(|pair| {
            let prompt = extract_prompt(mode, pair)?;
            let answers = extract_answers(mode, pair)?;

            Ok(GameQuestion {
                prompt,
                answers,
                word_pair: pair.clone().into(),
            })
        })
        .collect()
}

#[derive(Debug, Clone, Error)]
enum CreateQuestionError {
    #[error("pair is missing sense when game mode requires it: {0:#?}")]
    MissingSense(WordPair),
    #[error("pair is missing glossary when game mode requires it: {0:#?}")]
    MissingGlossary(WordPair),
    #[error("pair is missing kanji when game mode requires it: {0:#?}")]
    MissingKanji(WordPair),
}

fn extract_prompt(mode: GameMode, pair: &WordPair) -> Result<String, CreateQuestionError> {
    use CreateQuestionError as Cr;
    use GameMode as GM;

    match mode {
        GM::EngToKana | GM::EngToKanji => {
            let gloss = extract_glossary(pair)?;
            if gloss.is_empty() {
                return Err(Cr::MissingGlossary(pair.clone()));
            }
            Ok(gloss.join("; "))
        }
        GM::KanaToKanji | GM::KanaToEng => Ok(pair.kana.clone()),
        GM::KanjiToKana | GM::KanjiToEng => pair
            .kanji
            .clone()
            .ok_or_else(|| Cr::MissingKanji(pair.clone())),
    }
}

fn extract_answers(mode: GameMode, pair: &WordPair) -> Result<Vec<String>, CreateQuestionError> {
    use CreateQuestionError as Cr;
    use GameMode as GM;

    match mode {
        GM::EngToKana | GM::KanjiToKana => Ok(vec![pair.kana.clone()]),
        GM::EngToKanji | GM::KanaToKanji => Ok(vec![
            pair.kanji
                .clone()
                .ok_or_else(|| Cr::MissingKanji(pair.clone()))?,
        ]),
        GM::KanaToEng | GM::KanjiToEng => {
            let gloss = extract_glossary(pair)?;
            if gloss.is_empty() {
                return Err(Cr::MissingGlossary(pair.clone()));
            }
            Ok(gloss)
        }
    }
}

fn extract_glossary(pair: &WordPair) -> Result<Vec<String>, CreateQuestionError> {
    use CreateQuestionError as Cr;

    let sense = pair
        .senses
        .choose(&mut rand::rng())
        .ok_or_else(|| Cr::MissingSense(pair.clone()))?;

    Ok(sense.glossary.clone())
}
