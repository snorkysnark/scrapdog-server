mod node_mapper;

use super::schema::fs;
use super::Storage;
use crate::parser::RdfGraph;
use anyhow::Result;
use chrono::TimeZone;
use diesel::prelude::*;
use fs_extra::dir::CopyOptions;
use node_mapper::RemappedNode;
use std::path::Path;

#[derive(Insertable, Debug)]
#[table_name = "fs"]
struct NodeWithScrapbookId {
    scrapbook_id: i32,
    #[diesel(embed)]
    body: RemappedNode,
}

impl Storage {
    pub fn import(
        &self,
        rdf: RdfGraph,
        data_folder: impl AsRef<Path>,
        scrapbook_name: &str,
        timezone: &impl TimeZone,
    ) -> Result<()> {
        self.db.transaction(|| {
            let scrapbook_id = self.new_scrapbook(scrapbook_name)?;

            let next_node_id = self.next_node_id()?;
            let remapped_nodes: Vec<_> = node_mapper::remap(rdf, next_node_id, timezone)?
                .into_iter()
                .map(|node| NodeWithScrapbookId {
                    scrapbook_id,
                    body: node,
                })
                .collect();

            {
                use super::schema::fs::dsl::*;
                diesel::insert_into(fs)
                    .values(&remapped_nodes)
                    .execute(&self.db)?;
            }

            let copy_opts = CopyOptions {
                content_only: true,
                ..Default::default()
            };
            for node in remapped_nodes.iter() {
                if let Some(ref rdf_id) = node.body.rdf_id {
                    let old_folder = data_folder.as_ref().join(rdf_id);
                    if old_folder.is_dir() {
                        let new_folder = self.buckets.make_bucket_path_create(node.body.id)?;
                        fs_extra::dir::copy(&old_folder, &new_folder, &copy_opts)?;
                    }
                }
            }
            Ok(())
        })
    }
}
