use serde::Serialize;
use utoipa::ToSchema;

use crate::routes::dto::WordDto;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
#[schema(as = FreeResponseQuestion)]
pub struct GameQuestion {
    pub prompt: String,
    pub answers: Vec<String>,
    pub word_pair: WordDto,
}
