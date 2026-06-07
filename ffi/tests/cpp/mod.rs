use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::common::{TestError, build_cdylib, lib_dirs, lib_name, target_dir, workspace_root};

fn which_cxx() -> Option<String> {
    for candidate in ["c++"] {
        if let Ok(output) = Command::new(candidate).arg("--version").output()
            && output.status.success()
        {
            return Some(candidate.to_string());
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

fn compile(compiler: &str, source: &Path, binary: &Path) -> Result<Command, TestError> {
    let include = workspace_root()?.join("bindings").join("cpp");
    let lib_dirs = lib_dirs()?;

    let mut cmd = Command::new(compiler);
    cmd.arg("-std=c++17")
        .arg("-Wall")
        .arg("-Wextra")
        .arg("-I")
        .arg(include)
        .arg(source);
    for arg in lib_dir_args(&lib_dirs) {
        cmd.arg(arg);
    }
    cmd.arg("-l").arg(lib_name()).arg("-o").arg(binary);
    if cfg!(target_os = "linux") {
        cmd.arg("-pthread");
    }
    Ok(cmd)
}

#[test]
fn cpp_mock_server_tests() -> Result<(), TestError> {
    build_cdylib()?;

    let out_dir = target_dir()?.join("cpp-api-tests");
    create_dir_all(&out_dir).map_err(|_| TestError::CreateOutputDir)?;

    let cxx = which_cxx().ok_or(TestError::NoCxxCompiler)?;
    let source = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/cpp/mock_server_tests.cpp");
    let binary = out_dir.join("mock_server_tests");

    let mut cmd = compile(&cxx, &source, &binary)?;
    let compile_status = cmd.status().map_err(|_| TestError::RunCxxCompiler)?;
    assert!(compile_status.success(), "failed to compile C++ API tests");

    let run_status = Command::new(&binary)
        .status()
        .map_err(|_| TestError::RunSmokeTest)?;
    assert!(run_status.success(), "C++ API tests exited with failure");

    Ok(())
}
