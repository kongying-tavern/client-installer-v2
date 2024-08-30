use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportedLanguages {
    Undefined,
    EnUs,
    ZhHans,
}

impl Default for SupportedLanguages {
    fn default() -> Self {
        SupportedLanguages::Undefined
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Language {
    pub lang: Vec<language_item::Lang>,
}

pub mod language_item {
    use super::SupportedLanguages;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Lang {
        pub name: String,
        pub code: String,
        #[serde(default)]
        pub lang: SupportedLanguages,
        pub font_class: String,
    }
}

pub trait LanguageManifest {
    fn to_lang_manifest() -> Result<Language>;
}

impl LanguageManifest for Language {
    fn to_lang_manifest() -> Result<Language> {
        let manifest = include_str!("./lang/manifest.toml");
        let parsed = toml::from_str(manifest).context("Failed to parse toml");

        parsed.map(|mut p: Language| {
            p.lang = p
                .lang
                .into_iter()
                .map(move |mut l: language_item::Lang| {
                    l.lang = match l.code.as_str() {
                        "en_us" => SupportedLanguages::EnUs,
                        "zh_hans" => SupportedLanguages::ZhHans,
                        _ => unreachable!("Unsupported language code"),
                    };
                    l
                })
                .collect::<Vec<language_item::Lang>>();
            p
        })
    }
}
