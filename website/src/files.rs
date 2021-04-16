use std::path::PathBuf;
use dirs_next;

pub fn get_path() -> PathBuf {
    match dirs_next::home_dir() {
        Some (mut home) => {
            home.push("CanonCollisionWebsite");
            home
        }
        None => {
            panic!("could not get path of home");
        }
    }
}

pub fn git_path() -> PathBuf {
    let mut path = get_path();
    path.push("git_repo");
    path
}

pub fn builds_path() -> PathBuf {
    let mut path = get_path();
    path.push("builds");
    path
}
