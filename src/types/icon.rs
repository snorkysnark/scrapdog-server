use anyhow::{Result, anyhow};

#[derive(PartialEq, Eq, Debug)]
pub enum UnresolvedIcon {
    Url(String),
    InBucket(String),
    File(String),
}

impl UnresolvedIcon {
    pub fn resolve(self, bucket_id: Option<i32>) -> Result<String> {
        match self {
            UnresolvedIcon::Url(url) => Ok(url),
            UnresolvedIcon::InBucket(local_path) => match bucket_id {
                Some(bid) => Ok(format!("{}/{}", bid, local_path)),
                None => Err(anyhow!("Icon path {} cannot be resolved: node has no bucket id", local_path))
            },
            UnresolvedIcon::File(path) => Ok(format!("file://{}", path)),
        }
    }
}
