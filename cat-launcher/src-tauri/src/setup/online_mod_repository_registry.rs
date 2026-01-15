use tauri::{App, Manager};

use crate::mods::{
  lib::OnlineModRepositoryRegistry,
  online::bright_nights::BrightNightsModRepository,
};

pub fn manage_online_mod_repository_registry(app: &App) {
  let mut online_mod_repository_registry =
    OnlineModRepositoryRegistry::default();
  online_mod_repository_registry
    .register(Box::new(BrightNightsModRepository::new()));
  app.manage(online_mod_repository_registry);
}
