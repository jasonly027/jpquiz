use dictionary::WordPair;
use rand::seq::{IndexedRandom, IteratorRandom, SliceRandom};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

use crate::routes::dto::WordDto;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum GameMode {
    EngToKana,
    EngToKanji,
    KanaToKanji,
    KanaToEng,
    KanjiToKana,
    KanjiToEng,
}

pub const MULTI_CHOICE_GAME_QUESTION_CHOICES: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
#[schema(as = MultiChoiceQuestion)]
pub struct GameQuestion {
    prompt: String,
    choices: [String; MULTI_CHOICE_GAME_QUESTION_CHOICES],
    answer_idx: usize,
    word_pair: WordDto,
}

#[derive(Debug, Clone, Error)]
pub enum CreateQuestionsError {
    #[error(
        "not enough pairs to create questions. requires at least {MULTI_CHOICE_GAME_QUESTION_CHOICES}"
    )]
    InsufficientPairs,
    #[error(transparent)]
    QuestionConstruction(#[from] CreateQuestionError),
}

pub fn create_questions(
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
pub enum CreateQuestionError {
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
