use crate::mapvec::TupleVecMapVisitor;

use core::{fmt, marker::PhantomData};
use serde::{de::Visitor, Deserialize, Deserializer};

struct OptionalTupleVecMapVisitor<K, V> {
	marker: PhantomData<Vec<(K, V)>>,
}

impl<K, V> OptionalTupleVecMapVisitor<K, V> {
	pub fn new() -> Self {
		Self {
			marker: PhantomData,
		}
	}
}

impl<'de, K, V> Visitor<'de> for OptionalTupleVecMapVisitor<K, V>
where
	K: Deserialize<'de>,
	V: Deserialize<'de>,
{
	type Value = Option<Vec<(K, V)>>;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("null or a map")
	}

	fn visit_none<E>(self) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(None)
	}

	fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
	where
		D: Deserializer<'de>,
	{
		Ok(Some(
			deserializer.deserialize_any(TupleVecMapVisitor::new())?,
		))
	}
}

pub fn deserialize<'de, K, V, D>(deserializer: D) -> Result<Option<Vec<(K, V)>>, D::Error>
where
	D: Deserializer<'de>,
	K: Deserialize<'de>,
	V: Deserialize<'de>,
{
	deserializer.deserialize_option(OptionalTupleVecMapVisitor::new())
}
