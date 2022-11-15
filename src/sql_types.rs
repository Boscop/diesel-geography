//! SQL Types.

#[derive(SqlType, QueryId)]
#[diesel(postgres_type(name = "geography"))]
pub struct Geography;
