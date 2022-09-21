use std::{path::PathBuf, fs::read_to_string};

pub fn build_site_at_path(path: PathBuf) -> String {
    let contents = read_to_string(path).expect("Given file could not be read!");

    let title = path.as_os_str();

    build_site(contents.as_str(), title)
}

pub fn build_site(md: &str, title: &str) -> String {
    "".into()
}