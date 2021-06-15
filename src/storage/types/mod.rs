mod blob;

use diesel::sql_types::Integer;
use num_enum::TryFromPrimitive;
use scrapdog_derive::SqlIntegerEnum;
use serde_repr::Serialize_repr;

type ParserNodeType = crate::parser::NodeType;

pub use blob::BlobVecI32;

#[repr(i32)]
#[derive(
    SqlIntegerEnum, TryFromPrimitive, AsExpression, FromSqlRow, Serialize_repr, Debug, Copy, Clone,
)]
#[sql_type = "Integer"]
pub enum NodeType {
    Folder = 0,
    Page = 1,
    File = 2,
    Note = 3,
    Notex = 4,
    Separator = 5,
    Bookmark = 6,
}

impl From<ParserNodeType> for NodeType {
    fn from(type_: ParserNodeType) -> Self {
        match type_ {
            ParserNodeType::Folder => NodeType::Folder,
            ParserNodeType::Page => NodeType::Page,
            ParserNodeType::File => NodeType::File,
            ParserNodeType::Note => NodeType::Note,
            ParserNodeType::Notex => NodeType::Notex,
            ParserNodeType::Separator => NodeType::Separator,
            ParserNodeType::Bookmark => NodeType::Bookmark,
        }
    }
}
