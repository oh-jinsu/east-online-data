use std::collections::BTreeMap;

use serde::{ser::SerializeMap, Deserialize, Serialize};

#[derive(Debug, Hash, Ord, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl PartialOrd for Vector3 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.y.partial_cmp(&other.y) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.z.partial_cmp(&other.z) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.x.partial_cmp(&other.x)
    }
}

#[derive(Debug, Deserialize)]
pub struct Vector3Map<V>(BTreeMap<Vector3, V>)
where
    V: Serialize;

impl<V> Vector3Map<V>
where
    V: Serialize,
{
    pub fn insert(&mut self, key: Vector3, value: V) -> Option<V> {
        self.0.insert(key, value)
    }
}

impl<V> Serialize for Vector3Map<V>
where
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;

        for (k, v) in &self.0 {
            map.serialize_entry(k, v)?;
        }

        map.end()
    }
}

impl<T> Default for Vector3Map<T>
where
    T: Serialize,
{
    fn default() -> Self {
        Self(BTreeMap::default())
    }
}
