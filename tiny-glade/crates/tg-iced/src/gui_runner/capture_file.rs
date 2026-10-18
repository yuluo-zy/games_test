//! Raw screenshot persistence. Sidecars append an extension so they never replace pixels.
use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
};

/// Create pixels and a `<path>.json` sidecar without replacing existing files.
/// The image must have nonzero dimensions and exactly `width * height * 4` RGBA8
/// bytes. `path` names the raw pixel file; this does not encode a PNG.
///
/// # Errors
/// Returns `InvalidInput` for malformed images, `AlreadyExists` if either output
/// exists, or the underlying I/O error. Failure or unwind attempts to remove only
/// files created by this call; rollback cannot guarantee recovery from filesystem
/// errors, process termination, or concurrent changes to the outputs.
#[cfg(feature = "native-gui")]
pub fn write_native_capture(
    image: &iced::window::Screenshot,
    path: impl AsRef<Path>,
) -> io::Result<()> {
    write_rgba_capture(
        &image.bytes,
        image.size.width,
        image.size.height,
        path.as_ref(),
    )
}

fn write_rgba_capture(bytes: &[u8], width: u32, height: u32, path: &Path) -> io::Result<()> {
    let expected = u64::from(width)
        .checked_mul(u64::from(height))
        .and_then(|pixels| pixels.checked_mul(4));
    if width == 0 || height == 0 || expected != Some(bytes.len() as u64) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "RGBA dimensions must be nonzero and match the byte count",
        ));
    }
    let mut metadata_path = path.as_os_str().to_os_string();
    metadata_path.push(".json");
    let mut pixels = NewCaptureFile::create(path)?;
    let mut metadata = NewCaptureFile::create(Path::new(&metadata_path))?;
    pixels.write_all(bytes)?;
    metadata.write_all(
        format!(
            "{{\"width\":{width},\"height\":{height},\"bytes\":{},\"format\":\"RGBA8 sRGB\"}}",
            bytes.len()
        )
        .as_bytes(),
    )?;
    pixels.keep = true;
    metadata.keep = true;
    Ok(())
}

/// Own only a newly created file, and close it before rollback (including on Windows).
struct NewCaptureFile {
    path: PathBuf,
    file: Option<File>,
    keep: bool,
}
impl NewCaptureFile {
    fn create(path: &Path) -> io::Result<Self> {
        let file = OpenOptions::new().write(true).create_new(true).open(path)?;
        Ok(Self {
            path: path.to_owned(),
            file: Some(file),
            keep: false,
        })
    }
    fn write_all(&mut self, bytes: &[u8]) -> io::Result<()> {
        match self.file.as_mut() {
            Some(file) => file.write_all(bytes),
            None => Err(io::Error::other("capture file already closed")),
        }
    }
}
impl Drop for NewCaptureFile {
    fn drop(&mut self) {
        drop(self.file.take());
        if !self.keep {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn capture_output_validates_rolls_back_and_preserves_existing_files() {
        let directory =
            std::env::temp_dir().join(format!("tg-iced-capture-test-{}", std::process::id()));
        std::fs::create_dir_all(&directory).unwrap();
        let path = directory.join("capture.json");
        let metadata = directory.join("capture.json.json");
        let pixels = [1, 2, 3, 255];
        for (bytes, width, height) in [
            (&[][..], 0, 1),
            (&[][..], 1, 0),
            (&[][..], 1, 1),
            (&[][..], u32::MAX, u32::MAX),
        ] {
            assert_eq!(
                write_rgba_capture(bytes, width, height, &path)
                    .unwrap_err()
                    .kind(),
                io::ErrorKind::InvalidInput
            );
            assert!(!path.exists() && !metadata.exists());
        }

        // An existing sidecar must survive, and the new pixel file must roll back.
        std::fs::write(&metadata, b"old metadata").unwrap();
        assert_eq!(
            write_rgba_capture(&pixels, 1, 1, &path).unwrap_err().kind(),
            io::ErrorKind::AlreadyExists
        );
        assert!(!path.exists());
        assert_eq!(std::fs::read(&metadata).unwrap(), b"old metadata");
        std::fs::remove_file(&metadata).unwrap();

        // An existing pixel file must survive, without creating a sidecar.
        std::fs::write(&path, b"old pixels").unwrap();
        assert_eq!(
            write_rgba_capture(&pixels, 1, 1, &path).unwrap_err().kind(),
            io::ErrorKind::AlreadyExists
        );
        assert_eq!(std::fs::read(&path).unwrap(), b"old pixels");
        assert!(!metadata.exists());
        std::fs::remove_file(&path).unwrap();

        write_rgba_capture(&pixels, 1, 1, &path).unwrap();
        assert_eq!(std::fs::read(&path).unwrap(), pixels);
        let original_metadata = std::fs::read(&metadata).unwrap();
        assert!(String::from_utf8_lossy(&original_metadata).contains("\"width\":1"));
        assert_eq!(
            write_rgba_capture(&[9, 9, 9, 9], 1, 1, &path)
                .unwrap_err()
                .kind(),
            io::ErrorKind::AlreadyExists
        );
        assert_eq!(std::fs::read(&path).unwrap(), pixels);
        assert_eq!(std::fs::read(&metadata).unwrap(), original_metadata);
        std::fs::remove_dir_all(directory).unwrap();
    }
}
