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

const BASE_CATEGORIES: &[&str] =
  &["typeface", "map_typeface", "overmap_typeface"];
const DDA_CATEGORIES: &[&str] = &[
  "typeface",
  "map_typeface",
  "overmap_typeface",
  "gui_typeface",
];

impl GameVariant {
  pub fn id(&self) -> &'static str {
    self.into()
  }

  pub fn name(&self) -> &'static str {
    match self {
      GameVariant::DarkDaysAhead => "Dark Days Ahead",
      GameVariant::BrightNights => "Bright Nights",
      GameVariant::TheLastGeneration => "The Last Generation",
    }
  }

  pub fn determine_release_type(
    &self,
    tag_name: &str,
    prerelease: bool,
  ) -> ReleaseType {
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

  pub fn supported_typeface_categories(
    &self,
  ) -> &'static [&'static str] {
    match self {
      GameVariant::DarkDaysAhead => DDA_CATEGORIES,
      _ => BASE_CATEGORIES,
    }
  }
}
