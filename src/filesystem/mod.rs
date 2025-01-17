use std::fs::{create_dir_all, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

pub struct FileSystem {
    base_path: PathBuf,
    user_data_path: PathBuf,
}

impl FileSystem {
    pub fn new() -> Self {
        let base_path = Self::get_base_path();
        let user_data_path = Self::get_user_data_path();

        // Ensure directories exist
        create_dir_all(&base_path).unwrap_or_default();
        create_dir_all(&user_data_path).unwrap_or_default();

        Self {
            base_path,
            user_data_path,
        }
    }

    fn get_base_path() -> PathBuf {
        #[cfg(target_os = "windows")]
        {
            if let Ok(exe_path) = std::env::current_exe() {
                return exe_path.parent().unwrap_or(Path::new("")).to_path_buf();
            }
        }

        #[cfg(target_os = "linux")]
        {
            if let Ok(xdg_data) = std::env::var("XDG_DATA_HOME") {
                return PathBuf::from(xdg_data).join("nyanko_engine");
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Some(resource_path) = Self::get_macos_resources_path() {
                return resource_path;
            }
        }

        PathBuf::from(".")
    }

    fn get_user_data_path() -> PathBuf {
        #[cfg(target_os = "windows")]
        {
            if let Ok(app_data) = std::env::var("APPDATA") {
                return PathBuf::from(app_data).join("NyankoEngine");
            }
        }

        #[cfg(target_os = "linux")]
        {
            if let Ok(home) = std::env::var("HOME") {
                return PathBuf::from(home).join(".local/share/nyanko_engine");
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(home) = std::env::var("HOME") {
                return PathBuf::from(home).join("Library/Application Support/NyankoEngine");
            }
        }

        PathBuf::from("user_data")
    }

    #[cfg(target_os = "macos")]
    fn get_macos_resources_path() -> Option<PathBuf> {
        std::env::current_exe()
            .ok()
            .and_then(|exe_path| exe_path.parent().map(|p| p.to_path_buf()))
            .map(|path| path.join("../Resources"))
    }

    pub fn read_file(&self, path: &Path) -> io::Result<Vec<u8>> {
        let full_path = self.base_path.join(path);
        let mut file = File::open(full_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    pub fn write_file(&self, path: &Path, data: &[u8]) -> io::Result<()> {
        let full_path = self.user_data_path.join(path);
        if let Some(parent) = full_path.parent() {
            create_dir_all(parent)?;
        }
        let mut file = File::create(full_path)?;
        file.write_all(data)
    }
}
