use std::{env, path::PathBuf, fs};

use ssg::build_site_at_path;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        panic!("Usage: ssd <filename>");
    }

    let filename = args[1].clone();

    let site = build_site_at_path(PathBuf::from(filename));

    fs::write("index.html", site).expect("Couldn't save to file!");
}
