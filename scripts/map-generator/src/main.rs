use std::collections::BTreeMap;

use chrono::Utc;
use goguryeo_core::data::{Map, Placable};
use map_generator::map::generate_file;

fn main() {
    let version = format!("{}", Utc::now().format("%FT%XZ"));

    {
        let map = Map {
            id: "map_0000".to_string(),
            name: "map_1".to_string(),
            tiles: Placable {
                id: "tile_000".to_string(),
                rotation: 0,
            }
            .repeat(64, 1, 64),
            objects: BTreeMap::new(),
            version: version.to_string(),
        };

        generate_file(&map, "../../maps");
    }

    {
        let map = Map {
            id: "map_0001".to_string(),
            name: "map_2".to_string(),
            tiles: Placable {
                id: "tile_000".to_string(),
                rotation: 0,
            }
            .repeat(64, 1, 64),
            objects: BTreeMap::new(),
            version: version.to_string(),
        };

        generate_file(&map, "../../maps");
    }

    println!("Maps generated successfully.")
}
