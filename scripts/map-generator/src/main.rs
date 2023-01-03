use std::{collections::BTreeMap, io};

use chrono::Utc;
use east_online_core::{
    data::{Map, Placable},
    models::Rotation,
};
use map_generator::{generate_file, generate_manifest};

fn main() -> io::Result<()> {
    let path = "../../maps";

    let version = format!("{}", Utc::now().format("%FT%XZ"));

    {
        let map = Map {
            id: "map_0000".to_string(),
            name: "map_1".to_string(),
            tiles: Placable {
                id: "tile_000".to_string(),
                rotation: Rotation::Down,
            }
            .repeat(64, 1, 64),
            objects: BTreeMap::new(),
            version: version.to_string(),
        };

        generate_file(&map, path)?;
    }

    {
        let map = Map {
            id: "map_0001".to_string(),
            name: "map_2".to_string(),
            tiles: Placable {
                id: "tile_000".to_string(),
                rotation: Rotation::Down,
            }
            .repeat(64, 1, 64),
            objects: BTreeMap::new(),
            version: version.to_string(),
        };

        generate_file(&map, path)?;
    }

    generate_manifest(path)?;

    println!("Maps generated successfully.");

    Ok(())
}
