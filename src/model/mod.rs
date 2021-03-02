mod schema;
mod blob;
mod enums;

use chrono::naive::NaiveDateTime;
use diesel::Insertable;
use schema::fs;
pub use blob::ChildIds;
pub use enums::NodeType;

#[derive(Insertable, Debug)]
#[table_name = "fs"]
pub struct FullNode {
    scrapbook_id: i32,
    #[diesel(embed)]
    data: LocalNode
}

#[derive(Insertable, Debug)]
#[table_name = "fs"]
pub struct LocalNode {
    id: i32,
    rdf_id: Option<String>,
    type_: Option<NodeType>,
    created: Option<NaiveDateTime>,
    modified: Option<NaiveDateTime>,
    source: Option<String>,
    icon: Option<String>,
    comment: Option<String>,
    encoding: Option<String>,
    marked: bool,
    locked: bool,
    children: Option<ChildIds>,
}
