#[macro_use]
extern crate clap;

use clap::{App};

fn main() {
    let matches = App::new("tree")
        .author(crate_authors!())
        .version(crate_version!())
        .about("A GNU Tree-like app built in Rust")
        .get_matches();
}
