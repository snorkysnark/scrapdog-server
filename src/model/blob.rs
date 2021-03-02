use diesel::{
    backend::Backend,
    serialize::{self, Output, ToSql},
    sql_types::Binary,
};
use std::io::Write;

#[derive(AsExpression, Debug)]
#[sql_type = "Binary"]
pub struct ChildIds(Vec<i32>);

impl<DB> ToSql<Binary, DB> for ChildIds
where
    DB: Backend,
    Vec<u8>: ToSql<Binary, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        let bytes: Vec<u8> = bincode::serialize(&self.0).expect("Can't serialize Vec<i32> to blob");
        bytes.to_sql(out)
    }
}
