pub mod colors;
pub mod commands;
pub mod consts;
pub mod fonts;
pub mod paths;
pub mod repository;
#[allow(clippy::module_inception)]
pub mod settings;
pub mod types;
pub mod update_color_files;
pub mod update_font_files;
pub mod update_settings;

pub use settings::Settings;
