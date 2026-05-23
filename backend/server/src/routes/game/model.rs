use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
