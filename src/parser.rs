use anyhow::{anyhow, Context, Result};
use minidom::Element;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

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
struct RdfNode {
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
}

pub fn parse_file(path: impl AsRef<Path>) -> Result<()> {
    let xml = fs::read_to_string(&path).context("File reading error")?;
    let root: Element = xml.parse().context("Parsing error")?;

    let nodes: Vec<RdfNode> = Vec::new();
    let node_ids: HashMap<&str,usize> = HashMap::new();

    for item in root
        .children()
        .filter(|tag| matches!(tag.name(), "Description" | "BookmarkSeparator"))
    {
    }

    Ok(())
}

fn parse_description(item: &Element) -> Result<RdfNode> {
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

    Ok(RdfNode {
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
    })
}
