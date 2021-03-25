use num_enum::TryFromPrimitive;
use diesel::sql_types::Integer;
use scrapdog_derive::SqlIntegerEnum;

#[repr(i32)]
#[derive(SqlIntegerEnum, TryFromPrimitive, AsExpression, FromSqlRow, Debug, Copy, Clone)]
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
