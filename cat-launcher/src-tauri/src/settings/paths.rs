use std::path::PathBuf;

use crate::infra::utils::OS;

pub fn get_font_directories(os: &OS) -> Vec<PathBuf> {
  let mut paths = Vec::new();
  let home = std::env::var("HOME")
    .or_else(|_| std::env::var("USERPROFILE"))
    .ok();

  match os {
    OS::Linux => {
      if let Some(h) = &home {
        paths.push(PathBuf::from(h).join(".local/share/fonts"));
        paths.push(PathBuf::from(h).join(".fonts"));
      }
      paths.push(PathBuf::from("/usr/share/fonts"));
      paths.push(PathBuf::from("/usr/local/share/fonts"));
    }
    OS::Mac => {
      if let Some(h) = &home {
        paths.push(PathBuf::from(h).join("Library/Fonts"));
      }
      paths.push(PathBuf::from("/Library/Fonts"));
      paths.push(PathBuf::from("/System/Library/Fonts"));
    }
    OS::Windows => {
      paths.push(PathBuf::from("C:\\Windows\\Fonts"));
      if let Some(h) = &home {
        paths.push(
          PathBuf::from(h)
            .join("AppData\\Local\\Microsoft\\Windows\\Fonts"),
        );
      }
    }
  }
  paths
}
