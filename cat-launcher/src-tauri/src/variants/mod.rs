use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum GameVariant {
    DarkDaysAhead,
    BrightNights,
    TheLastGeneration,
}
