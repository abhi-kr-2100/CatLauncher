use tauri::{AppHandle, Emitter};
use tauri_plugin_updater::UpdaterExt;

use crate::infra::autoupdate::update_status::UpdateStatus;

pub async fn run_updater(handle: AppHandle) {
    // Nothing can be done with emit errors. We ignore them.
    let _ = handle.emit("autoupdate-status", &UpdateStatus::Checking);

    let updater = match handle.updater() {
        Ok(updater) => updater,
        Err(e) => {
            let _ = handle.emit(
                "autoupdate-status",
                &UpdateStatus::Failure {
                    error: e.to_string(),
                },
            );
            return;
        }
    };

    match updater.check().await {
        Err(e) => {
            let _ = handle.emit(
                "autoupdate-status",
                &UpdateStatus::Failure {
                    error: e.to_string(),
                },
            );
            return;
        }

        Ok(None) => {
            let _ = handle.emit("autoupdate-status", &UpdateStatus::UpToDate);
        }

        Ok(Some(update)) => {
            let _ = handle.emit("autoupdate-status", &UpdateStatus::Downloading);
            if let Err(e) = update
                .download_and_install(
                    |_, _| {},
                    || {
                        let _ = handle.emit("autoupdate-status", &UpdateStatus::Installing);
                    },
                )
                .await
            {
                let _ = handle.emit(
                    "autoupdate-status",
                    &UpdateStatus::Failure {
                        error: e.to_string(),
                    },
                );
            } else {
                let _ = handle.emit("autoupdate-status", &UpdateStatus::Success);
            }
        }
    }
}
