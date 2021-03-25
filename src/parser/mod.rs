mod icon;
mod regex_utils;
mod resource;

use anyhow::{anyhow, Context, Result};
use chrono::NaiveDateTime;
use icon::UnresolvedIcon;
use minidom::Element;
use resource::RdfResource;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::model::NodeType;

type ChildIds = Vec<i32>;

#[derive(Debug)]
pub struct RdfNode {
    rdf_id: String,
    type_: NodeType,
    title: Option<String>,
    created: Option<UnresolvedTime>,
    modified: Option<UnresolvedTime>,
    source: Option<String>,
    icon: Option<UnresolvedIcon>,
    comment: Option<String>,
    encoding: Option<String>,
    marked: bool,
    locked: bool,
    children: Option<ChildIds>,
}

#[derive(Debug)]
pub struct UnresolvedTime(NaiveDateTime);

impl UnresolvedTime {
    fn parse(timestr: &str) -> Result<Self> {
        let naive = NaiveDateTime::parse_from_str(timestr, "%Y%m%d%H%M%S")?;
        Ok(UnresolvedTime(naive))
    }
}

#[derive(Default)]
struct RdfGraphIndexed {
    root: ChildIds,
    nodes: Vec<RdfNode>,

    node_ids: HashMap<String, usize>,
}

impl RdfGraphIndexed {
    fn new() -> Self {
        Self::default()
    }

    fn add(&mut self, node: RdfNode) {
        self.node_ids.insert(node.rdf_id.clone(), self.nodes.len());
        self.nodes.push(node);
    }

    fn find_id(&self, rdf_id: &str) -> Result<usize> {
        let id = self
        .node_ids
        .get(rdf_id)
        .ok_or_else(|| anyhow!("Resource id {} not found", rdf_id))?;

        Ok(*id)
    }

    fn make_folder(&mut self, resource: RdfResource) -> Result<RdfFolder> {
        match resource {
            RdfResource::Root => Ok(RdfFolder{ graph: self, node_id: None }),

            RdfResource::RdfId(rdf_id) => {
                let id = self.find_id(rdf_id)?;

                self.nodes[id].children = Some(Vec::new());
                Ok(RdfFolder {
                    graph: self,
                    node_id: Some(id),
                })
            }
        }
    }
}

struct RdfFolder<'a> {
    graph: &'a mut RdfGraphIndexed,
    node_id: Option<usize>,
}

impl RdfFolder<'_> {
    fn connect(&mut self, resource: RdfResource) -> Result<()> {
        match resource {
            RdfResource::RdfId(rdf_id) => {
                let child_id = self.graph.find_id(rdf_id)? as i32;

                match self.node_id {
                    None => self.graph.root.push(child_id),
                    Some(ref parent_id) => match self.graph.nodes[*parent_id].children {
                        Some(ref mut children) => children.push(child_id),
                        None => unreachable!("Children should be initialized at this point")
                    }
                }
                Ok(())
            },
            RdfResource::Root => Err(anyhow!("Root node can't be a child of another node"))
        }
    }
}

#[derive(Debug)]
pub struct RdfGraph {
    root: ChildIds,
    nodes: Vec<RdfNode>,
}

impl From<RdfGraphIndexed> for RdfGraph {
    fn from(indexed: RdfGraphIndexed) -> Self {
        RdfGraph {
            root: indexed.root,
            nodes: indexed.nodes
        }
    }
}

pub fn parse_file(path: impl AsRef<Path>) -> Result<RdfGraph> {
    let xml = fs::read_to_string(&path).context("File reading error")?;
    let xml_root: Element = xml.parse().context("Parsing error")?;

    let mut graph = RdfGraphIndexed::new();

    for item in xml_root
        .children()
        .filter(|tag| matches!(tag.name(), "Description" | "BookmarkSeparator"))
    {
        let node = parse_description(&item)?;
        graph.add(node);
    }

    for sec in xml_root.children().filter(|tag| tag.name() == "Seq") {
        let parent_id = RdfResource::parse(
            sec.attr("RDF:about")
                .ok_or_else(|| anyhow!("RDF:about missing in Sequence tag"))?,
        )?;
        let mut folder = graph.make_folder(parent_id)?;

        for child in sec.children().filter(|tag| tag.name() == "li") {
            let child_id = RdfResource::parse(
                child
                    .attr("RDF:resource")
                    .ok_or_else(|| anyhow!("RDF:resource missing in 'li' tag"))?,
            )?;

            folder.connect(child_id)?;
        }
    }

    Ok(graph.into())
}

fn parse_description(item: &Element) -> Result<RdfNode> {
    fn attr_required<'a>(item: &'a Element, name: &str) -> Result<&'a str> {
        Ok(item
            .attr(name)
            .ok_or_else(|| anyhow!("{} attribute missing", name))?)
    }

    fn attr_owned(item: &Element, name: &str) -> Option<String> {
        item.attr(name).map(ToOwned::to_owned)
    }

    fn attr_time(item: &Element, name: &str) -> Option<UnresolvedTime> {
        item.attr(name)
            .map(|timestr| match UnresolvedTime::parse(timestr) {
                Ok(time) => Some(time),
                Err(err) => {
                    eprintln!("Timezone convertion error: {}", err);
                    None
                }
            })
            .flatten()
    }

    fn attr_icon(item: &Element, name: &str) -> Result<Option<UnresolvedIcon>> {
        Ok(item
            .attr(name)
            .map(|icostr| UnresolvedIcon::parse(icostr))
            .transpose()?
            .flatten())
    }

    let (type_, marked) = match attr_required(item, "NS1:type")? {
        "folder" => Ok((NodeType::Folder, false)),
        "" => Ok((NodeType::Page, false)),
        "marked" => Ok((NodeType::Page, true)),
        "file" => Ok((NodeType::File, false)),
        "note" => Ok((NodeType::Note, false)),
        "notex" => Ok((NodeType::Notex, false)),
        "separator" => Ok((NodeType::Separator, false)),
        "bookmark" => Ok((NodeType::Bookmark, false)),
        other => Err(anyhow!("Unknown node type: {}", other)),
    }?;

    let node = RdfNode {
        rdf_id: attr_required(item, "NS1:id")?.to_owned(),
        type_,
        title: attr_owned(item, "NS1:title"),
        created: attr_time(item, "NS1:create"),
        modified: attr_time(item, "NS1:modify"),
        source: attr_owned(item, "NS1:source"),
        icon: attr_icon(item, "NS1:icon")?,
        comment: attr_owned(item, "NS1:comment"),
        encoding: attr_owned(item, "NS1:chars"),
        marked,
        locked: matches!(item.attr("NS1:lock"), Some("true")),
        children: None,
    };

    Ok(node)
}
