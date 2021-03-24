#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod model;
mod parser;
mod paths;
mod storage;

use clap::Clap;
use directories_next::ProjectDirs;
use std::path::PathBuf;
//use storage::Storage;

#[derive(Clap, Debug)]
#[clap(author, about, version)]
enum Args {
    Parse {
        file: PathBuf,
    },
}

fn main() {
    let args = Args::parse();
    let dirs = ProjectDirs::from("", "", "scrapdog")
        .expect("Can't find project directory paths. Are you using an unsupported OS?");

    match args {
        Args::Parse{ file } => {
            let graph = parser::parse_file(&file).expect("Parsing error");
            println!("{:#?}", graph);
        }
    }
}
