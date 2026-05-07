use crate::{NLevel, PartOfSpeechCategory, Sense, Word, WordPair};

pub trait ContainsPartOfSpeechCategory {
    fn contains_pos(&self, category: PartOfSpeechCategory) -> bool;
}

impl ContainsPartOfSpeechCategory for Word {
    fn contains_pos(&self, category: PartOfSpeechCategory) -> bool {
        self.pairs.iter().any(|p| p.contains_pos(category))
    }
}

impl ContainsPartOfSpeechCategory for WordPair {
    fn contains_pos(&self, category: PartOfSpeechCategory) -> bool {
        self.senses.iter().any(|s| s.contains_pos(category))
    }
}

impl ContainsPartOfSpeechCategory for Sense {
    fn contains_pos(&self, category: PartOfSpeechCategory) -> bool {
        self.parts_of_speech.iter().any(|p| category.contains(*p))
    }
}

pub trait ContainsNLevel {
    fn contains_level(&self, level: NLevel) -> bool;
}

impl ContainsNLevel for Word {
    fn contains_level(&self, level: NLevel) -> bool {
        self.pairs.iter().any(|p| p.level == level)
    }
}

pub mod word_pair {
    use std::{collections::HashSet, sync::Arc};

    use crate::{NLevel, PartOfSpeechCategory, Sense, WordPair};

    pub fn levels(levels: impl Iterator<Item = NLevel>) -> impl Fn(&WordPair) -> bool {
        let levels: HashSet<NLevel> = levels.collect();
        move |pair| levels.contains(&pair.level)
    }

    pub fn categories(
        categories: impl Iterator<Item = PartOfSpeechCategory>,
    ) -> impl Fn(&WordPair) -> Option<WordPair> {
        let categories: HashSet<PartOfSpeechCategory> = categories.collect();

        move |pair| {
            let senses: Vec<Arc<Sense>> = pair
                .senses
                .iter()
                .filter(|sense| {
                    sense
                        .parts_of_speech
                        .iter()
                        .any(|pos| categories.iter().any(|cat| cat.contains(*pos)))
                })
                .cloned()
                .collect();

            if senses.is_empty() {
                return None;
            }

            Some(WordPair {
                senses,
                ..pair.clone()
            })
        }
    }

    pub fn has_kanji(pair: &WordPair) -> bool {
        pair.kanji.is_some()
    }

    pub fn passthrough(_pair: &WordPair) -> bool {
        true
    }
}
