use crate::variants::GameVariant;

pub fn get_platform_asset_substr(variant: &GameVariant, os: &str) -> Option<&'static str> {
    match (variant, os.to_lowercase().as_str()) {
        (GameVariant::DarkDaysAhead, "windows") => Some("windows-with-graphics-and-sounds"),
        (GameVariant::DarkDaysAhead, "macos") => Some("osx-terminal-only"),
        (GameVariant::DarkDaysAhead, "linux") => Some("linux-with-graphics-and-sounds"),
        (GameVariant::BrightNights, "windows") => Some("windows-tiles"),
        (GameVariant::BrightNights, "macos") => Some("osx-tiles-arm"),
        (GameVariant::BrightNights, "linux") => Some("linux-tiles"),
        (GameVariant::TheLastGeneration, "windows") => Some("windows-tiles-sounds-x64-msvc"),
        (GameVariant::TheLastGeneration, "macos") => Some("osx-tiles-universal"),
        (GameVariant::TheLastGeneration, "linux") => Some("linux-tiles-sounds"),
        _ => None,
    }
}
