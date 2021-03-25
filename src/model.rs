use diesel::{
    backend::Backend,
    deserialize::{self, FromSql},
    serialize::{self, Output, ToSql},
    sql_types::Integer,
};
use serde::Serialize;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::io::Write;

#[repr(i32)]
#[derive(TryFromPrimitive, AsExpression, FromSqlRow, Serialize, Debug, Clone, Copy)]
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

impl<DB> ToSql<Integer, DB> for NodeType
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        (*self as i32).to_sql(out)
    }
}

impl<DB> FromSql<Integer, DB> for NodeType
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(raw: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        Ok(Self::try_from(i32::from_sql(raw)?)?)
    }
}
