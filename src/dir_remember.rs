use crate::utils::*;
use std::path::PathBuf;

const DR_PATH: &str = "LASTOPENDIR.txt";

/// Saves the last opened directory path to a configuration file
///
/// # Arguments
/// * `path` - The directory path to save
///
/// # Returns
/// * `Result<(), String>` - Success or error message
pub fn set_dir(path: PathBuf) -> Result<(), String> {
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
    if let Some(path_str) = path.to_str() {
        write_file(&dr_file, path_str)
    } else {
        Err(format!(
            "Path contains invalid UTF-8 characters: {}",
            path.to_string_lossy()
        ))
    }
}

/// Retrieves the last opened directory path from the configuration file
///
/// # Returns
/// * `Option<PathBuf>` - The directory path if available, None otherwise
pub fn get_dir() -> Option<PathBuf> {
    // Try to get the configuration file path
    let dr_file = match sub_path(DR_PATH) {
        Ok(path) => path,
        Err(_) => return None,
    };

    // Check if the file exists
    if !dr_file.exists() {
        return None;
    }

    // Read the file content
    match read_file(&dr_file) {
        Ok(content) => {
            let path = PathBuf::from(content.trim());

            // Verify the path still exists
            if path.exists() && path.is_dir() {
                Some(path)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}
