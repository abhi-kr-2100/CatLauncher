use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::common::{build_cdylib, lib_dirs, lib_name, target_dir, workspace_root, TestError};

fn which_cc() -> Option<String> {
    for candidate in ["cc"] {
        if let Ok(output) = Command::new(candidate).arg("--version").output() {
            if output.status.success() {
                return Some(candidate.to_string());
            }
        }
    }
    None
}

fn lib_dir_args(lib_dirs: &[PathBuf]) -> Vec<String> {
    let mut args = Vec::new();
    for lib_dir in lib_dirs {
        args.push("-L".to_string());
        args.push(lib_dir.display().to_string());
        let rpath = lib_dir.display().to_string();
        if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
            args.push(format!("-Wl,-rpath,{rpath}"));
        }
    }
    args
}

fn pkg_config(args: &[&str]) -> Result<Vec<String>, TestError> {
    let output = Command::new("pkg-config")
        .args(args)
        .output()
        .map_err(|_| TestError::NoPkgConfig)?;
    if !output.status.success() {
        return Err(TestError::NoPkgConfig);
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout
        .split_whitespace()
        .map(|s| s.to_string())
        .collect())
}

fn compile(
    compiler: &str,
    source: &Path,
    binary: &Path,
) -> Result<Command, TestError> {
    let include = workspace_root()?.join("bindings").join("c");
    let lib_dirs = lib_dirs()?;

    let mut cmd = Command::new(compiler);
    cmd.arg("-std=c11")
        .arg("-Wall")
        .arg("-Wextra")
        .arg("-I")
        .arg(&include)
        .arg(source);
    for arg in lib_dir_args(&lib_dirs) {
        cmd.arg(arg);
    }
    for arg in pkg_config(&["--cflags", "check"])? {
        cmd.arg(arg);
    }
    cmd.arg("-l")
        .arg(lib_name());
    for arg in pkg_config(&["--libs", "check"])? {
        cmd.arg(arg);
    }
    cmd.arg("-o")
        .arg(binary);
    if cfg!(target_os = "linux") {
        cmd.arg("-pthread");
    }
    Ok(cmd)
}

#[test]
fn c_mock_server_check_test() -> Result<(), TestError> {
    build_cdylib()?;
    let out_dir = target_dir()?.join("c-api-tests");
    create_dir_all(&out_dir).map_err(|_| TestError::CreateOutputDir)?;

    let cc = which_cc().ok_or(TestError::NoCCompiler)?;
    let source = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/c/mock_server_test.c");
    let binary = out_dir.join("mock_server_test");

    let mut cmd = compile(&cc, &source, &binary)?;
    let compile_status = cmd.status().map_err(|_| TestError::RunCCompiler)?;
    assert!(
        compile_status.success(),
        "failed to compile C unit tests with check"
    );

    let run_status = Command::new(&binary)
        .status()
        .map_err(|_| TestError::RunCTest)?;
    assert!(
        run_status.success(),
        "C unit tests (check) exited with failure"
    );

    Ok(())
}
