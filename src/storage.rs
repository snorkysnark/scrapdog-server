use crate::{
    model::{
        dsl,
        schema::{fs, scrapbooks},
        NodeInfo,
    },
    paths::ProjectDirs,
};
use anyhow::{Context, Result};
use diesel::{Connection, Insertable, RunQueryDsl, SqliteConnection};
use std::path::PathBuf;

embed_migrations!();

no_arg_sql_function!(
    last_insert_rowid,
    diesel::sql_types::Integer,
    "Represents the SQL last_insert_row() function"
);

#[derive(Insertable, Debug)]
#[table_name = "scrapbooks"]
struct ScrapbookNew<'a> {
    pub name: &'a str,
}

#[derive(Insertable, Debug)]
#[table_name = "fs"]
struct NodeImport {
    scrapbook_id: i32,
    #[diesel(embed)]
    data: NodeInfo,
}

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

    pub fn import_scrapbook(&self, name: &str, nodes: Vec<NodeInfo>) -> Result<()> {
        diesel::insert_into(dsl::scrapbooks::scrapbooks)
            .values(&ScrapbookNew { name })
            .execute(&self.db)
            .context("Creating new entry in 'scrapbooks")?;

        let scrapbook_id: i32 = diesel::select(last_insert_rowid)
            .get_result(&self.db)
            .context("Getting last insert id")?;

        let nodes: Vec<NodeImport> = nodes
            .into_iter()
            .map(|data| NodeImport { scrapbook_id, data })
            .collect();
        diesel::insert_into(dsl::fs::fs)
            .values(&nodes)
            .execute(&self.db).context("Inserting imported nodes")?;

        Ok(())
    }
}

fn create_folders(dirs: &impl ProjectDirs) -> Result<()> {
    std::fs::create_dir_all(dirs.data_dir()).context("Creating data directory")?;
    Ok(())
}

fn make_db_path(dirs: &impl ProjectDirs) -> PathBuf {
    dirs.data_dir().join("fs.db")
}
