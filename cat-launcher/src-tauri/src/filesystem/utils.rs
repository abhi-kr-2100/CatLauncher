use std::io;
use std::path::Path;

use tokio::fs::{create_dir_all, read_dir};
use tokio::process::Command;

use crate::infra::utils::OS;

pub fn get_safe_filename(name: &str) -> String {
  name
    .chars()
    .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
    .collect()
}

#[derive(thiserror::Error, Debug)]
pub enum CopyDirError {
  #[error("IO error: {0}")]
  Io(#[from] io::Error),
}

pub async fn copy_dir_all(
  src: &Path,
  dst: &Path,
  os: &OS,
) -> Result<(), CopyDirError> {
  create_dir_all(&dst).await?;

  if os == &OS::Mac {
    let status =
      Command::new("ditto").arg(src).arg(dst).status().await?;
    if !status.success() {
      return Err(CopyDirError::Io(io::Error::other(
        "ditto command failed",
      )));
    }

    return Ok(());
  }

  let mut entries = read_dir(src).await?;
  while let Some(entry) = entries.next_entry().await? {
    let ty = entry.file_type().await?;
    if ty.is_dir() {
      Box::pin(copy_dir_all(
        &entry.path(),
        &dst.join(entry.file_name()),
        os,
      ))
      .await?;
    } else {
      tokio::fs::copy(&entry.path(), &dst.join(entry.file_name()))
        .await?;
    }
  }

  Ok(())
}

#[cfg(test)]
#[allow(
  clippy::panic_in_result_fn,
  clippy::indexing_slicing,
  clippy::expect_used,
  clippy::io_other_error,
  clippy::unwrap_used
)]
mod tests {
  use super::*;
  use tempfile::tempdir;

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  #[test]
  fn test_get_safe_filename_alphanumeric() {
    assert_eq!(get_safe_filename("abcXYZ123"), "abcXYZ123");
    assert_eq!(get_safe_filename("Release2026"), "Release2026");
  }

  #[test]
  fn test_get_safe_filename_replaces_unsafe_chars() {
    assert_eq!(get_safe_filename("foo bar"), "foo_bar");
    assert_eq!(
      get_safe_filename("path/to/file:name*test?"),
      "path_to_file_name_test_"
    );
    assert_eq!(get_safe_filename("v0.H-2026"), "v0_H_2026");
    assert_eq!(get_safe_filename("<>|:\"\\/"), "_______");
  }

  #[test]
  fn test_get_safe_filename_empty() {
    assert_eq!(get_safe_filename(""), "");
  }

  #[test]
  fn test_get_safe_filename_unicode() {
    assert_eq!(get_safe_filename("café"), "caf_");
    assert_eq!(get_safe_filename("日本語_test"), "____test");
    assert_eq!(get_safe_filename("🚀rock!"), "_rock_");
  }

