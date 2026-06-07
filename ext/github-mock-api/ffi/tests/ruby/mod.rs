use std::path::PathBuf;
use std::process::Command;

use crate::common::{TestError, lib_path_env_var, profile_dir, target_dir, workspace_root};

fn which_rspec() -> Option<String> {
    if let Ok(output) = Command::new("rspec").arg("--version").output()
        && output.status.success()
    {
        return Some("rspec".to_string());
    }
    None
}

fn ruby_lib_extension() -> &'static str {
    if cfg!(target_os = "windows") {
        "dll"
    } else if cfg!(target_os = "macos") {
        "dylib"
    } else {
        "so"
    }
}

fn ruby_module_extension() -> &'static str {
    if cfg!(target_os = "windows") {
        "dll"
    } else if cfg!(target_os = "macos") {
        "bundle"
    } else {
        "so"
    }
}

fn ruby_output_name() -> String {
    let prefix = if cfg!(target_os = "windows") {
        ""
    } else {
        "lib"
    };
    format!("{prefix}github_mock_api_ruby.{}", ruby_lib_extension())
}

fn ruby_lib_path() -> Result<PathBuf, TestError> {
    Ok(profile_dir()?.join(ruby_output_name()))
}

fn build_ruby_cdylib() -> Result<(), TestError> {
    let target = target_dir()?;
    let cargo = std::env::var("CARGO").map_err(|_| TestError::CargoEnvMissing)?;
    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    let mut cmd = Command::new(cargo);
    cmd.arg("build")
        .arg("-p")
        .arg("github-mock-api-ruby")
        .arg("--lib")
        .arg("--target-dir")
        .arg(&target);
    if profile != "debug" {
        cmd.arg("--profile").arg(profile);
    }
    let status = cmd.status().map_err(|_| TestError::SpawnCargoBuild)?;
    if !status.success() {
        return Err(TestError::CargoBuildFailed);
    }
    Ok(())
}

#[test]
fn ruby_mock_server_spec() -> Result<(), TestError> {
    build_ruby_cdylib()?;

    let rspec = which_rspec().ok_or(TestError::NoRubyRuntime)?;
    let root = workspace_root()?;
    let test_dir = root.join("ffi/tests/ruby");

    let lib_path = ruby_lib_path()?;
    let module_path = test_dir.join(format!("github_mock_api_ruby.{}", ruby_module_extension()));
    std::fs::copy(&lib_path, &module_path).map_err(|_| TestError::CopyModule)?;

    let mut cmd = Command::new(&rspec);
    cmd.arg("--format")
        .arg("documentation")
        .current_dir(&test_dir);
    cmd.env(lib_path_env_var(), profile_dir()?);

    let status = cmd.status().map_err(|_| TestError::RunRubyTest)?;
    assert!(status.success(), "Ruby API RSpec tests exited with failure");

    Ok(())
}
