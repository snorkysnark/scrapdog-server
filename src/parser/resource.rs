use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;
use super::regex_utils::RegexExt;

#[derive(Debug, Clone, Copy)]
pub enum RdfResource<'a> {
    Root,
    RdfId(&'a str),
}

impl<'a> RdfResource<'a> {
    pub fn parse(resource: &'a str) -> Result<Self> {
        lazy_static! {
            static ref ITEM_REGEX: Regex = Regex::new(r"urn:scrapbook:item(\d+)$").unwrap();
        }

        if resource == "urn:scrapbook:root" {
            Ok(RdfResource::Root)
        } else if let Some(rdf_id) = ITEM_REGEX.capture_first(resource) {
            Ok(RdfResource::RdfId(rdf_id))
        } else {
            Err(anyhow!("Unexpected RDF resource string: {}", resource))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rdf_resource_root() {
        assert!(matches!(
            RdfResource::parse("urn:scrapbook:root"),
            Ok(RdfResource::Root)
        ));
    }

    #[test]
    fn rdf_resource_id() {
        assert!(matches!(
            RdfResource::parse("urn:scrapbook:item20201221133741"),
            Ok(RdfResource::RdfId("20201221133741"))
        ));
    }

    #[test]
    fn rdf_resource_err() {
        assert!(RdfResource::parse("urn:scrapbook:bullshit").is_err());
    }
}
