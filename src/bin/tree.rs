#[macro_use]
extern crate clap;

use clap::{App, Arg};

use std::fs::{self};
use std::path::Path;

fn walk(path: &str) {
    println!("Walking over {}", path);
    if try!(fs::metadata(path)).is_dir() {
        for entry in try!(fs::read_dir(path)) {

        }
    }
}

fn main() {
    let matches = App::new("tree")
        .author(crate_authors!())
        .version(crate_version!())
        .about("A GNU Tree-like app built in Rust")
        .arg(Arg::with_name("level")
             .short("l")
             .long("level")
             .value_name("LEVEL")
             .takes_value(true)
             .help("How many levels of depth to go when printing the tree"))
        .arg(Arg::with_name("path")
             .help("Path to start from"))
        .get_matches();

    let path = matches.value_of("path").unwrap_or(".");
    walk(path);
}

