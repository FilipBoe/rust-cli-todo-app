use std::{ env, fs };
use std::path::PathBuf;

pub fn project_folder() -> PathBuf {
    if cfg!(debug_assertions) {
        env::current_dir().expect("Failed to get current directory")
    } else {
        let exe_path = env::current_exe().expect("Failed to determine executable path");
        let canonical_path = fs::canonicalize(&exe_path).expect("Failed to resolve symlink");

        canonical_path.parent().expect("Failed to get directory of the executable").to_path_buf()
    }
}
