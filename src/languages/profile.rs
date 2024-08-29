use super::lang::SupportedLanguages;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    #[serde(default)]
    pub logo: String,
    pub theme: profile_item::Theme,
}

pub mod profile_item {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Theme {
        pub font_class: String,
    }
}

pub trait AppProfile {
    fn to_profile(self) -> Result<Profile>;
}

impl AppProfile for SupportedLanguages {
    fn to_profile(self) -> Result<Profile> {
        let logo = match self {
            SupportedLanguages::ZhHans => include_str!("./profile/zh_hans.logo.svg"),
            _ => unreachable!("Unsupported language"),
        };

        let raw = match self {
            SupportedLanguages::ZhHans => include_str!("./profile/zh_hans.toml"),
            _ => unreachable!("Unsupported language"),
        };
        let parsed = toml::from_str(raw).context("Failed to parse toml");

        parsed.map(|mut p: Profile| {
            p.logo = logo.to_string();
            p
        })
    }
}
