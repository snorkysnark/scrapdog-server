#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod types;
mod parser;
mod paths;
mod storage;

use clap::Clap;
use directories_next::ProjectDirs;
use std::path::PathBuf;
use storage::Storage;

#[derive(Clap, Debug)]
#[clap(author, about, version)]
enum Args {
    Parse {
        file: PathBuf,
    },
    Import { file: PathBuf }
}

fn main() {
    let args = Args::parse();
    let dirs = ProjectDirs::from("", "", "scrapdog")
        .expect("Can't find project directory paths. Are you using an unsupported OS?");

    match args {
        Args::Parse{ file } => {
            let graph = parser::parse_file(&file).expect("Parsing error");
            println!("{:#?}", graph);
        },
        Args::Import{ file } => {
            let graph = parser::parse_file(&file).expect("Parsing error");
            let storage = Storage::init(&dirs).expect("Initializing sqlite");
            storage.import(graph).expect("Import error");
        }
    }
}
