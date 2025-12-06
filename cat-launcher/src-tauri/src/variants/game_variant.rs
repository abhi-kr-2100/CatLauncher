use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoStaticStr};
use ts_rs::TS;

use crate::game_release::game_release::ReleaseType;
use crate::settings::Settings;
use crate::variants::links::Link;

#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    EnumIter,
    Deserialize,
    Serialize,
    IntoStaticStr,
    TS,
    EnumString,
)]
#[non_exhaustive]
pub enum GameVariant {
    DarkDaysAhead,
    BrightNights,
    TheLastGeneration,
}

impl GameVariant {
    pub fn id(&self) -> &'static str {
        self.into()
    }

    pub fn name<'a>(&self, settings: &'a Settings) -> &'a str {
        settings
            .games
            .get(self.id())
            .map(|variant_settings| variant_settings.name.as_str())
            .unwrap_or_else(|| self.id())
    }

    pub fn links<'a>(&self, settings: &'a Settings) -> &'a [Link] {
        settings
            .games
            .get(self.id())
            .map(|variant_settings| variant_settings.links.as_slice())
            .unwrap_or_else(|| &[])
    }

    pub fn determine_release_type(&self, tag_name: &str, prerelease: bool) -> ReleaseType {
        match self {
            GameVariant::DarkDaysAhead => {
                if !prerelease {
                    ReleaseType::Stable
                } else if tag_name.contains("experimental") {
                    ReleaseType::Experimental
                } else {
                    ReleaseType::ReleaseCandidate
                }
            }
            _ => {
                if prerelease {
                    ReleaseType::Experimental
                } else {
                    ReleaseType::Stable
                }
            }
        }
    }
}
