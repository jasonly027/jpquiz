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

use super::model::{GameMode, GameQuestion};

use crate::{
    application::AppState,
    routes::{
        dto::{NLevelDto, PartOfSpeechCategoryDto},
        game::multi_choice::model::MULTI_CHOICE_GAME_QUESTION_CHOICES,
    },
};

#[derive(OpenApi)]
#[openapi(components(schemas(GameMode)))]
struct Api;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::with_openapi(Api::openapi()).routes(routes!(get_multi_choice))
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
    #[error("not enough pairs to create questions with provided filter(s)")]
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
#[tracing::instrument(name = "Create multiple choice game", skip(ctx))]
async fn get_multi_choice(
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

    let mut questions = create_questions(&pairs, query.mode).map_err(|e| match e {
        CreateQuestionsError::InsufficientPairs => GetGameError::BadQueryCombination,
        CreateQuestionsError::QuestionConstruction(e) => GetGameError::QuestionConstruction(e),
    })?;

    questions.shuffle(&mut rand::rng());

    Ok(Json(GetGameResponse { data: questions }))
}

#[derive(Debug, Clone, Error)]
enum CreateQuestionsError {
    #[error(
        "not enough pairs to create questions. requires at least {MULTI_CHOICE_GAME_QUESTION_CHOICES}"
    )]
    InsufficientPairs,
    #[error(transparent)]
    QuestionConstruction(#[from] CreateQuestionError),
}

fn create_questions(
    pairs: &[WordPair],
    mode: GameMode,
) -> Result<Vec<GameQuestion>, CreateQuestionsError> {
    if pairs.len() < MULTI_CHOICE_GAME_QUESTION_CHOICES {
        return Err(CreateQuestionsError::InsufficientPairs);
    }

    let rng = &mut rand::rng();

    pairs
        .iter()
        .map(|pair| {
            let prompt = extract_prompt(mode, pair)?;
            let answer = extract_answer(mode, pair)?;

            let other_pairs = pairs
                .iter()
                .filter(|p| **p != *pair)
                .sample(rng, MULTI_CHOICE_GAME_QUESTION_CHOICES - 1);

            let mut choices = other_pairs
                .iter()
                .map(|p| extract_answer(mode, *p))
                .collect::<Result<Vec<_>, _>>()?;
            choices.push(answer.clone());
            choices.shuffle(rng);

            let answer_idx = choices
                .iter()
                .position(|c| answer == *c)
                .expect("answer should be in choices");

            Ok(GameQuestion {
                prompt,
                choices: choices
                    .try_into()
                    .expect("choices should be correctly lengthed"),
                answer_idx,
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

fn extract_answer(mode: GameMode, pair: &WordPair) -> Result<String, CreateQuestionError> {
    use CreateQuestionError as Cr;
    use GameMode as GM;

    match mode {
        GM::EngToKana | GM::KanjiToKana => Ok(pair.kana.clone()),
        GM::EngToKanji | GM::KanaToKanji => pair
            .kanji
            .clone()
            .ok_or_else(|| Cr::MissingKanji(pair.clone())),
        GM::KanaToEng | GM::KanjiToEng => {
            let gloss = extract_glossary(pair)?;
            if gloss.is_empty() {
                return Err(Cr::MissingGlossary(pair.clone()));
            }
            Ok(gloss[0].clone())
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
