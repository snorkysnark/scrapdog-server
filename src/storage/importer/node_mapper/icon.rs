use crate::parser::UnresolvedIcon;

pub trait ConvertIconPath {
    fn into_icon_path(self, node_id: i32) -> String;
}

impl ConvertIconPath for UnresolvedIcon {
    fn into_icon_path(self, node_id: i32) -> String {
        match self {
            UnresolvedIcon::Url(url) => url,
            UnresolvedIcon::InBucket(local_path) => {
                format!("scrapdog://{}/{}", node_id, local_path)
            }
            UnresolvedIcon::File(path) => format!("file://{}", path),
        }
    }
}
