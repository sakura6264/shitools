use crate::utils::*;
use std::path::PathBuf;

const DR_PATH: &str = "LASTOPENDIR.txt";

pub fn set_dir(path: PathBuf) {
    if let Ok(dr_file) = sub_path(DR_PATH) {
        let _ = ensure_file(&dr_file);
        if let Some(path_str) = path.to_str() {
            let _ = write_file(&dr_file, path_str);
        }
    }
}

pub fn get_dir() -> Option<PathBuf> {
    if let Ok(dr_file) = sub_path(DR_PATH) {
        if let Ok(content) = read_file(&dr_file) {
            return Some(PathBuf::from(content));
        }
    }
    None
}
