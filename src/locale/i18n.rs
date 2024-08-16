use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct I18n {
    pub header: i18n_item::Header,
}

pub mod i18n_item {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Header {
        pub org_logo: String,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Language {
    ZhHans,
}

impl Language {
    pub fn to_i18n(self) -> Result<I18n> {
        let raw = match self {
            Language::ZhHans => include_str!("./translations/zh_hans.toml"),
        };
        toml::from_str(raw).context("Failed to parse toml")
    }
}
