use crate::{
    model::{dsl, LocalNode, FullNode, NodeType, Scrapbook},
    paths::ProjectDirs,
};
use anyhow::{Context, Result};
use diesel::{Connection, RunQueryDsl, SqliteConnection};
use std::fs;
use std::path::PathBuf;

embed_migrations!();

pub fn init(dirs: &impl ProjectDirs) -> Result<()> {
    create_folders(dirs)?;
    let db_path = make_db_path(dirs);

    let db =
        SqliteConnection::establish(&db_path.to_string_lossy()).context("Connecting to sqlite")?;
    embedded_migrations::run(&db).context("Running migrations")?;

    println!("Connected to database at {}", db_path.display());
    test_insert(&db)?;
    println!("Inserted some test data");

    Ok(())
}

fn test_insert(db: &SqliteConnection) -> Result<()> {
    diesel::insert_into(dsl::scrapbooks::scrapbooks)
        .values(&Scrapbook {
            name: "Test Scrapbook".to_owned(),
        })
        .execute(db)?;

    let date = chrono::Local::now().naive_utc();

    diesel::insert_into(dsl::fs::fs).values(&FullNode {
        scrapbook_id: 0,
        data: LocalNode {
            id: 0,
            rdf_id: Some("1234567".to_owned()),
            type_: Some(NodeType::Page),
            created: Some(date.clone()),
            modified: Some(date),
            source: Some("http://foo.bar".to_owned()),
            icon: Some("icons/pepe.png".to_owned()),
            comment: Some("^_^".to_owned()),
            encoding: None,
            marked: true,
            locked: false,
            children: Some(vec![0,3,5,2,9].into())
        }
    }).execute(db)?;

    Ok(())
}

fn create_folders(dirs: &impl ProjectDirs) -> Result<()> {
    fs::create_dir_all(dirs.data_dir()).context("Creating data directory")?;
    Ok(())
}

fn make_db_path(dirs: &impl ProjectDirs) -> PathBuf {
    dirs.data_dir().join("fs.db")
}
