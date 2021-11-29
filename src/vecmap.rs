use core::{cmp, fmt, marker::PhantomData};
use serde::{
	de::{MapAccess, Visitor},
	Deserialize, Deserializer, Serialize, Serializer,
};

pub(crate) struct TupleVecMapVisitor<K, V> {
	marker: PhantomData<Vec<(K, V)>>,
}

impl<K, V> TupleVecMapVisitor<K, V> {
	pub fn new() -> Self {
		Self {
			marker: PhantomData,
		}
	}
}

impl<'de, K, V> Visitor<'de> for TupleVecMapVisitor<K, V>
where
	K: Deserialize<'de>,
	V: Deserialize<'de>,
{
	type Value = Vec<(K, V)>;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("a map")
	}

	#[inline]
	fn visit_unit<E>(self) -> Result<Vec<(K, V)>, E> {
		Ok(Vec::new())
	}

	#[inline]
	fn visit_map<T>(self, mut access: T) -> Result<Vec<(K, V)>, T::Error>
	where
		T: MapAccess<'de>,
	{
		let mut values = Vec::with_capacity(cmp::min(access.size_hint().unwrap_or(0), 4069));

		while let Some((key, value)) = access.next_entry()? {
			values.push((key, value));
		}

		Ok(values)
	}
}

/// Serialize an array of `(K, V)` pairs as if it were a `HashMap<K, V>`.
///
/// In formats where dictionaries are ordered, this maintains the input data's order. Each pair is treated as a single
/// entry into the dictionary.
///
/// Behavior when duplicate keys are present in the data is unspecified and serializer-dependent. This function does
/// not check for duplicate keys and will not warn the serializer.
pub fn serialize<K, V, S>(data: &[(K, V)], serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
	K: Serialize,
	V: Serialize,
{
	serializer.collect_map(data.iter().map(|x| (&x.0, &x.1)))
}

/// Deserialize to a `Vec<(K, V)>` as if it were a `HashMap<K, V>`.
///
/// This directly deserializes into the returned vec with no intermediate allocation.
///
/// In formats where dictionaries are ordered, this maintains the input data's order.
pub fn deserialize<'de, K, V, D>(deserializer: D) -> Result<Vec<(K, V)>, D::Error>
where
	D: Deserializer<'de>,
	K: Deserialize<'de>,
	V: Deserialize<'de>,
{
	deserializer.deserialize_map(TupleVecMapVisitor::new())
}
