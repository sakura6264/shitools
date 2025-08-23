use crate::utils::*;
use std::path::{Path, PathBuf};

const DR_PATH: &str = "LASTOPENDIR.txt";

/// Saves the last opened directory path to a configuration file
///
/// # Arguments
/// * `path` - The directory path to save
///
/// # Returns
/// * `Result<(), String>` - Success or error message
pub fn set_dir(path: impl AsRef<Path>) -> Result<(), String> {
    let path = path.as_ref();
    // Verify the path is valid
    if !path.exists() {
        return Err(format!("Path does not exist: {}", path.to_string_lossy()));
    }
    if !path.is_dir() {
        return Err(format!(
            "Path is not a directory: {}",
            path.to_string_lossy()
        ));
    }

    // Get the configuration file path
    let dr_file = sub_path(DR_PATH)?;

    // Ensure the file exists
    ensure_file(&dr_file)?;

    // Convert path to string and write to file
    path.to_str()
        .ok_or_else(|| {
            format!(
                "Path contains invalid UTF-8 characters: {}",
                path.to_string_lossy()
            )
        })
        .and_then(|s| write_file(&dr_file, s))
}

/// Retrieves the last opened directory path from the configuration file
///
/// # Returns
/// * `Option<PathBuf>` - The directory path if available, None otherwise
pub fn get_dir() -> Option<PathBuf> {
    let dr_file = sub_path(DR_PATH).ok()?;
    if !dr_file.exists() {
        return None;
    }
    let content = read_file(&dr_file).ok()?;
    let path = PathBuf::from(content.trim());
    (path.exists() && path.is_dir()).then_some(path)
}
