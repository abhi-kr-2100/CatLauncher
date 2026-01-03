use std::path::PathBuf;

use crate::infra::utils::OS;

pub fn get_font_directories(os: &OS) -> Vec<PathBuf> {
  match os {
    OS::Linux => vec![
      PathBuf::from("/usr/share/fonts"),
      PathBuf::from("/usr/local/share/fonts"),
      dirs::home_dir()
        .map(|h| h.join(".fonts"))
        .unwrap_or_default(),
      dirs::home_dir()
        .map(|h| h.join(".local/share/fonts"))
        .unwrap_or_default(),
    ],
    OS::Windows => vec![
      PathBuf::from("C:\\Windows\\Fonts"),
      dirs::home_dir()
        .map(|h| h.join("AppData\\Local\\Microsoft\\Windows\\Fonts"))
        .unwrap_or_default(),
    ],
    OS::Mac => vec![
      PathBuf::from("/System/Library/Fonts"),
      PathBuf::from("/Library/Fonts"),
      dirs::home_dir()
        .map(|h| h.join("Library/Fonts"))
        .unwrap_or_default(),
    ],
  }
}
