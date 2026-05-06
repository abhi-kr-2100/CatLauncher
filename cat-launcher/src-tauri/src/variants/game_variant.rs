use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoStaticStr};
use ts_rs::TS;

use crate::game_release::game_release::ReleaseType;

/// Represents the different variants of the game supported by the launcher.
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
  /// Cataclysm: Dark Days Ahead
  DarkDaysAhead,
  /// Cataclysm: Bright Nights
  BrightNights,
  /// The Last Generation
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
  /// Returns a stable string identifier for the variant.
  pub fn id(&self) -> &'static str {
    self.into()
  }

  /// Returns the human-readable name of the variant.
  pub fn name(&self) -> &'static str {
    match self {
      GameVariant::DarkDaysAhead => "Dark Days Ahead",
      GameVariant::BrightNights => "Bright Nights",
      GameVariant::TheLastGeneration => "The Last Generation",
    }
  }

  /// Determines the `ReleaseType` based on the tag name and prerelease flag,
  /// which varies depending on the variant's naming conventions.
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

  /// Returns the list of typeface categories supported by this variant.
  pub fn supported_typeface_categories(
    &self,
  ) -> &'static [&'static str] {
    match self {
      GameVariant::DarkDaysAhead => DDA_CATEGORIES,
      _ => BASE_CATEGORIES,
    }
  }
}
