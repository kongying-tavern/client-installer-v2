use super::lang::SupportedLanguages;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Translation {
    pub menu: translation_item::Menu,
}

pub mod translation_item {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Menu {
        pub update_title: String,
        pub update_desc: String,
        pub uninstall_title: String,
        pub uninstall_desc: String,
    }
}

pub trait Translatable {
    fn to_translations(self) -> Result<Translation>;
}

impl Translatable for SupportedLanguages {
    fn to_translations(self) -> Result<Translation> {
        let raw = match self {
            SupportedLanguages::ZhHans => include_str!("./translations/zh_hans.toml"),
        };
        toml::from_str(raw).context("Failed to parse toml")
    }
}
