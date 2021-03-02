use diesel::{
    backend::Backend,
    serialize::{self, Output, ToSql},
    sql_types::Integer,
};
use std::io::Write;

#[repr(i32)]
#[derive(AsExpression, Debug, Clone, Copy)]
#[sql_type = "Integer"]
pub enum NodeType {
    Folder = 0,
    Page = 1,
    File = 2,
    Note = 3,
    Notex = 4,
    Separator = 5,
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
