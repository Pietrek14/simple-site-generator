use std::{env, path::PathBuf};

use ssg::build_site_at_path;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        panic!("Usage: ssd <filename>");
    }

    let filename = args[1].clone();

    println!("{}", build_site_at_path(PathBuf::from(filename)));
}
