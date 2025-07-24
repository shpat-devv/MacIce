pub mod finder_mod {
    use std::path::PathBuf;
    use whoami;

    pub fn find_path(relative_path: &str) -> PathBuf {
        let username = whoami::username();
        let full_path = format!("/Users/{}/{}", username, relative_path);
        let file = PathBuf::from(&full_path);
        if file.exists() {
            file
        } else {
            PathBuf::new()
        }
    }
}
