mod blob;
mod enums;
pub mod schema;

use chrono::naive::NaiveDateTime;
use diesel::Insertable;
use schema::fs;
pub use blob::ChildIds;
pub use enums::NodeType;

pub mod dsl {
    pub use super::schema::fs::dsl as fs;
    pub use super::schema::scrapbooks::dsl as scrapbooks;
}

#[derive(Insertable, Debug)]
#[table_name = "fs"]
pub struct NodeInfo {
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
