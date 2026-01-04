use std::sync::OnceLock;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoStaticStr};
use ts_rs::TS;

use crate::game_release::game_release::ReleaseType;
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

static DDA_LINKS: OnceLock<Vec<Link>> = OnceLock::new();
static BN_LINKS: OnceLock<Vec<Link>> = OnceLock::new();
static TLG_LINKS: OnceLock<Vec<Link>> = OnceLock::new();

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

  pub fn links(&self) -> Vec<Link> {
    match self {
      GameVariant::DarkDaysAhead => DDA_LINKS
        .get_or_init(|| {
          vec![
            Link {
              label: "Guide".to_string(),
              href: "https://cdda-guide.nornagon.net/".to_string(),
            },
            Link {
              label: "Discord".to_string(),
              href: "https://discord.gg/jFEc7Yp".to_string(),
            },
            Link {
              label: "Reddit".to_string(),
              href: "https://www.reddit.com/r/cataclysmdda/"
                .to_string(),
            },
          ]
        })
        .clone(),
      GameVariant::BrightNights => BN_LINKS
        .get_or_init(|| {
          vec![
            Link {
              label: "Guide".to_string(),
              href: "https://next.cbn-guide.pages.dev/".to_string(),
            },
            Link {
              label: "Discord".to_string(),
              href: "https://discord.gg/XW7XhXuZ89".to_string(),
            },
            Link {
              label: "Reddit".to_string(),
              href: "https://www.reddit.com/r/cataclysmbn/"
                .to_string(),
            },
          ]
        })
        .clone(),
      GameVariant::TheLastGeneration => TLG_LINKS
        .get_or_init(|| {
          vec![
            Link {
              label: "Discord".to_string(),
              href: "https://discord.com/invite/zT9sXmZNCK"
                .to_string(),
            },
            Link {
              label: "Wiki".to_string(),
              href:
                "https://cataclysmtlg.miraheze.org/wiki/Main_Page"
                  .to_string(),
            },
          ]
        })
        .clone(),
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
