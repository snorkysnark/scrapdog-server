mod parser;

use std::path::PathBuf;
use clap::Clap;

#[derive(Clap, Debug)]
#[clap(author, about, version)]
enum Args {
    Import{ file: PathBuf }
}

fn main() {
    let args = Args::parse();

    match args {
        Args::Import{ file } => parser::parse_file(file).unwrap()
    }
}
