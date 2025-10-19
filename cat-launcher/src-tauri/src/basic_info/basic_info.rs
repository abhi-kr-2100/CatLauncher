use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::variants::GameVariant;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct Link {
    pub label: &'static str,
    pub href: &'static str,
}

impl GameVariant {
    pub(crate) fn id(&self) -> &'static str {
        self.into()
    }

    pub(crate) fn name(&self) -> &'static str {
        match self {
            GameVariant::DarkDaysAhead => "Dark Days Ahead",
            GameVariant::BrightNights => "Bright Nights",
            GameVariant::TheLastGeneration => "The Last Generation",
        }
    }

    pub(crate) fn links(&self) -> Vec<Link> {
        match self {
            GameVariant::DarkDaysAhead => vec![
                Link {
                    label: "Guide",
                    href: "https://cdda-guide.nornagon.net/",
                },
                Link {
                    label: "Discord",
                    href: "https://discord.gg/jFEc7Yp",
                },
                Link {
                    label: "Reddit",
                    href: "https://www.reddit.com/r/cataclysmdda/",
                },
            ],
            GameVariant::BrightNights => vec![
                Link {
                    label: "Guide",
                    href: "https://next.cbn-guide.pages.dev/",
                },
                Link {
                    label: "Discord",
                    href: "https://discord.gg/XW7XhXuZ89",
                },
                Link {
                    label: "Reddit",
                    href: "https://www.reddit.com/r/cataclysmbn/",
                },
            ],
            GameVariant::TheLastGeneration => vec![Link {
                label: "Discord",
                href: "https://discord.com/invite/zT9sXmZNCK",
            }],
        }
    }
}
