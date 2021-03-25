use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(SqlIntegerEnum)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    let output = quote! {
        impl<DB> ::diesel::serialize::ToSql<::diesel::sql_types::Integer, DB> for #ident
        where
            DB: ::diesel::backend::Backend,
            i32: ::diesel::serialize::ToSql<::diesel::sql_types::Integer, DB>,
        {
            fn to_sql<W: ::std::io::Write>(&self, out: &mut ::diesel::serialize::Output<W, DB>) -> ::diesel::serialize::Result {
                (*self as i32).to_sql(out)
            }
        }

        impl<DB> ::diesel::deserialize::FromSql<::diesel::sql_types::Integer, DB> for #ident
        where
            DB: ::diesel::backend::Backend,
            i32: ::diesel::deserialize::FromSql<::diesel::sql_types::Integer, DB>,
        {
            fn from_sql(raw: Option<&DB::RawValue>) -> ::diesel::deserialize::Result<Self> {
                Ok(::std::convert::TryFrom::<i32>::try_from(::diesel::deserialize::FromSql::<::diesel::sql_types::Integer, DB>::from_sql(raw)?)?)
            }
        }
    };

    output.into()
}
