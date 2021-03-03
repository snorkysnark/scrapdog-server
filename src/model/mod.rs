mod blob;
mod enums;
mod schema;

use chrono::naive::NaiveDateTime;
use diesel::Insertable;
use schema::{fs,scrapbooks};
pub use blob::ChildIds;
pub use enums::NodeType;

pub mod dsl {
    pub use super::schema::fs::dsl as fs;
    pub use super::schema::scrapbooks::dsl as scrapbooks;
}

#[derive(Insertable, Debug)]
#[table_name = "scrapbooks"]
pub struct Scrapbook {
    pub name: String
}

#[derive(Insertable, Debug)]
#[table_name = "fs"]
pub struct FullNode {
    pub scrapbook_id: i32,
    #[diesel(embed)]
    pub data: LocalNode
}

#[derive(Insertable, Debug)]
#[table_name = "fs"]
pub struct LocalNode {
    pub id: i32,
    pub rdf_id: Option<String>,
    pub type_: Option<NodeType>,
    pub created: Option<NaiveDateTime>,
    pub modified: Option<NaiveDateTime>,
    pub source: Option<String>,
    pub icon: Option<String>,
    pub comment: Option<String>,
    pub encoding: Option<String>,
    pub marked: bool,
    pub locked: bool,
    pub children: Option<ChildIds>,
}
