use std::path::{PathBuf, Path};

pub struct TmpDir(PathBuf);

impl TmpDir {
    pub fn new() -> std::io::Result<Self> {
        let mut tmp = std::env::temp_dir();
        tmp.push(format!("mc-map-tools-{}", std::process::id()));
        std::fs::create_dir_all(&tmp)?;
        Ok(Self(tmp))
    }
}

impl AsRef<Path> for TmpDir {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl Drop for TmpDir {
    fn drop(&mut self) {
        if let Err(err) = std::fs::remove_dir_all(&self) {
            log::error!("Could not delete temporary directory \"{}\": {err}", self.as_ref().display())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::TmpDir;

    #[test]
    fn create_and_remove_tmp_dir() {
        let tmp = TmpDir::new().expect("Error creating folder");
        let path: &Path = AsRef::as_ref(&tmp);
        let path = path.to_owned();
        assert!(path.exists());
        drop(tmp);
        assert!(!path.exists());
    }
}

