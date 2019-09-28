//! SQL Types.

#[derive(SqlType, QueryId)]
#[postgres(type_name = "geography")]
pub struct Geography;