  #[test]
  fn test_copy_dir_error_display_and_conversion() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "not found");
    let copy_err = CopyDirError::from(io_err);
    assert_eq!(copy_err.to_string(), "IO error: not found");
    assert!(format!("{:?}", copy_err).contains("NotFound"));

    let custom_err =
      CopyDirError::Io(io::Error::other("ditto command failed"));
    assert_eq!(
      custom_err.to_string(),
      "IO error: ditto command failed"
    );
  }

  #[tokio::test]
  async fn test_copy_dir_all_empty_directory() -> TestResult {
    let temp = tempdir()?;
    let src = temp.path().join("src");
    let dst = temp.path().join("dst");

    tokio::fs::create_dir(&src).await?;

    copy_dir_all(&src, &dst, &OS::Linux).await?;

    assert!(dst.is_dir());
    let mut entries = tokio::fs::read_dir(&dst).await?;
    assert!(entries.next_entry().await?.is_none());

    Ok(())
  }

  #[tokio::test]
  async fn test_copy_dir_all_flat_files() -> TestResult {
    let temp = tempdir()?;
    let src = temp.path().join("src");
    let dst = temp.path().join("dst");

    tokio::fs::create_dir(&src).await?;
    tokio::fs::write(src.join("hello.txt"), b"Hello World").await?;
    tokio::fs::write(src.join("data.bin"), &[0, 1, 2, 3]).await?;

    copy_dir_all(&src, &dst, &OS::Linux).await?;

    assert_eq!(
      tokio::fs::read(dst.join("hello.txt")).await?,
      b"Hello World"
    );
    assert_eq!(
      tokio::fs::read(dst.join("data.bin")).await?,
      vec![0, 1, 2, 3]
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_copy_dir_all_nested_hierarchy() -> TestResult {
    let temp = tempdir()?;
    let src = temp.path().join("src");
    let dst = temp.path().join("dst");

    tokio::fs::create_dir_all(src.join("sub1").join("sub2")).await?;
    tokio::fs::create_dir(src.join("sub1").join("empty_child"))
      .await?;
    tokio::fs::create_dir(src.join("empty_dir")).await?;

    tokio::fs::write(src.join("root.txt"), b"root content").await?;
    tokio::fs::write(src.join("sub1").join("file1.txt"), b"child 1")
      .await?;
    tokio::fs::write(
      src.join("sub1").join("sub2").join("file2.txt"),
      b"child 2",
    )
    .await?;

    copy_dir_all(&src, &dst, &OS::Linux).await?;

    assert_eq!(
      tokio::fs::read(dst.join("root.txt")).await?,
      b"root content"
    );
    assert_eq!(
      tokio::fs::read(dst.join("sub1").join("file1.txt")).await?,
      b"child 1"
    );
    assert_eq!(
      tokio::fs::read(
        dst.join("sub1").join("sub2").join("file2.txt")
      )
      .await?,
      b"child 2"
    );
    assert!(dst.join("sub1").join("empty_child").is_dir());
    assert!(dst.join("empty_dir").is_dir());

    Ok(())
  }

  #[tokio::test]
  async fn test_copy_dir_all_into_existing_destination() -> TestResult
  {
    let temp = tempdir()?;
    let src = temp.path().join("src");
    let dst = temp.path().join("dst");

    tokio::fs::create_dir(&src).await?;
    tokio::fs::write(src.join("new_file.txt"), b"new").await?;
    tokio::fs::write(src.join("overwrite.txt"), b"overwritten")
      .await?;

    tokio::fs::create_dir(&dst).await?;
    tokio::fs::write(dst.join("existing.txt"), b"existing").await?;
    tokio::fs::write(dst.join("overwrite.txt"), b"initial").await?;

    copy_dir_all(&src, &dst, &OS::Linux).await?;

    assert_eq!(
      tokio::fs::read(dst.join("existing.txt")).await?,
      b"existing"
    );
    assert_eq!(
      tokio::fs::read(dst.join("new_file.txt")).await?,
      b"new"
    );
    assert_eq!(
      tokio::fs::read(dst.join("overwrite.txt")).await?,
      b"overwritten"
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_copy_dir_all_windows_os() -> TestResult {
    let temp = tempdir()?;
    let src = temp.path().join("src");
    let dst = temp.path().join("dst");

    tokio::fs::create_dir_all(src.join("nested")).await?;
    tokio::fs::write(src.join("nested").join("windows.txt"), b"win")
      .await?;

    copy_dir_all(&src, &dst, &OS::Windows).await?;

    assert_eq!(
      tokio::fs::read(dst.join("nested").join("windows.txt")).await?,
      b"win"
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_copy_dir_all_nonexistent_source() -> TestResult {
    let temp = tempdir()?;
    let src = temp.path().join("nonexistent_src");
    let dst = temp.path().join("dst");

    let result = copy_dir_all(&src, &dst, &OS::Linux).await;

    assert!(matches!(
      result,
      Err(CopyDirError::Io(err)) if err.kind() == io::ErrorKind::NotFound
    ));

    Ok(())
  }

  #[tokio::test]
  async fn test_copy_dir_all_source_is_file() -> TestResult {
    let temp = tempdir()?;
    let src = temp.path().join("src_file.txt");
    let dst = temp.path().join("dst");

    tokio::fs::write(&src, b"not a dir").await?;

    let result = copy_dir_all(&src, &dst, &OS::Linux).await;

    assert!(matches!(result, Err(CopyDirError::Io(_))));

    Ok(())
  }

  #[tokio::test]
  async fn test_copy_dir_all_destination_parent_is_file() -> TestResult
  {
    let temp = tempdir()?;
    let src = temp.path().join("src");
    let blocking_file = temp.path().join("file");
    let dst = blocking_file.join("dst");

    tokio::fs::create_dir(&src).await?;
    tokio::fs::write(&blocking_file, b"blocks dir creation").await?;

    let result = copy_dir_all(&src, &dst, &OS::Linux).await;

    assert!(matches!(result, Err(CopyDirError::Io(_))));

    Ok(())
  }

  #[tokio::test]
  async fn test_copy_dir_all_destination_conflict_with_nested_file()
  -> TestResult {
    let temp = tempdir()?;
    let src = temp.path().join("src");
    let dst = temp.path().join("dst");

    tokio::fs::create_dir_all(src.join("conflict")).await?;
    tokio::fs::write(src.join("conflict").join("file.txt"), b"data")
      .await?;

    tokio::fs::create_dir(&dst).await?;
    tokio::fs::write(dst.join("conflict"), b"blocks subfolder")
      .await?;

    let result = copy_dir_all(&src, &dst, &OS::Linux).await;

    assert!(matches!(result, Err(CopyDirError::Io(_))));

    Ok(())
  }

  #[tokio::test]
  async fn test_copy_dir_all_mac_os() -> TestResult {
    let temp = tempdir()?;
    let src = temp.path().join("src");
    let dst = temp.path().join("dst");

    tokio::fs::create_dir(&src).await?;
    tokio::fs::write(src.join("mac.txt"), b"mac content").await?;

    let result = copy_dir_all(&src, &dst, &OS::Mac).await;

    #[cfg(target_os = "macos")]
    {
      result?;
      assert_eq!(
        tokio::fs::read(dst.join("mac.txt")).await?,
        b"mac content"
      );
    }

    #[cfg(not(target_os = "macos"))]
    {
      match result {
        Ok(()) => {
          assert_eq!(
            tokio::fs::read(dst.join("mac.txt")).await?,
            b"mac content"
          );
        }
        Err(CopyDirError::Io(err)) => {
          assert_eq!(err.kind(), io::ErrorKind::NotFound);
        }
      }
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_copy_dir_all_mac_nonexistent_src() -> TestResult {
    let temp = tempdir()?;
    let src = temp.path().join("nonexistent");
    let dst = temp.path().join("dst");

    let result = copy_dir_all(&src, &dst, &OS::Mac).await;

    #[cfg(target_os = "macos")]
    {
      assert!(matches!(
        result,
        Err(CopyDirError::Io(err)) if err.to_string() == "ditto command failed"
      ));
    }

    #[cfg(not(target_os = "macos"))]
    {
      assert!(matches!(result, Err(CopyDirError::Io(_))));
    }

    Ok(())
  }
}
