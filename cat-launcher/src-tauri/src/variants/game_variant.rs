use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoStaticStr};
use ts_rs::TS;

use crate::game_release::game_release::ReleaseType;

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
