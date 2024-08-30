use super::lang::SupportedLanguages;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Translation {
    pub language: translation_item::Language,
    pub menu: translation_item::Menu,
}

pub mod translation_item {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Language {
        pub page_title: String,
        pub button_next: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Menu {
        pub page_title: String,
        pub update_title: String,
        pub update_desc: String,
        pub uninstall_title: String,
        pub uninstall_desc: String,
        pub button_prev: String,
    }
}

pub trait Translatable {
    fn to_translations(self) -> Result<Translation>;
}

impl Translatable for SupportedLanguages {
    fn to_translations(self) -> Result<Translation> {
        let raw = match self {
            SupportedLanguages::EnUs => include_str!("./translations/en_us.toml"),
            SupportedLanguages::ZhHans => include_str!("./translations/zh_hans.toml"),
            _ => unreachable!("Unsupported language"),
        };
        toml::from_str(raw).context("Failed to parse toml")
    }
}
