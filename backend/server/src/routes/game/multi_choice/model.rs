use serde::{Deserialize, Serialize};
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
#[schema(as = MultiChoiceQuestion)]
pub struct GameQuestion {
    pub prompt: String,
    pub choices: [String; MULTI_CHOICE_GAME_QUESTION_CHOICES],
    pub answer_idx: usize,
    pub word_pair: WordDto,
}

pub const MULTI_CHOICE_GAME_QUESTION_CHOICES: usize = 4;
