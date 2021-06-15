#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod parser;
mod paths;
mod server;
mod storage;

use clap::Clap;
use directories_next::ProjectDirs;
use std::path::{Path, PathBuf};
use storage::Storage;

#[derive(Clap, Debug)]
#[clap(author, about, version)]
enum Args {
    Parse {
        folder: PathBuf,
    },
    Import {
        folder: PathBuf,
        scrapbook_name: String,
    },
    Serve,
    Load {
        scrapbook_id: i32,
    },
}

fn main() {
    let args = Args::parse();
    let dirs = ProjectDirs::from("", "", "scrapdog")
        .expect("Can't find project directory paths. Are you using an unsupported OS?");

    fn make_rdf_path(folder: impl AsRef<Path>) -> PathBuf {
        folder.as_ref().join("scrapbook.rdf")
    }

    fn make_data_path(folder: impl AsRef<Path>) -> PathBuf {
        folder.as_ref().join("data")
    }

    match args {
        Args::Parse { folder } => {
            let rdf_path = make_rdf_path(folder);
            let graph = parser::parse_file(rdf_path).expect("Parsing error");
            println!("{:#?}", graph);
        }
        Args::Import {
            folder,
            scrapbook_name,
        } => {
            let rdf_path = make_rdf_path(&folder);

            let graph = parser::parse_file(rdf_path).expect("Parsing error");
            let storage = Storage::init(&dirs).expect("Initializing sqlite");
            storage
                .import(
                    graph,
                    make_data_path(&folder),
                    &scrapbook_name,
                    &chrono::Local,
                )
                .expect("Import error");
        }
        Args::Serve => {
            let storage = Storage::init(&dirs).expect("Initializing sqlite");
            server::serve(storage);
        }
        Args::Load { scrapbook_id } => {
            let storage = Storage::init(&dirs).expect("Initializing sqlite");
            let tree = storage
                .get_scrapbook_node_tree(scrapbook_id)
                .expect("Query failed");
            serde_json::to_writer_pretty(std::io::stdout(), &tree).unwrap();
        }
    }
}
