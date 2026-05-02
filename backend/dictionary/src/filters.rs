use std::rc::Rc;

use crate::{Dictionary, NLevel, PartOfSpeechCategory, Sense, Word, WordPair};

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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct WordPairFilter {
    pub level: Option<NLevel>,
    pub pos: Option<PartOfSpeechCategory>,
}

impl WordPairFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn level(mut self, level: NLevel) -> Self {
        self.level = Some(level);
        self
    }

    pub fn pos(mut self, pos: PartOfSpeechCategory) -> Self {
        self.pos = Some(pos);
        self
    }

    pub fn matches_sense(&self, sense: &Sense) -> bool {
        self.pos.is_none_or(|p| sense.contains_pos(p))
    }

    /// Constructs a new [`WordPair`] containing only the senses that match the filter.
    /// Returns [`None`] if `pair` does not match the filter level, or if no senses match.
    pub fn apply(&self, pair: &WordPair) -> Option<WordPair> {
        if self.level.is_some_and(|l| l != pair.level) {
            return None;
        }

        let senses: Vec<Rc<Sense>> = pair
            .senses
            .iter()
            .filter(|s| self.matches_sense(s))
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

    pub fn pairs(&self, dict: &Dictionary) -> impl Iterator<Item = WordPair> {
        dict.words
            .values()
            .flat_map(|word| word.pairs.iter().filter_map(|pair| self.apply(pair)))
    }
}
