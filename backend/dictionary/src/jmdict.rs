#![allow(dead_code)]
use std::{convert::AsRef, fs::File, path::Path};

use thiserror::Error;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct JMDict {
    pub words: Vec<JMDictWord>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct JMDictWord {
    pub id: String,
    pub kana: Vec<JMDictKana>,
    pub kanji: Vec<JMDictKanji>,
    pub sense: Vec<JMDictSense>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JMDictKana {
    pub applies_to_kanji: Vec<String>,
    pub common: bool,
    pub tags: Vec<String>,
    pub text: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct JMDictKanji {
    pub common: bool,
    pub tags: Vec<String>,
    pub text: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JMDictSense {
    pub applies_to_kana: Vec<String>,
    pub applies_to_kanji: Vec<String>,
    pub gloss: Vec<JMDictGloss>,
    #[serde(rename = "partOfSpeech")]
    pub parts_of_speech: Vec<JMDictPartOfSpeechTag>,
    pub misc: Vec<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct JMDictGloss {
    pub lang: String,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize)]
pub enum JMDictTag {
    PartOfSpeech(JMDictPartOfSpeechTag),
    Misc(JMDictMiscTag),
}

macro_rules! pos_enum {
    ($(($ident:ident, $tag:literal, $desc:literal)),+) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize)]
        pub enum JMDictPartOfSpeechTag {
        $(
            #[serde(rename = $tag)]
            $ident,
        )*
        }
    };
}

#[rustfmt::skip]
pos_enum![
    (V5uru, "v5uru", "Godan verb - Uru old class verb (old form of Eru)"),
    (V2gS, "v2g-s", "Nidan verb (lower class) with 'gu' ending (archaic)"),
    (Pref, "pref", "prefix"),
    (Exp, "exp", "expressions (phrases, clauses, etc.)"),
    (V2gK, "v2g-k", "Nidan verb (upper class) with 'gu' ending (archaic)"),
    (AuxV, "aux-v", "auxiliary verb"),
    (Ctr, "ctr", "counter"),
    (V5kS, "v5k-s", "Godan verb - Iku/Yuku special class"),
    (V2wS, "v2w-s", "Nidan verb (lower class) with 'u' ending and 'we' conjugation (archaic)"),
    (NPref, "n-pref", "noun, used as a prefix"),
    (NSuf, "n-suf", "noun, used as a suffix"),
    (Suf, "suf", "suffix"),
    (V2nS, "v2n-s", "Nidan verb (lower class) with 'nu' ending (archaic)"),
    (N, "n", "noun (common) (futsuumeishi)"),
    (V5rI, "v5r-i", "Godan verb with 'ru' ending (irregular verb)"),
    (V1, "v1", "Ichidan verb"),
    (V2yK, "v2y-k", "Nidan verb (upper class) with 'yu' ending (archaic)"),
    (V2aS, "v2a-s", "Nidan verb with 'u' ending (archaic)"),
    (V5uS, "v5u-s", "Godan verb with 'u' ending (special class)"),
    (Adv, "adv", "adverb (fukushi)"),
    (Prt, "prt", "particle"),
    (Vi, "vi", "intransitive verb"),
    (V2yS, "v2y-s", "Nidan verb (lower class) with 'yu' ending (archaic)"),
    (Vk, "vk", "Kuru verb - special class"),
    (Vn, "vn", "irregular nu verb"),
    (AdjKari, "adj-kari", "'kari' adjective (archaic)"),
    (Vr, "vr", "irregular ru verb, plain form ends with -ri"),
    (Vs, "vs", "noun or participle which takes the aux. verb suru"),
    (Vt, "vt", "transitive verb"),
    (Vz, "vz", "Ichidan verb - zuru verb (alternative form of -jiru verbs)"),
    (Aux, "aux", "auxiliary"),
    (V2hS, "v2h-s", "Nidan verb (lower class) with 'hu/fu' ending (archaic)"),
    (Nt, "n-t", "noun (temporal) (jisoumeishi)"),
    (V2hK, "v2h-k", "Nidan verb (upper class) with 'hu/fu' ending (archaic)"),
    (AdvTo, "adv-to", "adverb taking the 'to' particle"),
    (Unc, "unc", "unclassified"),
    (NAdv, "n-adv", "adverbial noun (fukushitekimeishi)"),
    (AdjShiku, "adj-shiku", "'shiku' adjective (archaic)"),
    (V2kS, "v2k-s", "Nidan verb (lower class) with 'ku' ending (archaic)"),
    (Conj, "conj", "conjunction"),
    (V2sS, "v2s-s", "Nidan verb (lower class) with 'su' ending (archaic)"),
    (V2kK, "v2k-k", "Nidan verb (upper class) with 'ku' ending (archaic)"),
    (V5aru, "v5aru", "Godan verb - -aru special class"),
    (VUnspec, "v-unspec", "verb unspecified"),
    (AdjIx, "adj-ix", "adjective (keiyoushi) - yoi/ii class"),
    (AdjNari, "adj-nari", "archaic/formal form of na-adjective"),
    (V2rK, "v2r-k", "Nidan verb (upper class) with 'ru' ending (archaic)"),
    (AdjF, "adj-f", "noun or verb acting prenominally"),
    (AdjI, "adj-i", "adjective (keiyoushi)"),
    (AdjT, "adj-t", "'taru' adjective"),
    (V2rS, "v2r-s", "Nidan verb (lower class) with 'ru' ending (archaic)"),
    (V2bK, "v2b-k", "Nidan verb (upper class) with 'bu' ending (archaic)"),
    (VsS, "vs-s", "suru verb - special class"),
    (V2bS, "v2b-s", "Nidan verb (lower class) with 'bu' ending (archaic)"),
    (VsC, "vs-c", "su verb - precursor to the modern suru"),
    (AdjKu, "adj-ku", "'ku' adjective (archaic)"),
    (VsI, "vs-i", "suru verb - included"),
    (V2zS, "v2z-s", "Nidan verb (lower class) with 'zu' ending (archaic)"),
    (V2mS, "v2m-s", "Nidan verb (lower class) with 'mu' ending (archaic)"),
    (Cop, "cop", "copula"),
    (Num, "num", "numeric"),
    (AuxAdj, "aux-adj", "auxiliary adjective"),
    (V1S, "v1-s", "Ichidan verb - kureru special class"),
    (V2mK, "v2m-k", "Nidan verb (upper class) with 'mu' ending (archaic)"),
    (AdjNo, "adj-no", "nouns which may take the genitive case particle 'no'"),
    (AdjNa, "adj-na", "adjectival nouns or quasi-adjectives (keiyodoshi)"),
    (V4b, "v4b", "Yodan verb with 'bu' ending (archaic)"),
    (V4g, "v4g", "Yodan verb with 'gu' ending (archaic)"),
    (V4h, "v4h", "Yodan verb with 'hu/fu' ending (archaic)"),
    (V4k, "v4k", "Yodan verb with 'ku' ending (archaic)"),
    (V4m, "v4m", "Yodan verb with 'mu' ending (archaic)"),
    (V4n, "v4n", "Yodan verb with 'nu' ending (archaic)"),
    (V4s, "v4s", "Yodan verb with 'su' ending (archaic)"),
    (V4r, "v4r", "Yodan verb with 'ru' ending (archaic)"),
    (V4t, "v4t", "Yodan verb with 'tsu' ending (archaic)"),
    (V2tK, "v2t-k", "Nidan verb (upper class) with 'tsu' ending (archaic)"),
    (V5b, "v5b", "Godan verb with 'bu' ending"),
    (V2tS, "v2t-s", "Nidan verb (lower class) with 'tsu' ending (archaic)"),
    (V5g, "v5g", "Godan verb with 'gu' ending"),
    (V5k, "v5k", "Godan verb with 'ku' ending"),
    (V5n, "v5n", "Godan verb with 'nu' ending"),
    (V5m, "v5m", "Godan verb with 'mu' ending"),
    (V2dK, "v2d-k", "Nidan verb (upper class) with 'dzu' ending (archaic)"),
    (V5r, "v5r", "Godan verb with 'ru' ending"),
    (V5t, "v5t", "Godan verb with 'tsu' ending"),
    (V5s, "v5s", "Godan verb with 'su' ending"),
    (V5u, "v5u", "Godan verb with 'u' ending"),
    (V2dS, "v2d-s", "Nidan verb (lower class) with 'dzu' ending (archaic)"),
    (AdjPn, "adj-pn", "pre-noun adjectival (rentaishi)"),
    (Int, "int", "interjection (kandoushi)"),
    (NPr, "n-pr", "proper noun"),
    (Pn, "pn", "pronoun")
];

pub type JMDictMiscTag = String;

#[derive(Debug, Error)]
pub enum JMDictLoadError {
    #[error("failed to load JMDict file")]
    File(std::io::Error),
    #[error("failed to deserialize JMDict file")]
    Deserialize(serde_json::Error),
}

pub fn load(path: impl AsRef<Path>) -> Result<JMDict, JMDictLoadError> {
    let jmdict_file = File::open(path).map_err(JMDictLoadError::File)?;
    serde_json::from_reader(jmdict_file).map_err(JMDictLoadError::Deserialize)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::jmdict;

    #[test]
    fn load_works() {
        let jmdict_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../static/jmdict-eng-common-3.6.2.json");

        assert!(jmdict::load(jmdict_path).is_ok());
    }
}
