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
use storage::Storage;

#[derive(Clap, Debug)]
#[clap(author, about, version)]
enum Args {
    Import {
        scrapbook_name: String,
        file: PathBuf,
    },
    Load {
        scrapbook_id: i32,
    },
}

fn main() {
    let args = Args::parse();
    let dirs = ProjectDirs::from("", "", "scrapdog")
        .expect("Can't find project directory paths. Are you using an unsupported OS?");

    match args {
        Args::Import {
            scrapbook_name,
            file,
        } => {
            let timezone = chrono::Local;
            let nodes = parser::parse_file(&file, &timezone).expect("Parsing error");

            let storage = Storage::init(&dirs).expect("Database init error");
            storage
                .import_scrapbook(&scrapbook_name, nodes)
                .expect("Import error");
            println!("Successfully imported file");
        }
        Args::Load { scrapbook_id } => {
            let storage = Storage::init(&dirs).expect("Database init error");
            let nodes = storage.load_with_id(scrapbook_id).expect("Query failed");

            for node in nodes.iter() {
                println!(
                    "{}\n",
                    serde_json::to_string(&node).expect("Json serialization error")
                );
            }
        }
    }
}
