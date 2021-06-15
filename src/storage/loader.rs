use std::collections::HashMap;

use super::{
    types::{BlobVecI32, NodeType},
    Storage,
};
use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Debug)]
pub struct FsNodeWithId {
    pub id: i32,
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

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FsNode {
    pub rdf_id: Option<String>,
    #[serde(rename = "type")]
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

impl FsNode {
    fn from_fs_node_with_id(node: FsNodeWithId) -> Self {
        FsNode {
            rdf_id: node.rdf_id,
            type_: node.type_,
            title: node.title,
            source: node.source,
            icon: node.icon,
            comment: node.comment,
            encoding: node.encoding,
            marked: node.marked,
            locked: node.locked,
            created: node.created,
            modified: node.modified,
            children: node.children,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ScrapbookFs {
    root: Vec<i32>,
    nodes: HashMap<i32, FsNode>,
}

impl Storage {
    pub fn get_scrapbook_nodes_raw(&self, scrapbook_id: i32) -> Result<Vec<FsNodeWithId>> {
        use super::schema::fs::dsl as fs;

        let nodes: Vec<FsNodeWithId> = fs::fs
            .select((
                fs::id,
                fs::rdf_id,
                fs::type_,
                fs::title,
                fs::source,
                fs::icon,
                fs::comment,
                fs::encoding,
                fs::marked,
                fs::locked,
                fs::created,
                fs::modified,
                fs::children,
            ))
            .filter(fs::scrapbook_id.eq(scrapbook_id))
            .order_by(fs::is_root.desc())
            .load(&self.db)?;
        Ok(nodes)
    }

    pub fn get_scrapbook_node_tree(&self, scrapbook_id: i32) -> Result<Option<ScrapbookFs>> {
        let mut nodes = self.get_scrapbook_nodes_raw(scrapbook_id)?.into_iter();
        if let Some(root_node) = nodes.next() {
            let root_id = root_node.id;
            let root_children = root_node.children.ok_or_else(|| {
                anyhow!("Root node with id {} has empty children vector", root_id)
            })?;

            let id_map: HashMap<i32, FsNode> = nodes
                .map(|node| (node.id, FsNode::from_fs_node_with_id(node)))
                .collect();

            Ok(Some(ScrapbookFs {
                root: root_children.into(),
                nodes: id_map,
            }))
        } else {
            Ok(None)
        }
    }
}
