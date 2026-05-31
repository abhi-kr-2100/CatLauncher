use std::path::{Path, PathBuf};
use std::process::Command;

use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum TestError {
    #[error("ffi crate lives in the workspace root")]
    WorkspaceRoot,

    #[error("C compiler required for C API tests (cc or gcc)")]
    NoCCompiler,

    #[error("C++ compiler required for C++ API tests (c++, g++, or clang++)")]
    NoCxxCompiler,

    #[error("create c-api-tests output dir")]
    CreateOutputDir,

    #[error("run C compiler")]
    RunCCompiler,

    #[error("run C++ compiler")]
    RunCxxCompiler,

    #[error("pkg-config required to find check unit test framework")]
    NoPkgConfig,

    #[error("run C unit test binary")]
    RunCTest,

    #[error("run C API smoke test binary")]
    RunSmokeTest,

    #[error("Dart runtime required for Dart API tests (dart)")]
    NoDartRuntime,

    #[error("Kotlin compiler required for Kotlin API tests (kotlinc)")]
    NoKotlinc,

    #[error("JNA jar not found (set JNA_JAR env var)")]
    NoJnaJar,

    #[error("create kotlin-api-tests output dir")]
    CreateKotlinOutputDir,

    #[error("run Kotlin compiler")]
    RunKotlinc,

    #[error("run Kotlin API smoke test")]
    RunKotlinTest,

    #[error("run Dart API smoke test")]
    RunDartTest,

    #[error("Python runtime required for Python API tests (python3)")]
    NoPythonRuntime,

    #[error("run Python API tests")]
    RunPythonTest,

    #[error("copy Python extension module")]
    CopyModule,

    #[error("join classpath components")]
    JoinClasspath,

    #[error("Ruby runtime required for Ruby API tests (rspec)")]
    NoRubyRuntime,

    #[error("run Ruby API unit tests")]
    RunRubyTest,

    #[error("CARGO environment variable not set")]
    CargoEnvMissing,

    #[error("spawn cargo build for cdylib")]
    SpawnCargoBuild,

    #[error("cargo build for cdylib failed")]
    CargoBuildFailed,
}

pub(crate) fn workspace_root() -> Result<PathBuf, TestError> {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or(TestError::WorkspaceRoot)
        .map(|p| p.to_path_buf())
}

pub(crate) fn target_dir() -> Result<PathBuf, TestError> {
    Ok(workspace_root()?.join("target"))
}

pub(crate) fn profile_dir() -> Result<PathBuf, TestError> {
    let profile = if cfg!(debug_assertions) { "debug" } else { "release" };
    Ok(target_dir()?.join(profile))
}

pub(crate) fn lib_dirs() -> Result<[PathBuf; 2], TestError> {
    let profile = profile_dir()?;
    Ok([profile.clone(), profile.join("deps")])
}

pub(crate) fn lib_name() -> &'static str {
    "github_mock_api_ffi"
}

pub(crate) fn build_cdylib() -> Result<(), TestError> {
    let target = target_dir()?;
    let cargo = std::env::var("CARGO").map_err(|_| TestError::CargoEnvMissing)?;
    let profile = if cfg!(debug_assertions) { "debug" } else { "release" };
    let mut cmd = Command::new(cargo);
    cmd.arg("build")
        .arg("-p")
        .arg("github-mock-api-ffi")
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

pub(crate) fn mock_api_lib_path() -> Result<PathBuf, TestError> {
    let ext = if cfg!(target_os = "windows") {
        "dll"
    } else if cfg!(target_os = "macos") {
        "dylib"
    } else {
        "so"
    };
    let prefix = if cfg!(target_os = "windows") { "" } else { "lib" };
    Ok(profile_dir()?.join(format!("{prefix}{}.{ext}", lib_name())))
}

pub(crate) fn preload_env_vars(lib_path: &Path) -> Vec<(&'static str, String)> {
    if cfg!(target_os = "windows") {
        let profile = lib_path.parent().expect("lib_path should have a parent");
        let path = std::env::var("PATH").unwrap_or_default();
        vec![("PATH", format!("{};{}", profile.display(), path))]
    } else {
        let preload_var = if cfg!(target_os = "macos") {
            "DYLD_INSERT_LIBRARIES"
        } else {
            "LD_PRELOAD"
        };
        vec![(preload_var, lib_path.display().to_string())]
    }
}

pub(crate) fn lib_path_env_var() -> &'static str {
    if cfg!(target_os = "macos") {
        "DYLD_LIBRARY_PATH"
    } else if cfg!(target_os = "windows") {
        "PATH"
    } else {
        "LD_LIBRARY_PATH"
    }
}
