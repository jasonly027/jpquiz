use std::{collections::HashMap, io::Read};

use thiserror::Error;

use crate::jmdict::JMDictId;

#[derive(Debug, Clone)]
pub struct JLPTEntry {
    pub pair: JLPTPair,
    pub level: NLevel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JLPTPair {
    KanaOnly(String),
    KanaKanji(String, String),
    KanaKanjiId(String, Option<String>, JMDictId),
}

impl JLPTPair {
    fn try_new(key: String, hint: String) -> Option<Self> {
        if key == hint {
            return Some(Self::KanaOnly(key));
        }

        if hint.contains(|c| c == '（' || c == '）') {
            // [1]: Not in JMDict English Common version
            // [2]: Indiscernable

            return match hint.as_ref() {
                "（感）" if key == "はあ" => None,       // [1]
                "（する）" if key == "しいんと" => None, // [1]
                "（接。感）" if key == "それ" => None,   // [1]
                "（終わり）" if key == "しまい" => None, // [1]
                "（感）" if key == "すみませ" => None,   // [2]
                "発生）" if key == "発生）" => None,     // [2]
                "（接。副）" if key == "どう" => None,   // [2]
                "（感）" if key == "はい" => None,       // [2]
                "（感）" if key == "よろしく" => Some(Self::KanaKanjiId(
                    "よろしく".to_string(),
                    Some("宜しく".to_string()),
                    JMDictId(1224890.to_string()),
                )),
                "（1000" if key == "とん" => Some(Self::KanaKanjiId(
                    "トン".to_string(),
                    Some("屯".to_string()),
                    JMDictId(1457320.to_string()),
                )),
                "（終わる）" if key == "しまう" => Some(Self::KanaKanjiId(
                    "しまう".to_string(),
                    None,
                    JMDictId(1305380.to_string()),
                )),
                "（副）" if key == "だいいち" => Some(Self::KanaKanjiId(
                    "だいいち".to_string(),
                    Some("第一".to_string()),
                    JMDictId(1415270.to_string()),
                )),
                "（可能。出現。" if key == "できる" => Some(Self::KanaKanjiId(
                    "できる".to_string(),
                    Some("出来る".to_string()),
                    JMDictId(1340450.to_string()),
                )),
                "（メートル）" if key == "ミリ" => Some(Self::KanaKanjiId(
                    "ミリ".to_string(),
                    None,
                    JMDictId(1131830.to_string()),
                )),
                "（副）" if key == "ふと" => Some(Self::KanaKanjiId(
                    "ふと".to_string(),
                    None,
                    JMDictId(1493240.to_string()),
                )),
                "（カーペット）" if key == "じゅうたん" => Some(Self::KanaKanjiId(
                    "じゅうたん".to_string(),
                    None,
                    JMDictId(1595370.to_string()),
                )),
                "（感）" if key == "しまった" => Some(Self::KanaKanjiId(
                    "しまった".to_string(),
                    None,
                    JMDictId(1005600.to_string()),
                )),
                "（感）" if key == "うん" => Some(Self::KanaKanjiId(
                    "うん".to_string(),
                    None,
                    JMDictId(1001090.to_string()),
                )),
                "（no）" if key == "ノー" => Some(Self::KanaKanjiId(
                    "ノー".to_string(),
                    None,
                    JMDictId(2080530.to_string()),
                )),
                "（感）" if key == "ね" => Some(Self::KanaKanjiId(
                    "ね".to_string(),
                    None,
                    JMDictId(2029080.to_string()),
                )),
                _ => {
                    panic!("unhandled malformed hint: {key}: {hint}");
                }
            };
        }

        return Some(Self::KanaKanji(hint, key));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "u8")]
#[serde(into = "u8")]
pub enum NLevel {
    N1,
    N2,
    N3,
    N4,
    N5,
}

impl TryFrom<u8> for NLevel {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::N1),
            2 => Ok(Self::N2),
            3 => Ok(Self::N3),
            4 => Ok(Self::N4),
            5 => Ok(Self::N5),
            _ => Err("invalid NLevel, expected 1-5"),
        }
    }
}

impl From<NLevel> for u8 {
    fn from(value: NLevel) -> Self {
        match value {
            NLevel::N1 => 1,
            NLevel::N2 => 2,
            NLevel::N3 => 3,
            NLevel::N4 => 4,
            NLevel::N5 => 5,
        }
    }
}

#[derive(Debug, Error)]
#[error("failed to deserialize JLPT")]
pub struct JLPTLoadError(#[from] serde_json::Error);

pub fn load(rdr: impl Read) -> Result<Vec<JLPTEntry>, JLPTLoadError> {
    let jlpt_raw = load_raw(rdr)?;

    Ok(jlpt_raw
        .into_iter()
        .flat_map(|(key, readings)| {
            readings
                .into_iter()
                .filter_map(move |JLPTReading { hint, level }| {
                    Some(JLPTEntry {
                        pair: JLPTPair::try_new(key.clone(), hint)?,
                        level,
                    })
                })
        })
        .collect())
}

/// A key represents either the kanji or kana form of a word. An element of the value array
/// may be:
/// - The kana form of the word.
///     - If the key was in kana, this could be it repeated or an alternate form.
/// - The category of part of speech surrounded by parentheses
type RawJLPT = HashMap<String, Vec<JLPTReading>>;

#[derive(Debug, Clone, serde::Deserialize)]
struct JLPTReading {
    #[serde(rename = "reading")]
    hint: String,
    level: NLevel,
}

fn load_raw(rdr: impl Read) -> Result<RawJLPT, JLPTLoadError> {
    Ok(serde_json::from_reader(rdr)?)
}

#[cfg(test)]
mod tests {
    use std::{fs::File, path::Path};

    use crate::jlpt;

    #[test]
    fn load_works() {
        let jlpt_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../static/JLPTWords-1.4.json");
        let jlpt_reader = File::open(jlpt_path).expect("failed to open JLPT file");

        assert!(jlpt::load(jlpt_reader).is_ok());
    }
}
