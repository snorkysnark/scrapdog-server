mod schema;
mod importer;
mod blob;

use crate::paths::ProjectDirs;
use anyhow::{Context, Result};
use diesel::prelude::*;
use std::path::PathBuf;

type DieselError = diesel::result::Error;

embed_migrations!();

//no_arg_sql_function!(
//    last_insert_rowid,
//    diesel::sql_types::Integer,
//    "Represents the SQL last_insert_row() function"
//);

pub struct Storage {
    db: SqliteConnection,
}

impl Storage {
    pub fn init(dirs: &impl ProjectDirs) -> Result<Self> {
        fn create_folders(dirs: &impl ProjectDirs) -> Result<()> {
            std::fs::create_dir_all(dirs.data_dir()).context("Creating data directory")?;
            Ok(())
        }

        fn make_db_path(dirs: &impl ProjectDirs) -> PathBuf {
            dirs.data_dir().join("fs.db")
        }

        create_folders(dirs)?;
        let db_path = make_db_path(dirs);

        let db = SqliteConnection::establish(&db_path.to_string_lossy())
            .context("Connecting to sqlite")?;
        embedded_migrations::run(&db).context("Running migrations")?;

        Ok(Storage { db })
    }

    pub fn next_bucket_id(&self) -> Result<i32> {
        use schema::fs::dsl::*;
        use diesel::dsl::max;

        match fs.select(max(bucket_id)).first::<Option<i32>>(&self.db) {
            Ok(Some(max_id)) => Ok(max_id + 1),
            Ok(None) | Err(DieselError::NotFound) => Ok(1),
            Err(other) => Err(other.into())
        }
    }
}
