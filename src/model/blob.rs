use anyhow::anyhow;
use diesel::{
    backend::Backend,
    deserialize::{self, FromSql, FromSqlRow},
    serialize::{self, Output, ToSql},
    sql_types::Binary,
};
use std::io::Write;

#[derive(AsExpression, FromSqlRow, Debug)]
#[sql_type = "Binary"]
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

        if bytes.len() == 0 {
            Ok(Vec::new().into())
        } else {
            let ints: Vec<i32> = bytemuck::try_cast_slice(&bytes[..])
                .map_err(|e| anyhow!("Can't convert blob to Vec<i32>: {:?}", e))?
                .iter()
                .cloned()
                .collect();

            Ok(ints.into())
        }
    }
}

impl From<Vec<i32>> for ChildIds {
    fn from(vec: Vec<i32>) -> Self {
        ChildIds(vec)
    }
}
