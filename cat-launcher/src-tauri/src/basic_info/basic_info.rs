use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::settings::Settings;
use crate::variants::GameVariant;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct Link {
    pub label: String,
    pub href: String,
}

impl GameVariant {
    pub(crate) fn id(&self) -> &'static str {
        self.into()
    }

    pub(crate) fn name<'a>(&self, settings: &'a Settings) -> &'a str {
        settings
            .games
            .get(self.id())
            .map(|g| g.name.as_str())
            .unwrap_or_else(|| self.id())
    }

    pub(crate) fn links<'a>(&self, settings: &'a Settings) -> &'a [Link] {
        match settings.games.get(self.id()) {
            Some(game_settings) => &game_settings.links,
            None => &[],
        }
    }
}
