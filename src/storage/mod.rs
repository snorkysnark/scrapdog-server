pub mod schema;

use crate::paths::ProjectDirs;
use anyhow::{Context, Result};
use diesel::{Connection, SqliteConnection};
use std::fs;
use std::path::PathBuf;

embed_migrations!();

pub fn init(dirs: &impl ProjectDirs) -> Result<()> {
    create_folders(dirs)?;
    let db_path = make_db_path(dirs);

    let db = SqliteConnection::establish(&db_path.to_string_lossy()).context("Connecting to sqlite")?;
    embedded_migrations::run(&db).context("Running migrations")?;

    println!("Connected to database at {}", db_path.display());

    Ok(())
}

fn create_folders(dirs: &impl ProjectDirs) -> Result<()> {
    fs::create_dir_all(dirs.data_dir()).context("Creating data directory")?;
    Ok(())
}

fn make_db_path(dirs: &impl ProjectDirs) -> PathBuf {
    dirs.data_dir().join("fs.db")
}
