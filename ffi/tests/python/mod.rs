use std::path::PathBuf;
use std::process::Command;

use crate::common::{
    lib_path_env_var, profile_dir, target_dir, workspace_root, TestError,
};

fn which_python() -> Option<String> {
    for candidate in ["python3", "python"] {
        if let Ok(output) = Command::new(candidate).arg("--version").output() {
            if output.status.success() {
                return Some(candidate.to_string());
            }
        }
    }
    None
}

fn which_pytest() -> Option<String> {
    if let Ok(output) = Command::new("pytest").arg("--version").output() {
        if output.status.success() {
            return Some("pytest".to_string());
        }
    }
    None
}

fn python_lib_extension() -> &'static str {
    if cfg!(target_os = "windows") {
        "dll"
    } else if cfg!(target_os = "macos") {
        "dylib"
    } else {
        "so"
    }
}

fn python_module_extension() -> &'static str {
    if cfg!(target_os = "windows") {
        "pyd"
    } else {
        "so"
    }
}

fn python_output_name() -> String {
    let prefix = if cfg!(target_os = "windows") { "" } else { "lib" };
    format!("{prefix}github_mock_api_python.{}", python_lib_extension())
}

fn python_lib_path() -> Result<PathBuf, TestError> {
    Ok(profile_dir()?.join(python_output_name()))
}

fn build_python_cdylib() -> Result<(), TestError> {
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
        .arg("github-mock-api-python")
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
fn python_mock_server_tests() -> Result<(), TestError> {
    build_python_cdylib()?;

    let root = workspace_root()?;
    let test_dir = root.join("ffi/tests/python");

    let lib_path = python_lib_path()?;
    let module_path = test_dir.join(format!("github_mock_api.{}", python_module_extension()));
    std::fs::copy(&lib_path, &module_path).map_err(|_| TestError::CopyModule)?;

    let mut cmd = if let Some(pytest) = which_pytest() {
        let mut c = Command::new(pytest);
        c.arg("test_mock_server.py");
        c
    } else {
        let python = which_python().ok_or(TestError::NoPythonRuntime)?;
        let mut c = Command::new(python);
        c.arg("-m").arg("pytest").arg("test_mock_server.py");
        c
    };
    cmd.current_dir(&test_dir);
    cmd.env(lib_path_env_var(), profile_dir()?);

    let status = cmd.status().map_err(|_| TestError::RunPythonTest)?;
    assert!(status.success(), "Python API tests exited with failure");

    Ok(())
}
