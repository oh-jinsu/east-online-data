use serde::{Deserialize, Serialize};

use crate::{common::Vector3Map, object::Object, tile::Tile};

#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
    pub id: String,
    pub version: String,
    pub name: String,
    pub tiles: Vector3Map<Tile>,
    pub objects: Vector3Map<Object>,
}

impl Map {
    pub fn builder() -> MapBuilder {
        MapBuilder::default()
    }

    pub fn generate_file(&self, path: &str) {
        let path = std::env::current_dir()
            .unwrap()
            .join(format!("{}/{}.yml", path, self.id));

        let file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .unwrap();

        let writer = std::io::BufWriter::new(file);

        serde_yaml::to_writer(writer, self).unwrap();
    }
}

#[derive(Default)]
pub struct MapBuilder {
    id: String,
    name: String,
    tiles: Vector3Map<Tile>,
    objects: Vector3Map<Object>,
}

impl MapBuilder {
    pub fn set_id(mut self, value: &str) -> Self {
        self.id = value.to_string();

        self
    }

    pub fn set_name(mut self, value: &str) -> Self {
        self.name = value.to_string();

        self
    }

    pub fn set_tiles(mut self, value: Vector3Map<Tile>) -> Self {
        self.tiles = value;

        self
    }

    pub fn set_objects(mut self, value: Vector3Map<Object>) -> Self {
        self.objects = value;

        self
    }

    pub fn build(self, version: &str) -> Map {
        Map {
            id: self.id,
            version: version.to_string(),
            name: self.name,
            tiles: self.tiles,
            objects: self.objects,
        }
    }
}
