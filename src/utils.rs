use std::{
    hash::{Hash, Hasher},
    path::PathBuf,
};

pub fn dir_path() -> Result<PathBuf, String> {
    // return the directory of the executable
    let mut path = std::env::current_exe().map_err(|e| e.to_string())?;
    path.pop();
    Ok(path)
}

pub fn static_dir_path() -> &'static std::path::PathBuf {
    // return the directory of the executable
    static DIR: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();
    DIR.get_or_init(|| dir_path().unwrap())
}

pub fn sub_path(sub: &str) -> Result<std::path::PathBuf, String> {
    // return dir_path/sub
    let mut path = static_dir_path().clone();
    path.push(sub);
    Ok(path)
}

pub fn ensure_dir(absolute: &PathBuf) -> Result<(), String> {
    // ensure absolute exists
    if absolute.exists() && !absolute.is_dir() {
        return Err(format!("{} is not a directory", absolute.to_string_lossy()));
    }
    if !absolute.exists() {
        std::fs::create_dir_all(absolute).map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub fn ensure_file(absolute: &PathBuf) -> Result<(), String> {
    // ensure absolute exists
    if absolute.exists() && !absolute.is_file() {
        return Err(format!("{} is not a file", absolute.to_string_lossy()));
    }
    if !absolute.exists() {
        std::fs::File::create(absolute).map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub fn read_file(absolute: &PathBuf) -> Result<String, String> {
    // read absolute
    std::fs::read_to_string(absolute).map_err(|e| e.to_string())
}

pub fn write_file(absolute: &PathBuf, content: &str) -> Result<(), String> {
    // write content to absolute
    std::fs::write(absolute, content).map_err(|e| e.to_string())
}

pub fn list_subdir_files(sub: &str, filter_exts: &[String]) -> Result<Vec<String>, String> {
    // use lowercase for comparison
    let path = sub_path(sub)?;
    if !path.exists() {
        return Err(format!("{} does not exist", path.to_string_lossy()));
    }
    if !path.is_dir() {
        return Err(format!("{} is not a directory", path.to_string_lossy()));
    }
    let mut ret = Vec::new();
    for entry in std::fs::read_dir(path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if filter_exts.is_empty()
                    || filter_exts.contains(&ext.to_string_lossy().to_lowercase())
                {
                    ret.push(path.to_string_lossy().to_string());
                }
            }
        }
    }
    Ok(ret)
}

pub fn get_seed() -> [u8; 32] {
    // return a seed for ChaChaRng
    let mut seed = [0; 32];
    let time = chrono::Utc::now()
        .timestamp_nanos_opt()
        .unwrap_or(chrono::Utc::now().timestamp_micros())
        .to_le_bytes();
    let pid = std::process::id();
    let pid_le = pid.to_le_bytes();
    let pid_be = pid.to_be_bytes();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    std::thread::current().id().hash(&mut hasher);
    let tid_hash = hasher.finish().to_le_bytes();
    const MAGIC: &[u8; 8] = b"SHITOOLS";
    // seed = [time, pid_le, pid_be, tid_hash, MAGIC]
    seed[..8].copy_from_slice(&time);
    seed[8..12].copy_from_slice(&pid_le);
    seed[12..16].copy_from_slice(&pid_be);
    seed[16..24].copy_from_slice(&tid_hash);
    seed[24..32].copy_from_slice(MAGIC);
    seed
}

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
