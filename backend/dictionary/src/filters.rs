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

    pub fn matches_pair(&self, pair: &WordPair) -> bool {
        self.level.is_none_or(|l| l == pair.level) && self.pos.is_none_or(|p| pair.contains_pos(p))
    }

    pub fn matches_sense(&self, sense: &Sense) -> bool {
        self.pos.is_none_or(|p| sense.contains_pos(p))
    }
}
