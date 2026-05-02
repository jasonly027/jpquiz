use std::rc::Rc;
use std::{collections::HashMap, io::Read};

use thiserror::Error;

pub use crate::jlpt::NLevel;
use crate::jmdict::{JMDictId, JMDictPartOfSpeechTag};

#[derive(Debug, Clone)]
pub struct Dictionary {
    pub words: HashMap<Rc<DictionaryId>, Word>,
}

impl Dictionary {
    pub fn load(rdr: impl Read) -> Result<Self, DictionaryLoadError> {
        let dict: persistence::Dictionary = serde_json::from_reader(rdr)?;

        let words = dict
            .words
            .into_iter()
            .map(|word| {
                let id = Rc::new(word.id);

                let senses: Vec<Rc<Sense>> =
                    word.senses.into_iter().map(|s| Rc::new(s.into())).collect();

                let pairs = word
                    .pairs
                    .into_iter()
                    .map(|pair| pair.to_runtime(id.clone(), &senses))
                    .collect();

                (id.clone(), Word { id, pairs })
            })
            .collect();

        Ok(Self { words })
    }
}

#[derive(Debug, Error)]
#[error("failed to deserialize dictionary")]
pub struct DictionaryLoadError(#[from] serde_json::Error);

pub type DictionaryId = JMDictId;

#[derive(Debug, Clone)]
pub struct Word {
    pub id: Rc<DictionaryId>,
    pub pairs: Vec<WordPair>,
}

#[derive(Debug, Clone)]
pub struct WordPair {
    pub id: Rc<DictionaryId>,
    pub kana: String,
    pub kanji: Option<String>,
    pub level: NLevel,
    pub senses: Vec<Rc<Sense>>,
}

#[derive(Debug, Clone)]
pub struct Sense {
    pub glossary: Vec<String>,
    pub parts_of_speech: Vec<PartOfSpeechTag>,
}

pub type PartOfSpeechTag = JMDictPartOfSpeechTag;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartOfSpeechCategory {
    Nouns,
    Verbs,
    Adjectives,
    Adverbs,
    Expressions,
    Conjunctions,
    Other,
}

macro_rules! is_category_fn {
    ($fn_name:ident, [$($tags:ident),+ $(,)?]) => {
        fn $fn_name(tag: PartOfSpeechTag) -> bool {
            matches!(tag, $(PartOfSpeechTag::$tags)|+)
        }
    };
}

impl PartOfSpeechCategory {
    pub fn contains(&self, tag: PartOfSpeechTag) -> bool {
        match self {
            PartOfSpeechCategory::Nouns => Self::is_noun(tag),
            PartOfSpeechCategory::Verbs => Self::is_verb(tag),
            PartOfSpeechCategory::Adjectives => Self::is_adjective(tag),
            PartOfSpeechCategory::Adverbs => Self::is_adverb(tag),
            PartOfSpeechCategory::Expressions => Self::is_expression(tag),
            PartOfSpeechCategory::Conjunctions => Self::is_conjunction(tag),
            PartOfSpeechCategory::Other => Self::is_other(tag),
        }
    }

    is_category_fn!(is_noun, [N, Pn, NPr, NPref, NSuf, Nt]);

    is_category_fn!(
        is_verb,
        [
            Vi, Vt, Vn, Vr, Vk, Vz, VsC, VsS, VsI, Vs, AuxV, VUnspec, V1, V1S, V2aS, V2gK, V2kK,
            V2rK, V2bK, V2mK, V2tK, V2dK, V2yK, V2hK, V2gS, V2wS, V2nS, V2yS, V2hS, V2kS, V2sS,
            V2rS, V2bS, V2zS, V2mS, V2dS, V2tS, V4b, V4g, V4h, V4k, V4m, V4n, V4s, V4r, V4t, V5b,
            V5g, V5k, V5n, V5m, V5r, V5t, V5s, V5u, V5kS, V5rI, V5uS, V5aru, V5uru,
        ]
    );

    is_category_fn!(
        is_adjective,
        [
            AdjI, AdjIx, AdjT, AuxAdj, AdjKari, AdjShiku, AdjNari, AdjKu, AdjNa, AdjF, AdjNo,
            AdjPn,
        ]
    );

    is_category_fn!(is_adverb, [Adv, AdvTo, NAdv,]);

    is_category_fn!(is_expression, [Exp, Int]);

    is_category_fn!(is_conjunction, [Conj]);

    is_category_fn!(is_other, [Pref, Ctr, Suf, Prt, Aux, Cop, Num, Unc,]);
}

mod persistence {
    use std::{
        collections::HashMap,
        io::{Read, Write},
        rc::Rc,
    };

    use thiserror::Error;
    use tracing::{debug, info};

    use crate::{
        dictionary as runtime,
        jlpt::{self, JLPTLoadError, JLPTPair, NLevel},
        jmdict::{self, JMDict, JMDictLoadError, JMDictSense},
    };

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct Dictionary {
        pub words: Vec<Word>,
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct Word {
        pub id: runtime::DictionaryId,
        pub pairs: Vec<WordPair>,
        pub senses: Vec<Sense>,
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct WordPair {
        pub kana: String,
        pub kanji: Option<String>,
        pub level: NLevel,
        pub sense_indices: Vec<usize>,
    }

    impl WordPair {
        pub fn to_runtime(
            self,
            id: Rc<runtime::DictionaryId>,
            senses: &[Rc<runtime::Sense>],
        ) -> runtime::WordPair {
            runtime::WordPair {
                id: id.clone(),
                kana: self.kana,
                kanji: self.kanji,
                level: self.level,
                senses: self
                    .sense_indices
                    .into_iter()
                    .map(|idx| senses[idx].clone())
                    .collect(),
            }
        }
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct Sense {
        pub glossary: Vec<String>,
        pub parts_of_speech: Vec<runtime::PartOfSpeechTag>,
    }

    impl Into<runtime::Sense> for Sense {
        fn into(self) -> runtime::Sense {
            runtime::Sense {
                glossary: self.glossary,
                parts_of_speech: self.parts_of_speech,
            }
        }
    }

    impl Into<Sense> for JMDictSense {
        fn into(self) -> Sense {
            let glossary = self
                .gloss
                .iter()
                .map(|g| {
                    // Should always be true while using JMDict English version
                    assert_eq!(g.lang, "eng");
                    g.text.clone()
                })
                .collect();

            let parts_of_speech = self.parts_of_speech.clone();

            Sense {
                glossary,
                parts_of_speech,
            }
        }
    }

    #[derive(Debug, Error)]
    pub enum GenerateDictionaryFileError {
        #[error(transparent)]
        JMDict(#[from] JMDictLoadError),
        #[error(transparent)]
        JLPT(#[from] JLPTLoadError),
        #[error("failed serialize dictionary")]
        Serialization(#[from] serde_json::Error),
    }

    pub fn generate_dictionary_file(
        jmdict_rdr: impl Read,
        jlpt_rdr: impl Read,
        writer: impl Write,
    ) -> Result<(), GenerateDictionaryFileError> {
        let jmdict = jmdict::load(jmdict_rdr)?;
        info!(
            "Loaded JMDict v{} Common={}",
            jmdict.version, jmdict.common_only,
        );
        info!("{} JMDict words", jmdict.words.len());

        let jlpt = jlpt::load(jlpt_rdr)?;
        info!("Loaded JLPT List",);
        info!("{} JLPT entries", jlpt.len());

        let mut word_map: HashMap<runtime::DictionaryId, Word> = HashMap::new();

        // Insert JLPT pairs parsed with JMDict
        jlpt.into_iter()
            .for_each(|jlpt::JLPTEntry { pair, level }| {
                let Some((id, pair)) = find_pair(&jmdict, level, pair) else {
                    return;
                };

                let word = word_map.entry(id.clone()).or_insert_with(|| Word {
                    id,
                    pairs: Vec::new(),
                    senses: Vec::new(),
                });

                word.pairs.push(pair);
            });

        // Hydrate with senses
        word_map.iter_mut().for_each(|(id, word)| {
            let jmdict_word = jmdict
                .words
                .iter()
                .find(|w| *id == w.id)
                .expect(&format!("failed to find word with dictionary_id of {id}"));

            jmdict_word.sense.iter().for_each(|sense| {
                // Store sense if it applies to at least one of the pairs

                let pair_filter = filter_by_pairs_applicable_to_sense(sense);

                let mut word_pairs = word.pairs.iter_mut().filter(|p| pair_filter(p)).peekable();
                if word_pairs.peek().is_none() {
                    return;
                }

                let sense_idx = word.senses.len();
                word_pairs.for_each(|p| p.sense_indices.push(sense_idx));
                word.senses.push(sense.clone().into());
            });
        });

        info!(
            "{} pairs matched",
            word_map.values().map(|e| e.pairs.len()).sum::<usize>()
        );

        let words = word_map.into_values().collect();
        let dictionary = Dictionary { words };

        serde_json::to_writer_pretty(writer, &dictionary)?;

        Ok(())
    }

    fn find_pair(
        jmdict: &JMDict,
        level: NLevel,
        pair: JLPTPair,
    ) -> Option<(runtime::DictionaryId, WordPair)> {
        match pair {
            JLPTPair::KanaOnly(kana) => {
                let mut words = jmdict
                    .words
                    .iter()
                    .filter(|w| w.kana.iter().any(|k| kana == k.text))
                    .peekable();

                let Some(word) = words.next().and_then(|w| {
                    if words.peek().is_some() {
                        None
                    } else {
                        Some(w)
                    }
                }) else {
                    debug!(
                        "Failed to pair {kana} ({} match)",
                        if words.peek().is_some() {
                            "ambiguous"
                        } else {
                            "no"
                        }
                    );
                    return None;
                };

                Some((
                    word.id.clone(),
                    WordPair {
                        kana,
                        kanji: None,
                        level,
                        sense_indices: Vec::new(),
                    },
                ))
            }

            JLPTPair::KanaKanji(kana, kanji) => {
                let mut words = jmdict
                    .words
                    .iter()
                    .filter(|w| {
                        w.kanji.iter().any(|k| kanji == k.text)
                            && w.kana.iter().any(|k| kana == k.text)
                    })
                    .peekable();

                let Some(word) = words.next().and_then(|w| {
                    if words.peek().is_some() {
                        None
                    } else {
                        Some(w)
                    }
                }) else {
                    debug!(
                        "Failed to pair {kana} and {kanji} ({} match)",
                        if words.peek().is_some() {
                            "ambiguous"
                        } else {
                            "no"
                        }
                    );
                    return None;
                };

                Some((
                    word.id.clone(),
                    WordPair {
                        kana,
                        kanji: Some(kanji),
                        level,
                        sense_indices: Vec::new(),
                    },
                ))
            }

            JLPTPair::KanaKanjiId(kana, kanji, jmdict_id) => Some((
                jmdict_id,
                WordPair {
                    kana,
                    kanji,
                    level,
                    sense_indices: Vec::new(),
                },
            )),
        }
    }

    fn filter_by_pairs_applicable_to_sense(sense: &JMDictSense) -> impl Fn(&WordPair) -> bool {
        |WordPair { kana, kanji, .. }: &WordPair| {
            if sense
                .applies_to_kana
                .iter()
                .any(|apply_to_kana| apply_to_kana == "*" || kana == apply_to_kana)
            {
                return true;
            }

            if kanji.as_ref().is_some_and(|kanji| {
                sense
                    .applies_to_kanji
                    .iter()
                    .any(|apply_to_kanji| apply_to_kanji == "*" || kanji == apply_to_kanji)
            }) {
                return true;
            }

            return false;
        }
    }
}

pub use persistence::generate_dictionary_file;

#[cfg(test)]
mod test {
    use std::{fs::File, path::Path};

    use crate::{Dictionary, dictionary};

    #[test]
    fn generate_dictionary_file_works() {
        let jmdict_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../static/jmdict-eng-common-3.6.2.json");
        let jmdict_reader = File::open(jmdict_path).expect("failed to open JMDict file");

        let jlpt_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../static/JLPTWords-1.4.json");
        let jlpt_reader = File::open(jlpt_path).expect("failed to open JLPT file");

        let output = std::io::sink();

        assert!(dictionary::generate_dictionary_file(jmdict_reader, jlpt_reader, output).is_ok());
    }

    #[test]
    fn dictionary_load_time() {
        let dict_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../static/dictionary.json");

        let mut total_duration = std::time::Duration::ZERO;
        let iterations = 5;

        for _ in 0..iterations {
            let dict_rdr = File::open(&dict_path).expect("failed to open dictionary file");
            let start = std::time::Instant::now();
            let _dict = Dictionary::load(dict_rdr).expect("failed to load dictionary");
            total_duration += start.elapsed();
        }

        let avg = total_duration / iterations;
        println!("Average dictionary load time over {iterations} runs: {avg:?}");
    }
}
