use chrono::Utc;
use map_generator::{Map, Tile};

fn main() {
    let version = format!("{}", Utc::now().format("%FT%XZ"));

    {
        let map = Map::builder()
            .set_id("map_0000")
            .set_name("map_1")
            .set_tiles(Tile::new("tile_000").repeat(64, 1, 64))
            .build(&version);

        map.generate_file("../../maps");
    }

    {
        let map = Map::builder()
            .set_id("map_0001")
            .set_name("map_2")
            .set_tiles(Tile::new("tile_000").repeat(32, 1, 32))
            .build(&version);

        map.generate_file("../../maps");
    }

    println!("Maps generated successfully.")
}
