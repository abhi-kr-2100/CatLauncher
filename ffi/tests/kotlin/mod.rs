use std::path::{Path, PathBuf};
use std::process::Command;

use crate::common::{
    TestError, build_cdylib, lib_dirs, lib_path_env_var, profile_dir, target_dir, workspace_root,
};

fn which_kotlinc() -> Option<String> {
    for candidate in ["kotlinc"] {
        if let Ok(output) = Command::new(candidate).arg("-version").output()
            && output.status.success()
        {
            return Some(candidate.to_string());
        }
    }
    None
}

fn env_lib_path(lib_dirs: &[PathBuf]) -> Result<String, TestError> {
    std::env::join_paths(lib_dirs.iter())
        .map(|s| s.to_string_lossy().into_owned())
        .map_err(|_| TestError::JoinClasspath)
}

fn find_jna_jar() -> Option<PathBuf> {
    let path = std::env::var("JNA_JAR").ok()?;
    let p = PathBuf::from(path);
    if p.is_file() { Some(p) } else { None }
}

fn join_classpath(paths: &[&Path]) -> Result<String, TestError> {
    std::env::join_paths(paths.iter().copied())
        .map(|s| s.to_string_lossy().into_owned())
        .map_err(|_| TestError::JoinClasspath)
}

#[test]
fn kotlin_mock_server_smoke_test() -> Result<(), TestError> {
    build_cdylib()?;
    let kotlinc = which_kotlinc().ok_or(TestError::NoKotlinc)?;
    let jna_jar = find_jna_jar().ok_or(TestError::NoJnaJar)?;
    let lib_dirs = lib_dirs()?;
    let out_dir = target_dir()?.join("kotlin-api-tests");
    std::fs::create_dir_all(&out_dir).map_err(|_| TestError::CreateKotlinOutputDir)?;

    let source = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/kotlin/MockServerTest.kt");
    let jar = out_dir.join("MockServerTest.jar");

    let bindings_dir = workspace_root()?.join("bindings/kotlin/src/main/kotlin");
    let compile_cp = join_classpath(&[&jna_jar, &bindings_dir])?;
    // Compile both the test and all generated Kotlin sources under bindings_dir
    let compile_status = Command::new(&kotlinc)
        .arg("-cp")
        .arg(&compile_cp)
        .arg(&source)
        .arg(&bindings_dir)
        .arg("-include-runtime")
        .arg("-d")
        .arg(&jar)
        .status()
        .map_err(|_| TestError::RunKotlinc)?;
    assert!(
        compile_status.success(),
        "failed to compile Kotlin API smoke test"
    );

    let lib_path = env_lib_path(&lib_dirs)?;
    let classpath = join_classpath(&[&jar, &bindings_dir, &jna_jar])?;
    let profile = profile_dir()?;
    let run_status = Command::new("java")
        .arg(format!("-Djava.library.path={}", profile.display()))
        .arg(format!("-Djna.library.path={}", profile.display()))
        .arg("-cp")
        .arg(&classpath)
        .arg("MockServerTestKt")
        .env(lib_path_env_var(), &lib_path)
        .status()
        .map_err(|_| TestError::RunKotlinTest)?;
    assert!(
        run_status.success(),
        "Kotlin API smoke test exited with failure"
    );
    Ok(())
}
