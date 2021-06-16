mod buckets;
mod importer;
mod loader;
mod schema;
mod types;

use std::path::Path;

use crate::paths::ProjectDirs;
use anyhow::{Context, Result};
use buckets::BucketsFolder;
use diesel::prelude::*;
use serde::Serialize;

type DieselError = diesel::result::Error;

embed_migrations!();

no_arg_sql_function!(
    last_insert_rowid,
    diesel::sql_types::Integer,
    "Represents the SQL last_insert_row() function"
);

pub struct Storage {
    db: SqliteConnection,
    buckets: BucketsFolder,
}

impl Storage {
    pub fn init(dirs: &impl ProjectDirs) -> Result<Self> {
        let data_folder = dirs.data_dir();
        let buckets_folder = data_folder.join("data");
        let db_path = data_folder.join("fs.db");

        std::fs::create_dir_all(&buckets_folder).context("Creating data folder")?;

        let db = SqliteConnection::establish(&db_path.to_string_lossy())
            .context("Connecting to sqlite")?;
        embedded_migrations::run(&db).context("Running migrations")?;

        Ok(Storage {
            db,
            buckets: BucketsFolder::from_path(buckets_folder),
        })
    }

    pub fn next_node_id(&self) -> Result<i32> {
        use diesel::dsl::max;
        use schema::fs::dsl::*;

        match fs.select(max(id)).first::<Option<i32>>(&self.db) {
            Ok(Some(max_id)) => Ok(max_id + 1),
            Ok(None) | Err(DieselError::NotFound) => Ok(1),
            Err(other) => Err(other.into()),
        }
    }

    pub fn new_scrapbook(&self, scrapbook_name: &str) -> Result<i32> {
        use schema::scrapbooks::dsl::*;

        diesel::insert_into(scrapbooks)
            .values(name.eq(scrapbook_name))
            .execute(&self.db)
            .context("Creating a new scrapbook")?;

        let scrapbooks_id: i32 = diesel::select(last_insert_rowid).get_result(&self.db)?;
        Ok(scrapbooks_id)
    }

    pub fn list_scrapbooks(&self) -> Result<Vec<ScrapbookEntry>> {
        use schema::scrapbooks::dsl::*;

        let names: Vec<ScrapbookEntry> =
            scrapbooks.select((id, name)).order_by(id).load(&self.db)?;
        Ok(names)
    }

    pub fn get_bucket_path(&self) -> &Path {
        self.buckets.get_path()
    }
}

#[derive(Queryable, Serialize)]
pub struct ScrapbookEntry {
    id: i32,
    name: String,
}
