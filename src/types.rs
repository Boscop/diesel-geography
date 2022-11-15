//! Rust Types.

use std::io::prelude::*;
use std::convert::From;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::pg::Pg;
use diesel::backend::RawValue;
use postgis::ewkb::Point;
use crate::sql_types::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, PartialEq, FromSqlRow, AsExpression, Serialize, Deserialize)]
#[diesel(sql_type = Geography)]
pub struct GeogPoint {
	pub x: f64, // lon
	pub y: f64, // lat
	pub srid: Option<i32>,
}

impl From<Point> for GeogPoint {
	fn from(p: Point) -> Self {
		let Point { x, y, srid } = p;
		Self { x, y, srid }
	}
}
impl From<GeogPoint> for Point {
	fn from(p: GeogPoint) -> Self {
		let GeogPoint { x, y, srid } = p;
		Self { x, y, srid }
	}
}

impl FromSql<Geography, Pg> for GeogPoint {
	fn from_sql(bytes: RawValue<Pg>) -> deserialize::Result<Self> {
		use std::io::Cursor;
		use postgis::ewkb::EwkbRead;
		let bytes = bytes.as_bytes();
		let mut rdr = Cursor::new(bytes);
		Ok(Point::read_ewkb(&mut rdr)?.into())
	}
}

impl ToSql<Geography, Pg> for GeogPoint {
	fn to_sql<'a>(&'a self, out: &mut Output<'a, '_, Pg>) -> serialize::Result {
		use postgis::ewkb::{AsEwkbPoint, EwkbWrite};
		Point::from(*self).as_ewkb().write_ewkb(out)?;
		Ok(IsNull::No)
	}
}
