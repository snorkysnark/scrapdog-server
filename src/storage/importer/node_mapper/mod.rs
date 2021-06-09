mod icon;
mod time;

use crate::parser::{RdfGraph, UnresolvedTime};
use crate::storage::schema::fs;
use crate::storage::types::{BlobVecI32, NodeType};
use anyhow::Result;
use chrono::{NaiveDateTime, TimeZone};
use icon::ConvertIconPath;
use time::ConvertTime;

#[derive(Insertable, Debug, Default)]
#[table_name = "fs"]
pub struct RemappedNode {
    pub id: i32,
    pub is_root: bool,
    pub rdf_id: Option<String>,
    pub type_: Option<NodeType>,
    pub title: Option<String>,
    pub source: Option<String>,
    pub icon: Option<String>,
    pub comment: Option<String>,
    pub encoding: Option<String>,
    pub marked: Option<bool>,
    pub locked: Option<bool>,
    pub created: Option<NaiveDateTime>,
    pub modified: Option<NaiveDateTime>,
    pub children: Option<BlobVecI32>,
}

pub fn remap(rdf: RdfGraph, root_id: i32, timezone: &impl TimeZone) -> Result<Vec<RemappedNode>> {
    let id_shift = root_id + 1;
    let mut new_nodes = Vec::<RemappedNode>::new();

    fn shift_ids(mut ids: Vec<i32>, shift: i32) -> BlobVecI32 {
        for id in ids.iter_mut() {
            *id += shift;
        }
        ids.into()
    }

    new_nodes.push(RemappedNode {
        id: root_id,
        children: Some(shift_ids(rdf.root, id_shift)),
        is_root: true,
        ..Default::default()
    });

    for (local_id, node) in rdf.nodes.into_iter().enumerate() {
        fn convert_time(
            time: Option<UnresolvedTime>,
            tz: &impl TimeZone,
        ) -> Result<Option<NaiveDateTime>> {
            time.map(|time| time.local_to_utc(tz)).transpose()
        }

        let node_id = local_id as i32 + id_shift;
        let remapped_node = RemappedNode {
            id: node_id,
            is_root: false,
            rdf_id: Some(node.rdf_id),
            type_: Some(node.type_.into()),
            title: node.title,
            source: node.source,
            icon: node.icon.map(|icon| icon.into_icon_path(node_id)),
            comment: node.comment,
            encoding: node.encoding,
            marked: Some(node.marked),
            locked: Some(node.locked),
            created: convert_time(node.created, timezone)?,
            modified: convert_time(node.modified, timezone)?,
            children: node.children.map(|ids| shift_ids(ids, id_shift)),
        };
        new_nodes.push(remapped_node);
    }
    Ok(new_nodes)
}
