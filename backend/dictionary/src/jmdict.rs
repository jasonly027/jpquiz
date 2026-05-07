use std::{
    fmt::{self, Display},
    io::Read,
};

use thiserror::Error;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JMDict {
    pub common_only: bool,
    pub version: String,
    pub words: Vec<JMDictWord>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct JMDictWord {
    pub id: JMDictId,
    pub kana: Vec<JMDictKana>,
    pub kanji: Vec<JMDictKanji>,
    pub sense: Vec<JMDictSense>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub struct JMDictId(pub String);

impl Display for JMDictId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct JMDictKana {
    pub applies_to_kanji: Vec<String>,
    pub common: bool,
    pub tags: Vec<String>,
    pub text: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[allow(dead_code)]
pub struct JMDictKanji {
    pub common: bool,
    pub tags: Vec<String>,
    pub text: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
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
#[allow(dead_code)]
pub enum JMDictTag {
    PartOfSpeech(JMDictPartOfSpeechTag),
    Misc(JMDictMiscTag),
}
macro_rules! pos_enum {
    ($(($ident:ident, $tag:literal, $desc:literal)),+ $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
        pub enum JMDictPartOfSpeechTag {
        $(
            #[serde(rename = $tag)]
            $ident,
        )*
        }

        impl JMDictPartOfSpeechTag {
            pub fn detail(&self) -> &'static str {
                match self {
                    $(Self::$ident => $desc),*
                }
            }
        }
    };
}

#[rustfmt::skip]
pos_enum![
    // Nouns
    (N, "n", "noun (common) (futsuumeishi)"),
    (Pn, "pn", "pronoun"),
    (NPr, "n-pr", "proper noun"),
    (NPref, "n-pref", "noun, used as a prefix"),
    (NSuf, "n-suf", "noun, used as a suffix"),
    (Nt, "n-t", "noun (temporal) (jisoumeishi)"),


    // Verbs
    (Vi, "vi", "intransitive verb"),
    (Vt, "vt", "transitive verb"),
    (Vn, "vn", "irregular nu verb"),
    (Vr, "vr", "irregular ru verb, plain form ends with -ri"),
    (Vk, "vk", "Kuru verb - special class"),
    (Vz, "vz", "Ichidan verb - zuru verb (alternative form of -jiru verbs)"),
    (VsC, "vs-c", "su verb - precursor to the modern suru"),
    (VsS, "vs-s", "suru verb - special class"),
    (VsI, "vs-i", "suru verb - included"),
    (Vs, "vs", "noun or participle which takes the aux. verb suru"),
    (AuxV, "aux-v", "auxiliary verb"),
    (VUnspec, "v-unspec", "verb unspecified"),

    (V1, "v1", "Ichidan verb"),
    (V1S, "v1-s", "Ichidan verb - kureru special class"),

    (V2aS, "v2a-s", "Nidan verb with 'u' ending (archaic)"),
    (V2gK, "v2g-k", "Nidan verb (upper class) with 'gu' ending (archaic)"),
    (V2kK, "v2k-k", "Nidan verb (upper class) with 'ku' ending (archaic)"),
    (V2rK, "v2r-k", "Nidan verb (upper class) with 'ru' ending (archaic)"),
    (V2bK, "v2b-k", "Nidan verb (upper class) with 'bu' ending (archaic)"),
    (V2mK, "v2m-k", "Nidan verb (upper class) with 'mu' ending (archaic)"),
    (V2tK, "v2t-k", "Nidan verb (upper class) with 'tsu' ending (archaic)"),
    (V2dK, "v2d-k", "Nidan verb (upper class) with 'dzu' ending (archaic)"),
    (V2yK, "v2y-k", "Nidan verb (upper class) with 'yu' ending (archaic)"),
    (V2hK, "v2h-k", "Nidan verb (upper class) with 'hu/fu' ending (archaic)"),
    (V2gS, "v2g-s", "Nidan verb (lower class) with 'gu' ending (archaic)"),
    (V2wS, "v2w-s", "Nidan verb (lower class) with 'u' ending and 'we' conjugation (archaic)"),
    (V2nS, "v2n-s", "Nidan verb (lower class) with 'nu' ending (archaic)"),
    (V2yS, "v2y-s", "Nidan verb (lower class) with 'yu' ending (archaic)"),
    (V2hS, "v2h-s", "Nidan verb (lower class) with 'hu/fu' ending (archaic)"),
    (V2kS, "v2k-s", "Nidan verb (lower class) with 'ku' ending (archaic)"),
    (V2sS, "v2s-s", "Nidan verb (lower class) with 'su' ending (archaic)"),
    (V2rS, "v2r-s", "Nidan verb (lower class) with 'ru' ending (archaic)"),
    (V2bS, "v2b-s", "Nidan verb (lower class) with 'bu' ending (archaic)"),
    (V2zS, "v2z-s", "Nidan verb (lower class) with 'zu' ending (archaic)"),
    (V2mS, "v2m-s", "Nidan verb (lower class) with 'mu' ending (archaic)"),
    (V2dS, "v2d-s", "Nidan verb (lower class) with 'dzu' ending (archaic)"),
    (V2tS, "v2t-s", "Nidan verb (lower class) with 'tsu' ending (archaic)"),

    (V4b, "v4b", "Yodan verb with 'bu' ending (archaic)"),
    (V4g, "v4g", "Yodan verb with 'gu' ending (archaic)"),
    (V4h, "v4h", "Yodan verb with 'hu/fu' ending (archaic)"),
    (V4k, "v4k", "Yodan verb with 'ku' ending (archaic)"),
    (V4m, "v4m", "Yodan verb with 'mu' ending (archaic)"),
    (V4n, "v4n", "Yodan verb with 'nu' ending (archaic)"),
    (V4s, "v4s", "Yodan verb with 'su' ending (archaic)"),
    (V4r, "v4r", "Yodan verb with 'ru' ending (archaic)"),
    (V4t, "v4t", "Yodan verb with 'tsu' ending (archaic)"),

    (V5b, "v5b", "Godan verb with 'bu' ending"),
    (V5g, "v5g", "Godan verb with 'gu' ending"),
    (V5k, "v5k", "Godan verb with 'ku' ending"),
    (V5n, "v5n", "Godan verb with 'nu' ending"),
    (V5m, "v5m", "Godan verb with 'mu' ending"),
    (V5r, "v5r", "Godan verb with 'ru' ending"),
    (V5t, "v5t", "Godan verb with 'tsu' ending"),
    (V5s, "v5s", "Godan verb with 'su' ending"),
    (V5u, "v5u", "Godan verb with 'u' ending"),
    (V5kS, "v5k-s", "Godan verb - Iku/Yuku special class"),
    (V5rI, "v5r-i", "Godan verb with 'ru' ending (irregular verb)"),
    (V5uS, "v5u-s", "Godan verb with 'u' ending (special class)"),
    (V5aru, "v5aru", "Godan verb - -aru special class"),
    (V5uru, "v5uru", "Godan verb - Uru old class verb (old form of Eru)"),


    // Adjectives
    (AdjI, "adj-i", "adjective (keiyoushi)"),
    (AdjIx, "adj-ix", "adjective (keiyoushi) - yoi/ii class"),
    (AdjT, "adj-t", "'taru' adjective"),
    (AuxAdj, "aux-adj", "auxiliary adjective"),
    (AdjKari, "adj-kari", "'kari' adjective (archaic)"),
    (AdjShiku, "adj-shiku", "'shiku' adjective (archaic)"),
    (AdjNari, "adj-nari", "archaic/formal form of na-adjective"),
    (AdjKu, "adj-ku", "'ku' adjective (archaic)"),
    (AdjNa, "adj-na", "adjectival nouns or quasi-adjectives (keiyodoshi)"),
    (AdjF, "adj-f", "noun or verb acting prenominally"),
    (AdjNo, "adj-no", "nouns which may take the genitive case particle 'no'"),
    (AdjPn, "adj-pn", "pre-noun adjectival (rentaishi)"),


    // Adverb
    (Adv, "adv", "adverb (fukushi)"),
    (AdvTo, "adv-to", "adverb taking the 'to' particle"),
    (NAdv, "n-adv", "adverbial noun (fukushitekimeishi)"),


    // Expressions
    (Exp, "exp", "expressions (phrases, clauses, etc.)"),
    (Int, "int", "interjection (kandoushi)"),


    // Conjunctions
    (Conj, "conj", "conjunction"),


    // Other
    (Pref, "pref", "prefix"),
    (Ctr, "ctr", "counter"),
    (Suf, "suf", "suffix"),
    (Prt, "prt", "particle"),
    (Aux, "aux", "auxiliary"),
    (Cop, "cop", "copula"),
    (Num, "num", "numeric"),
    (Unc, "unc", "unclassified"),
];

pub type JMDictMiscTag = String;

#[derive(Debug, Error)]
#[error("failed to deserialize JMDict")]
pub struct JMDictLoadError(#[from] serde_json::Error);

pub fn load(rdr: impl Read) -> Result<JMDict, JMDictLoadError> {
    Ok(serde_json::from_reader(rdr)?)
}

#[cfg(test)]
mod tests {
    use std::{fs::File, path::Path};

    use crate::jmdict;

    #[test]
    fn load_works() {
        let jmdict_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../static/jmdict-eng-common-3.6.2.json");
        let jmdict_reader = File::open(jmdict_path).expect("failed to open JMDict file");

        let dict = jmdict::load(jmdict_reader);

        assert!(dict.is_ok());
    }
}
