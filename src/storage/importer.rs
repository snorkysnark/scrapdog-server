use super::{blob::BlobVecI32, schema::fs, Storage};
use crate::parser::{RdfGraph, RdfNode};
use crate::types::{NodeType, UnresolvedIcon, UnresolvedTime};
use anyhow::Result;
use chrono::{NaiveDateTime, TimeZone};
use diesel::prelude::*;
use minidom::element::Texts;

#[derive(Insertable, Debug)]
#[table_name = "fs"]
struct ImportedNode {
    scrapbook_id: i32,
    id: i32,
    bucket_id: Option<i32>,
    rdf_id: Option<String>,
    type_: Option<NodeType>,
    title: Option<String>,
    source: Option<String>,
    icon: Option<String>,
    comment: Option<String>,
    encoding: Option<String>,
    marked: bool,
    locked: bool,
    created: Option<NaiveDateTime>,
    modified: Option<NaiveDateTime>,
    children: Option<BlobVecI32>,
}

impl ImportedNode {
    fn root(scrapbook_id: i32, children: BlobVecI32) -> Self {
        ImportedNode {
            scrapbook_id,
            id: 0,
            bucket_id: None,
            rdf_id: None,
            type_: None,
            title: None,
            source: None,
            icon: None,
            comment: None,
            encoding: None,
            marked: false,
            locked: false,
            created: None,
            modified: None,
            children: Some(children),
        }
    }
}

fn convert_rdf_node(
    rdf: RdfNode,
    scrapbook_id: i32,
    id: i32,
    bucket_id: Option<i32>,
    timezone: &impl TimeZone,
) -> Result<ImportedNode> {
    fn convert_time(
        timeopt: Option<UnresolvedTime>,
        timezone: &impl TimeZone,
    ) -> Result<Option<NaiveDateTime>> {
        timeopt.map(|time| time.resolve(timezone)).transpose()
    }

    fn convert_icon(iconopt: Option<UnresolvedIcon>, bucket_id: Option<i32>) -> Result<Option<String>> {
        iconopt.map(|icon| icon.resolve(bucket_id)).transpose()
    }

    Ok(ImportedNode {
        scrapbook_id,
        id,
        bucket_id,
        rdf_id: Some(rdf.rdf_id),
        type_: Some(rdf.type_),
        title: rdf.title,
        source: rdf.source,
        icon: convert_icon(rdf.icon, bucket_id)?,
        comment: rdf.comment,
        encoding: rdf.encoding,
        marked: rdf.marked,
        locked: rdf.locked,
        created: convert_time(rdf.created, timezone)?,
        modified: convert_time(rdf.modified, timezone)?,
        children: rdf.children.map(|vec| vec.into()),
    })
}

fn needs_a_bucket(type_: NodeType) -> bool {
    matches!(
        type_,
        NodeType::Page | NodeType::File | NodeType::Note | NodeType::Notex
    )
}

impl Storage {
    pub fn import(&self, rdf: RdfGraph) -> Result<()> {
        println!("{:#?}", self.make_insertables(rdf, 0, chrono::Local)?);
        Ok(())
    }

    fn make_insertables(
        &self,
        rdf: RdfGraph,
        scrapbook_id: i32,
        timezone: impl TimeZone,
    ) -> Result<Vec<ImportedNode>> {
        let mut next_bucket_id = self.next_bucket_id()?;

        let mut nodes: Vec<ImportedNode> = Vec::new();
        nodes.push(ImportedNode::root(scrapbook_id, rdf.root.into()));

        for (id, rdf_node) in rdf.nodes.into_iter().enumerate() {
            let bucket_id = if needs_a_bucket(rdf_node.type_) {
                let bucket_id = next_bucket_id;
                next_bucket_id += 1;
                Some(bucket_id)
            } else {
                None
            };

            let node = convert_rdf_node(rdf_node, scrapbook_id, id as i32, bucket_id, &timezone)?;
            nodes.push(node);
        }

        Ok(nodes)
    }
}
