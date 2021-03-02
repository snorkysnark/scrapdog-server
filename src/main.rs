#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod parser;
mod paths;
mod storage;

use clap::Clap;
use directories_next::ProjectDirs;
use std::path::PathBuf;

#[derive(Clap, Debug)]
#[clap(author, about, version)]
enum Args {
    InitDb,
    Import { file: PathBuf },
}

fn main() {
    let args = Args::parse();
    let dirs = ProjectDirs::from("", "", "scrapdog")
        .expect("Can't find project directory paths. Are you using an unsupported OS?");

    match args {
        Args::InitDb => storage::init(&dirs).unwrap(),
        Args::Import { file } => parser::parse_file(file).unwrap(),
    }
}
