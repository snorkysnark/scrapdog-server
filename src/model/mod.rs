mod blob;
mod enums;
pub mod schema;

use chrono::naive::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::Serialize;
use schema::fs;
pub use blob::ChildIds;
pub use enums::NodeType;

pub mod dsl {
    pub use super::schema::fs::dsl as fs;
    pub use super::schema::scrapbooks::dsl as scrapbooks;
}

#[derive(Queryable, Serialize, Debug)]
pub struct NodeFull {
    pub scrapbook_id: i32,
    pub id: i32,
    pub rdf_id: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<NodeType>,
    pub title: Option<String>,
    pub source: Option<String>,
    pub icon: Option<String>,
    pub comment: Option<String>,
    pub encoding: Option<String>,
    pub marked: bool,
    pub locked: bool,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
    pub children: Option<ChildIds>,
}

#[derive(Insertable, Debug, Default)]
#[table_name = "fs"]
pub struct NodeBody {
    pub rdf_id: Option<String>,
    pub type_: Option<NodeType>,
    pub title: Option<String>,
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

impl NodeBody {
    pub fn root() -> Self {
        Self::default()
    }
}
