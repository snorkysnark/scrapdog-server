mod blob;
mod enums;
pub mod schema;

use chrono::naive::NaiveDateTime;
use diesel::{Insertable, Queryable};
use schema::fs;
pub use blob::ChildIds;
pub use enums::NodeType;

pub mod dsl {
    pub use super::schema::fs::dsl as fs;
    pub use super::schema::scrapbooks::dsl as scrapbooks;
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
