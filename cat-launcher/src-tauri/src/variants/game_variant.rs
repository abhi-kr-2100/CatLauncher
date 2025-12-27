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

  pub async fn name(&self, settings: &Settings) -> String {
    settings
      .games()
      .await
      .unwrap_or_default()
      .get(self.id())
      .map(|variant_settings| variant_settings.name.clone())
      .unwrap_or_else(|| self.id().to_string())
  }

  pub async fn links(&self, settings: &Settings) -> Vec<Link> {
    settings
      .games()
      .await
      .unwrap_or_default()
      .get(self.id())
      .map(|variant_settings| variant_settings.links.clone())
      .unwrap_or_default()
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
}
