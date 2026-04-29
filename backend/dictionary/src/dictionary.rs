#![allow(dead_code)]
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::rc::Rc;

use thiserror::Error;

use crate::jlpt::NLevel;
use crate::jmdict::{JMDictId, JMDictPartOfSpeechTag};

#[derive(Debug)]
pub struct Dictionary {
    words: HashMap<Rc<DictionaryId>, Word>,
}

impl Dictionary {
    pub fn load(dictionary_path: impl AsRef<Path>) -> Result<Self, DictionaryLoadError> {
        let dict_file = File::open(dictionary_path).map_err(DictionaryLoadError::File)?;
        let dict_raw: persistence::Dictionary =
            serde_json::from_reader(dict_file).map_err(DictionaryLoadError::Deserialize)?;

        let words = dict_raw
            .words
            .into_iter()
            .map(|(id, word_raw)| {
                let id = Rc::new(id);

                let senses: Vec<Rc<Sense>> = word_raw
                    .senses
                    .into_iter()
                    .map(|s| Rc::new(s.into()))
                    .collect();

                let pairs = word_raw
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
pub enum DictionaryLoadError {
    #[error("failed to open dictionary file")]
    File(std::io::Error),
    #[error("failed to deserialize dictionary")]
    Deserialize(serde_json::Error),
}

pub type DictionaryId = JMDictId;

#[derive(Debug)]
pub struct Word {
    id: Rc<DictionaryId>,
    pairs: Vec<WordPair>,
}

#[derive(Debug)]
pub struct WordPair {
    id: Rc<DictionaryId>,
    kana: String,
    kanji: Option<String>,
    level: NLevel,
    senses: Vec<Rc<Sense>>,
}

#[derive(Debug)]
pub struct Sense {
    glossary: Vec<String>,
    parts_of_speech: Vec<PartOfSpeechTag>,
}

pub type PartOfSpeechTag = JMDictPartOfSpeechTag;

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
        pub words: HashMap<runtime::DictionaryId, Word>,
    }

    #[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
    pub struct Word {
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
        JMDict(JMDictLoadError),
        #[error(transparent)]
        JLPT(JLPTLoadError),
        #[error("failed to open output file")]
        FileWrite(std::io::Error),
        #[error("failed serialize dictionary")]
        Serialization(serde_json::Error),
    }

    pub fn generate_dictionary_file(
        jmdict_rdr: impl Read,
        jlpt_rdr: impl Read,
        writer: impl Write,
    ) -> Result<(), GenerateDictionaryFileError> {
        let jmdict = jmdict::load(jmdict_rdr).map_err(GenerateDictionaryFileError::JMDict)?;
        info!(
            "Loaded JMDict v{} Common={}",
            jmdict.version, jmdict.common_only,
        );
        info!("{} JMDict words", jmdict.words.len());

        let jlpt = jlpt::load(jlpt_rdr).map_err(GenerateDictionaryFileError::JLPT)?;
        info!("Loaded JLPT List",);
        info!("{} JLPT entries", jlpt.len());

        let mut dictionary: HashMap<runtime::DictionaryId, Word> = HashMap::new();

        // Insert JLPT pairs parsed with JMDict
        jlpt.into_iter()
            .for_each(|jlpt::JLPTEntry { pair, level }| {
                let Some((id, pair)) = find_pair(&jmdict, level, pair) else {
                    return;
                };
                let word = dictionary.entry(id).or_insert_with(Word::default);
                word.pairs.push(pair);
            });

        // Hydrate with senses
        dictionary.iter_mut().for_each(|(id, word)| {
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
            dictionary.values().map(|e| e.pairs.len()).sum::<usize>()
        );

        serde_json::to_writer_pretty(writer, &dictionary)
            .map_err(GenerateDictionaryFileError::Serialization)?;

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

    use crate::dictionary;

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
}
