mod schema;
mod blob;
mod enums;

use chrono::naive::NaiveDateTime;
use diesel::Insertable;
use schema::fs;
use blob::ChildIds;
use enums::NodeType;

#[derive(Insertable, Debug)]
#[table_name = "fs"]
struct ImportedNodeFull {
    scrapbook_id: i32,
    #[diesel(embed)]
    data: ImportedNode
}

#[derive(Insertable, Debug)]
#[table_name = "fs"]
struct ImportedNode {
    id: i32,
    is_root: bool,
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
