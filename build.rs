use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

fn main() {
    let lua_dir = get_output_path().join("lua");
    fs::create_dir_all(&lua_dir).unwrap();
    let lua_preload = lua_dir.join("preload.lua");
    let lua_preload_from = Path::new("assets/preload.lua");
    fs::copy(&lua_preload_from, &lua_preload).unwrap();
    let _ = embed_resource::compile("assets/icon.rc", embed_resource::NONE);
}

fn get_output_path() -> &'static PathBuf {
    //<root or manifest path>/target/<profile>/
    static ONCE: OnceLock<PathBuf> = OnceLock::new();
    ONCE.get_or_init(|| {
        let manifest_dir_string = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let build_type = std::env::var("PROFILE").unwrap();
        let path = Path::new(&manifest_dir_string)
            .join("target")
            .join(build_type);
        path
    })
}
