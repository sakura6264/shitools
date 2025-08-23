use std::{
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
};

//
// Path and Directory Utilities
//

/// Returns the directory of the executable
///
/// # Returns
/// - `Ok(PathBuf)` - The directory containing the executable
/// - `Err(String)` - Error message if the path cannot be determined
pub fn dir_path() -> Result<PathBuf, String> {
    let mut path =
        std::env::current_exe().map_err(|e| format!("Failed to get executable path: {}", e))?;
    path.pop();
    Ok(path)
}

/// Returns a static reference to the directory of the executable
/// This is cached for efficiency. Falls back to current_dir or "." if needed.
pub fn static_dir_path() -> &'static Path {
    static DIR: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();
    DIR.get_or_init(|| {
        dir_path()
            .or_else(|_| std::env::current_dir().map_err(|e| e.to_string()))
            .unwrap_or_else(|_| PathBuf::from("."))
    })
    .as_path()
}

/// Constructs a path relative to the executable directory
///
/// # Arguments
/// * `sub` - The relative path to append
///
/// # Returns
/// - `Ok(PathBuf)` - The constructed path
/// - `Err(String)` - Error message if the path cannot be determined
pub fn sub_path(sub: &str) -> Result<PathBuf, String> {
    let mut path = PathBuf::from(static_dir_path());
    path.push(sub);
    Ok(path)
}

//
// File System Operations
//

/// Ensures a directory exists, creating it if necessary
///
/// # Arguments
/// * `absolute` - The absolute path to the directory
///
/// # Returns
/// - `Ok(())` - The directory exists or was created
/// - `Err(String)` - Error message if the directory cannot be created
pub fn ensure_dir(absolute: impl AsRef<Path>) -> Result<(), String> {
    let absolute = absolute.as_ref();
    if absolute.exists() && !absolute.is_dir() {
        return Err(format!(
            "Path exists but is not a directory: {}",
            absolute.to_string_lossy()
        ));
    }
    if !absolute.exists() {
        std::fs::create_dir_all(absolute)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }
    Ok(())
}

/// Ensures a file exists, creating it if necessary
///
/// # Arguments
/// * `absolute` - The absolute path to the file
///
/// # Returns
/// - `Ok(())` - The file exists or was created
/// - `Err(String)` - Error message if the file cannot be created
pub fn ensure_file(absolute: impl AsRef<Path>) -> Result<(), String> {
    let absolute = absolute.as_ref();
    if absolute.exists() && !absolute.is_file() {
        return Err(format!(
            "Path exists but is not a file: {}",
            absolute.to_string_lossy()
        ));
    }
    if !absolute.exists() {
        std::fs::File::create(absolute).map_err(|e| format!("Failed to create file: {}", e))?;
    }
    Ok(())
}

/// Reads the contents of a file as a string
///
/// # Arguments
/// * `absolute` - The absolute path to the file
///
/// # Returns
/// - `Ok(String)` - The contents of the file
/// - `Err(String)` - Error message if the file cannot be read
pub fn read_file(absolute: impl AsRef<Path>) -> Result<String, String> {
    std::fs::read_to_string(absolute).map_err(|e| format!("Failed to read file: {}", e))
}

/// Writes content to a file
///
/// # Arguments
/// * `absolute` - The absolute path to the file
/// * `content` - The content to write
///
/// # Returns
/// - `Ok(())` - The content was written successfully
/// - `Err(String)` - Error message if the file cannot be written
pub fn write_file(absolute: impl AsRef<Path>, content: &str) -> Result<(), String> {
    std::fs::write(absolute, content).map_err(|e| format!("Failed to write file: {}", e))
}

/// Lists files in a subdirectory with optional extension filtering
///
/// # Arguments
/// * `sub` - The subdirectory relative to the executable
/// * `filter_exts` - List of file extensions to include (empty for all)
///
/// # Returns
/// - `Ok(Vec<String>)` - List of matching file paths
/// - `Err(String)` - Error message if the directory cannot be read
pub fn list_subdir_files(sub: &str, filter_exts: &[String]) -> Result<Vec<String>, String> {
    let path = sub_path(sub)?;
    if !path.exists() {
        return Err(format!(
            "Directory does not exist: {}",
            path.to_string_lossy()
        ));
    }
    if !path.is_dir() {
        return Err(format!(
            "Path is not a directory: {}",
            path.to_string_lossy()
        ));
    }

    let mut result = Vec::new();
    for entry in std::fs::read_dir(path).map_err(|e| format!("Failed to read directory: {}", e))? {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if filter_exts.is_empty()
                    || filter_exts.contains(&ext.to_string_lossy().to_lowercase())
                {
                    result.push(path.to_string_lossy().to_string());
                }
            }
        }
    }
    Ok(result)
}

//
// Random and Utility Functions
//

/// Generates a deterministic seed for random number generation
///
/// # Returns
/// A 32-byte array containing a seed derived from system properties
pub fn get_seed() -> [u8; 32] {
    let mut seed = [0; 32];

    // Get current time in nanoseconds or microseconds as fallback
    let time = chrono::Utc::now()
        .timestamp_nanos_opt()
        .unwrap_or(chrono::Utc::now().timestamp_micros())
        .to_le_bytes();

    // Get process ID in both little-endian and big-endian formats
    let pid = std::process::id();
    let pid_le = pid.to_le_bytes();
    let pid_be = pid.to_be_bytes();

    // Get thread ID hash
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    std::thread::current().id().hash(&mut hasher);
    let tid_hash = hasher.finish().to_le_bytes();

    // Add application magic bytes
    const MAGIC: &[u8; 8] = b"SHITOOLS";

    // Combine all sources into the seed
    seed[..8].copy_from_slice(&time);
    seed[8..12].copy_from_slice(&pid_le);
    seed[12..16].copy_from_slice(&pid_be);
    seed[16..24].copy_from_slice(&tid_hash);
    seed[24..32].copy_from_slice(MAGIC);

    seed
}

/// Formats a memory size in bytes to a human-readable string
///
/// # Arguments
/// * `mem` - Memory size in bytes
///
/// # Returns
/// A formatted string with appropriate units (B, KB, MB, GB)
pub fn format_mem(mem: usize) -> String {
    if mem < 1024 {
        format!("{}B", mem)
    } else if mem < 1024 * 1024 {
        format!("{:.2}KB", mem as f64 / 1024.0)
    } else if mem < 1024 * 1024 * 1024 {
        format!("{:.2}MB", mem as f64 / 1024.0 / 1024.0)
    } else {
        format!("{:.2}GB", mem as f64 / 1024.0 / 1024.0 / 1024.0)
    }
}
