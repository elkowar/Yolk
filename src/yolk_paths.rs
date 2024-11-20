use std::path::Path;

use anyhow::Result;

const DEFAULT_RHAI: &str = indoc::indoc! {r#"
    fn canonical_data() {
        #{}
    }
    fn local_data(machine_name) {
        canonical_data()
    }
"#};

pub struct YolkPaths {
    /// Path to the yolk directory.
    root_path: std::path::PathBuf,
    home: std::path::PathBuf,
}

impl YolkPaths {
    pub fn new(path: std::path::PathBuf, home: std::path::PathBuf) -> Self {
        YolkPaths {
            root_path: path,
            home,
        }
    }

    pub fn testing() -> Self {
        let base_dir = std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR")).join("test_home");
        Self::new(base_dir.join("yolk"), base_dir)
    }

    pub fn from_env() -> Self {
        Self {
            root_path: dirs::config_dir()
                .expect("No config dir available")
                .join("yolk"),
            home: dirs::home_dir().expect("No config dir available"),
        }
    }

    pub fn check(&self) -> Result<()> {
        if !self.root_path().exists() {
            anyhow::bail!(
                "Yolk directory does not exist at {}",
                self.root_path().display()
            );
        }
        if !self.rhai_path().exists() {
            anyhow::bail!(
                "Yolk directory does not contain a .rhai file at {}",
                self.rhai_path().display()
            );
        }
        if !self.local_dir_path().exists() {
            anyhow::bail!(
                "Yolk directory does not contain a local directory at {}",
                self.local_dir_path().display()
            );
        }
        if !self.canonical_dir_path().exists() {
            anyhow::bail!(
                "Yolk directory does not contain a canonical directory at {}",
                self.canonical_dir_path().display()
            );
        }
        Ok(())
    }

    pub fn create(&self) -> Result<()> {
        let path = self.root_path();
        if path.exists() && path.is_dir() && fs_err::read_dir(path)?.next().is_some() {
            anyhow::bail!("Yolk directory already exists at {}", path.display());
        }
        fs_err::create_dir_all(&path)?;
        fs_err::create_dir_all(&self.local_dir_path())?;
        fs_err::create_dir_all(&self.canonical_dir_path())?;
        fs_err::write(self.root_path().join(".gitignore"), "/local")?;
        fs_err::write(self.rhai_path(), DEFAULT_RHAI)?;

        Ok(())
    }

    pub fn root_path(&self) -> &std::path::Path {
        &self.root_path
    }
    pub fn home_path(&self) -> &std::path::Path {
        &self.home
    }
    pub fn rhai_path(&self) -> std::path::PathBuf {
        self.root_path.join("yolk.rhai")
    }
    pub fn local_dir_path(&self) -> std::path::PathBuf {
        self.root_path.join("local")
    }
    pub fn canonical_dir_path(&self) -> std::path::PathBuf {
        self.root_path.join("canonical")
    }
    pub fn local_thing_path(&self, thing: &str) -> std::path::PathBuf {
        self.local_dir_path().join(thing)
    }
    pub fn canonical_thing_path(&self, thing: &str) -> std::path::PathBuf {
        self.canonical_dir_path().join(thing)
    }
}

#[cfg(test)]
mod test {

    use tempdir::TempDir;

    use super::YolkPaths;

    #[test]
    pub fn test_setup() {
        let tmp_root = TempDir::new("yolk-setup").unwrap();
        let tmp_home = TempDir::new("yolk-home").unwrap();
        let root = tmp_root.path().to_path_buf();
        let home = tmp_home.path().to_path_buf();

        let yolk_paths = YolkPaths::new(root.clone(), home.clone());
        assert!(yolk_paths.check().is_err());
        yolk_paths.create().unwrap();
        assert!(yolk_paths.check().is_ok());
    }
}