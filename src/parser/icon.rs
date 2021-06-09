use super::regex_utils::RegexExt;
use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(PartialEq, Eq, Debug)]
pub enum UnresolvedIcon {
    Url(String),
    InBucket(String),
    File(String),
}

impl UnresolvedIcon {
    pub(super) fn parse(rdf_icon: &str) -> Result<Option<UnresolvedIcon>> {
        lazy_static! {
            static ref HTTP: Regex = Regex::new(r"^https?://").unwrap();
            static ref RESOURCE: Regex =
                Regex::new(r"^resource://scrapbook/data/\d+/(.+)$").unwrap();
            static ref FILE: Regex = Regex::new(r"^file://(.+)$").unwrap();
        }

        if rdf_icon == "" {
            Ok(None)
        } else if rdf_icon.starts_with("moz-icon://") || HTTP.is_match(rdf_icon) {
            Ok(Some(UnresolvedIcon::Url(rdf_icon.to_owned())))
        } else if let Some(local_path) = RESOURCE.capture_first(rdf_icon) {
            Ok(Some(UnresolvedIcon::InBucket(local_path.into())))
        } else if let Some(path) = FILE.capture_first(rdf_icon) {
            Ok(Some(UnresolvedIcon::File(path.into())))
        } else {
            Err(anyhow!("Unexpected icon path: {}", rdf_icon))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_url(url: &str) {
        assert_eq!(
            UnresolvedIcon::parse(url).unwrap(),
            Some(UnresolvedIcon::Url(url.into()))
        );
    }

    #[test]
    fn moz_icon() {
        assert_url("moz-icon://Meta.pdf?size=16");
    }

    #[test]
    fn https() {
        assert_url("https://www.rust-lang.org/static/images/favicon.svg");
    }

    #[test]
    fn http() {
        assert_url("http://www.rust-lang.org/static/images/favicon.svg");
    }

    #[test]
    fn resource() {
        assert_eq!(
            UnresolvedIcon::parse("resource://scrapbook/data/20210317144721/favicon.ico").unwrap(),
            Some(UnresolvedIcon::InBucket("favicon.ico".into()))
        )
    }

    #[test]
    fn file() {
        assert_eq!(
            UnresolvedIcon::parse(
                "file:///home/lisk/Images/goretober_31__you_cannot_run_by_sinnykitt_debpa89.png"
            )
            .unwrap(),
            Some(UnresolvedIcon::File(
                "/home/lisk/Images/goretober_31__you_cannot_run_by_sinnykitt_debpa89.png".into()
            ))
        )
    }

    #[test]
    fn none() {
        assert_eq!(UnresolvedIcon::parse("").unwrap(), None)
    }
}
