use anyhow::anyhow;
use diesel::{
    backend::Backend,
    deserialize::{self, FromSql},
    serialize::{self, Output, ToSql},
    sql_types::Binary,
};
use serde::Serialize;
use std::io::Write;

#[derive(AsExpression, FromSqlRow, Serialize, Debug)]
#[sql_type = "Binary"]
#[serde(transparent)]
pub struct ChildIds(pub Vec<i32>);

impl<DB> ToSql<Binary, DB> for ChildIds
where
    DB: Backend,
    [u8]: ToSql<Binary, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        let bytes: &[u8] = bytemuck::try_cast_slice(&self.0[..])
            .map_err(|e| anyhow!("Can't convert &[i32] to blob: {:?}", e))?;
        bytes.to_sql(out)
    }
}

impl<DB> FromSql<Binary, DB> for ChildIds
where
    DB: Backend,
    Vec<u8>: FromSql<Binary, DB>,
{
    fn from_sql(raw: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let bytes: Vec<u8> = Vec::from_sql(raw)?;

        let ints: Vec<i32> = bytemuck::allocation::pod_collect_to_vec(&bytes[..]);
        Ok(ints.into())
    }
}

impl From<Vec<i32>> for ChildIds {
    fn from(vec: Vec<i32>) -> Self {
        ChildIds(vec)
    }
}
