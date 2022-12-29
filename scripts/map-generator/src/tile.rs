use serde::{Deserialize, Serialize};

use crate::common::{Vector3, Vector3Map};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub id: String,
    pub rotation: u8,
}

impl Tile {
    pub fn new(id: &str) -> Tile {
        Tile {
            id: id.to_string(),
            rotation: 0,
        }
    }

    pub fn with_rotation(id: &str, rotation: u8) -> Tile {
        Tile {
            id: id.to_string(),
            rotation,
        }
    }

    pub fn repeat(&self, x: i32, y: i32, z: i32) -> Vector3Map<Self> {
        let mut result = Vector3Map::default();

        for x in 0..x {
            for z in 0..z {
                for y in 0..y {
                    result.insert(Vector3 { x, y, z }, self.clone());
                }
            }
        }

        result
    }
}
