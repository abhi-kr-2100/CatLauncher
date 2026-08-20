use std::io::Write;
use zip::write::SimpleFileOptions;

/// Creates an in-memory zip archive containing the given (filename, content) pairs.
pub fn create_test_zip(
  files: &[(&str, &[u8])],
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
  let mut buf = Vec::new();
  {
    let mut zip = zip::ZipWriter::new(std::io::Cursor::new(&mut buf));
    for (name, content) in files {
      zip.start_file(*name, SimpleFileOptions::default())?;
      zip.write_all(content)?;
    }
    zip.finish()?;
  }
  Ok(buf)
}
