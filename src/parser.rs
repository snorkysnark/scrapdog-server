use anyhow::{anyhow, Context, Result};
use minidom::Element;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug)]
enum NodeType {
    Folder,
    Page,
    File,
    Note,
    Notex,
    Separator,
}

#[derive(Debug)]
struct RdfNodeData {
    rdf_id: String,
    type_: NodeType,
    title: Option<String>,
    created: Option<String>,
    modified: Option<String>,
    source: Option<String>,
    icon: Option<String>,
    comment: Option<String>,
    encoding: Option<String>,
    marked: bool,
    locked: bool,
    children: Option<Vec<i32>>,
}

#[derive(Debug, Default)]
struct RdfNode {
    data: Option<RdfNodeData>,
    children: Option<Vec<i32>>,
}

impl RdfNode {
    pub fn root() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, Copy)]
enum RdfResource<'a> {
    Root,
    RdfId(&'a str),
}

struct RdfGraph {
    nodes: Vec<RdfNode>,
    node_ids: HashMap<String, usize>
}

impl RdfGraph {
    pub fn new() -> Self {
        RdfGraph {
            nodes: vec![RdfNode::root()],
            node_ids: HashMap::new()
        }
    }

    pub fn add(&mut self, node: RdfNode) {
        if let Some(ref data) = node.data
        {
            self.node_ids.insert(data.rdf_id.clone(), self.nodes.len());
        }
        self.nodes.push(node);
    }

    fn find_id(&self, resource: RdfResource) -> Result<usize> {
        match resource {
            RdfResource::Root => Ok(0),
            RdfResource::RdfId(rdf_id) => match self.node_ids.get(rdf_id) {
                Some(id) => Ok(*id),
                None => Err(anyhow!("Resource id {} not found", rdf_id))
            }
        }
    }

    pub fn make_folder(&mut self, resource: RdfResource) -> Result<RdfFolder> {
        let id = self.find_id(resource)?;
        self.nodes[id].children = Some(Vec::new());

        Ok(RdfFolder {
            graph: self,
            node_id: id
        })
    }
}

struct RdfFolder<'a> {
    graph: &'a mut RdfGraph,
    node_id: usize
}

impl RdfFolder<'_> {
    pub fn connect(&mut self, resource: RdfResource) -> Result<()> {
        let child_id = self.graph.find_id(resource)?;
        match &mut self.graph.nodes[self.node_id].children {
            Some(ref mut children) => children.push(child_id as i32),
            None => unreachable!("Children should be initialized at this point")
        }

        Ok(())
    }
}

pub fn parse_file(path: impl AsRef<Path>) -> Result<()> {
    let xml = fs::read_to_string(&path).context("File reading error")?;
    let xml_root: Element = xml.parse().context("Parsing error")?;

    let mut graph = RdfGraph::new();

    for item in xml_root
        .children()
        .filter(|tag| matches!(tag.name(), "Description" | "BookmarkSeparator"))
    {
        let node = parse_description(&item)?;

        graph.add(RdfNode {
            data: Some(node),
            children: None,
        });
    }

    for sec in xml_root.children().filter(|tag| tag.name() == "Seq") {
        let parent_id = parse_rdf_resource(
            sec.attr("RDF:about")
                .ok_or_else(|| anyhow!("RDF:about missing in Sequence tag"))?,
        )?;
        let mut folder = graph.make_folder(parent_id)?;

        for child in sec.children().filter(|tag| tag.name() == "li") {
            let child_id = parse_rdf_resource(
                child
                    .attr("RDF:resource")
                    .ok_or_else(|| anyhow!("RDF:resource missing in 'li' tag"))?,
            )?;

            folder.connect(child_id)?;
        }
    }

    for (id, node) in graph.nodes.iter().enumerate() {
        println!("{}:{:#?}\n", id, node);
    }

    Ok(())
}

fn parse_rdf_resource(resource: &str) -> Result<RdfResource> {
    const PREFIX: &'static str = "urn:scrapbook:item";

    if resource == "urn:scrapbook:root" {
        Ok(RdfResource::Root)
    } else if resource.starts_with(PREFIX) {
        Ok(RdfResource::RdfId(&resource[PREFIX.len()..]))
    } else {
        Err(anyhow!("Unexpected RDF resource string: {}", resource))
    }
}

fn parse_description(item: &Element) -> Result<RdfNodeData> {
    let (type_, marked) = match item
        .attr("NS1:type")
        .ok_or_else(|| anyhow!("Type attribute missing"))?
    {
        "folder" => Ok((NodeType::Folder, false)),
        "" => Ok((NodeType::Page, false)),
        "marked" => Ok((NodeType::Page, true)),
        "file" => Ok((NodeType::File, false)),
        "note" => Ok((NodeType::Note, false)),
        "notex" => Ok((NodeType::Notex, false)),
        "separator" => Ok((NodeType::Separator, false)),
        other => Err(anyhow!("Unknown node type: {}", other)),
    }?;

    Ok(RdfNodeData {
        rdf_id: item
            .attr("NS1:id")
            .ok_or_else(|| anyhow!("Id attribute missing"))?
            .to_owned(),
        type_,
        title: item.attr("NS1:title").map(ToOwned::to_owned),
        created: item.attr("NS1:create").map(ToOwned::to_owned),
        modified: item.attr("NS1:modify").map(ToOwned::to_owned),
        source: item.attr("NS1:source").map(ToOwned::to_owned),
        icon: item.attr("NS1:icon").map(ToOwned::to_owned),
        comment: item.attr("NS1:comment").map(ToOwned::to_owned),
        encoding: item.attr("NS1:chars").map(ToOwned::to_owned),
        marked,
        locked: matches!(item.attr("NS1:lock"), Some("true")),
        children: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rdf_resource_root() {
        assert!(matches!(
            parse_rdf_resource("urn:scrapbook:root"),
            Ok(RdfResource::Root)
        ));
    }

    #[test]
    fn rdf_resource_id() {
        assert!(matches!(
            parse_rdf_resource("urn:scrapbook:item20201221133741"),
            Ok(RdfResource::RdfId("20201221133741"))
        ));
    }

    #[test]
    fn rdf_resource_err() {
        assert!(parse_rdf_resource("urn:scrapbook:bullshit").is_err());
    }
}
