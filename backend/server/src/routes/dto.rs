use serde::{Deserialize, Serialize};
use server_derive::EnumDtoToDomain;
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, ToSchema, EnumDtoToDomain,
)]
#[dto_to_domain(target = dictionary::NLevel)]
pub enum NLevelDto {
    N1,
    N2,
    N3,
    N4,
    N5,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, ToSchema, EnumDtoToDomain,
)]
#[dto_to_domain(target = dictionary::PartOfSpeechCategory)]
#[serde(rename_all = "lowercase")]
pub enum PartOfSpeechCategoryDto {
    Nouns,
    Verbs,
    Adjectives,
    Adverbs,
    Expressions,
    Conjunctions,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct WordDto {
    id: String,
    kana: String,
    kanji: Option<String>,
    level: NLevelDto,
    senses: Vec<SenseDto>,
}

impl Into<WordDto> for dictionary::WordPair {
    fn into(self) -> WordDto {
        WordDto {
            id: self.id.0.clone(),
            kana: self.kana,
            kanji: self.kanji,
            level: self.level.into(),
            senses: self
                .senses
                .into_iter()
                .map(|s| (*s).clone().into())
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SenseDto {
    glossary: Vec<String>,
    parts_of_speech: Vec<&'static str>,
}

impl Into<SenseDto> for dictionary::Sense {
    fn into(self) -> SenseDto {
        SenseDto {
            glossary: self.glossary,
            parts_of_speech: self
                .parts_of_speech
                .into_iter()
                .map(|p| p.detail())
                .collect(),
        }
    }
}
