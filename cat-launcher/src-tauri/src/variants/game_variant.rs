use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, IntoStaticStr};
use ts_rs::TS;

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
)]
#[non_exhaustive]
pub enum GameVariant {
    DarkDaysAhead,
    BrightNights,
    TheLastGeneration,
}
