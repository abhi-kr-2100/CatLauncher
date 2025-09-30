use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, IntoStaticStr};
use ts_rs::TS;

#[derive(
    Debug,
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
    Default,
)]
#[non_exhaustive]
pub enum GameVariant {
    #[default]
    DarkDaysAhead,
    BrightNights,
    TheLastGeneration,
}
