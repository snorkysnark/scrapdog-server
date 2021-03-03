use diesel::{ backend::Backend,
    serialize::{self, Output, ToSql},
    sql_types::Binary,
};
use std::io::Write;

#[derive(AsExpression, Debug)]
#[sql_type = "Binary"]
pub struct ChildIds(pub Vec<i32>);

impl<DB> ToSql<Binary, DB> for ChildIds
where
    DB: Backend,
    [u8]: ToSql<Binary, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        let bytes: &[u8] =
            bytemuck::try_cast_slice(&self.0[..]).expect("Can't convert &[i32] to blob");
        bytes.to_sql(out) }
}

impl From<Vec<i32>> for ChildIds {
    fn from(vec: Vec<i32>) -> Self {
        ChildIds(vec)
    }
}
