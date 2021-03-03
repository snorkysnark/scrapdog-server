#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod paths;
mod model;
mod parser;
mod storage;

use clap::Clap;
use directories_next::ProjectDirs;
use std::path::PathBuf;
use storage::Storage;

#[derive(Clap, Debug)]
#[clap(author, about, version)]
enum Args {
    TestImport,
    Import { file: PathBuf },
}

fn main() {
    let args = Args::parse();
    let dirs = ProjectDirs::from("", "", "scrapdog")
        .expect("Can't find project directory paths. Are you using an unsupported OS?");

    match args {
        Args::TestImport => {
            use model::{NodeInfo, NodeType};

            let storage = Storage::init(&dirs).unwrap();
            storage.import_scrapbook("Cacaboojo", vec![
                NodeInfo {
                    id: 0,
                    rdf_id: None,
                    type_: None,
                    created: None,
                    modified: None,
                    source: None,
                    icon: None,
                    comment: None,
                    encoding: None,
                    marked: false,
                    locked: false,
                    children: Some(vec![1,2].into())
                },
                NodeInfo {
                    id: 1,
                    rdf_id: Some("98q24978204".into()),
                    type_: Some(NodeType::Page),
                    created: Some(chrono::Local::now().naive_utc()),
                    modified: Some(chrono::Local::now().naive_utc()),
                    source: Some("http://foo".into()),
                    icon: Some("icons/jkawf".into()),
                    comment: None,
                    encoding: None,
                    marked: true,
                    locked: true,
                    children: None
                }
            ]).unwrap();
        },
        Args::Import { file } => parser::parse_file(file).unwrap(),
    }
}
