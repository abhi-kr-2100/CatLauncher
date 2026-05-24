use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
enum BuildError {
    #[error("CARGO_MANIFEST_DIR not set")]
    MissingManifestDir(#[from] std::env::VarError),

    #[error("ffi crate has no parent directory")]
    NoParentDir,

    #[error("failed to load config.toml: {0}")]
    ConfigLoad(String),

    #[error("failed to generate bindings with diplomat: {0}")]
    GenFailed(#[from] std::io::Error),
}

struct BindingTarget<'a> {
    language: &'a str,
    out_folder: PathBuf,
}

fn main() -> Result<(), BuildError> {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let project_root = manifest_dir.parent().ok_or(BuildError::NoParentDir)?;
    let entry = manifest_dir.join("src/lib.rs");
    let config_file = project_root.join("config.toml");
    let bindings_folder = project_root.join("bindings");

    let mut config = diplomat_tool::config::Config::default();
    if config_file.exists() {
        config.read_file(&config_file).map_err(BuildError::ConfigLoad)?;
    }

    let docs = diplomat_tool::DocsUrlGenerator::with_base_urls(None, HashMap::new());
    let targets = [
        BindingTarget {
            language: "c",
            out_folder: bindings_folder.join("c").join("github_mock_api"),
        },
        BindingTarget {
            language: "cpp",
            out_folder: bindings_folder.join("cpp").join("github_mock_api"),
        },
        BindingTarget {
            language: "dart",
            out_folder: bindings_folder.join("dart"),
        },
        BindingTarget {
            language: "kotlin",
            out_folder: bindings_folder.join("kotlin"),
        },
    ];

    for target in targets {
        diplomat_tool::r#gen(
            &entry,
            target.language,
            &target.out_folder,
            &docs,
            config.clone(),
            false,
        )
        .map_err(BuildError::GenFailed)?;
    }

    println!("cargo:rerun-if-changed={}", entry.display());
    println!("cargo:rerun-if-changed={}", config_file.display());

    Ok(())
}
