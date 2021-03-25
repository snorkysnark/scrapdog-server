use crate::paths::ProjectDirs;
use anyhow::{Context, Result};
use diesel::prelude::*;
use std::path::PathBuf;

embed_migrations!();

no_arg_sql_function!(
    last_insert_rowid,
    diesel::sql_types::Integer,
    "Represents the SQL last_insert_row() function"
);

pub struct Storage {
    db: SqliteConnection,
}

impl Storage {
    pub fn init(dirs: &impl ProjectDirs) -> Result<Self> {
        create_folders(dirs)?;
        let db_path = make_db_path(dirs);

        let db = SqliteConnection::establish(&db_path.to_string_lossy())
            .context("Connecting to sqlite")?;
        embedded_migrations::run(&db).context("Running migrations")?;

        println!("Connected to database at {}", db_path.display());
        Ok(Storage { db })
    }
}

fn create_folders(dirs: &impl ProjectDirs) -> Result<()> {
    std::fs::create_dir_all(dirs.data_dir()).context("Creating data directory")?;
    Ok(())
}

fn make_db_path(dirs: &impl ProjectDirs) -> PathBuf {
    dirs.data_dir().join("fs.db")
}
