use goguryeo_core::data::{Tile, TileTexture};
use tile_generator::generate_file;

fn main() {
    let path = "../../tiles";

    {
        let tile = Tile::Simple {
            id: "tile_000".to_string(),
            texture: TileTexture {
                top: 0,
                bottom: 0,
                left: 0,
                right: 0,
                front: 0,
                back: 0,
            },
        };

        generate_file(&tile, path);
    }

    {
        let tile = Tile::Simple {
            id: "tile_001".to_string(),
            texture: TileTexture {
                top: 1,
                bottom: 1,
                left: 1,
                right: 1,
                front: 1,
                back: 1,
            },
        };

        generate_file(&tile, path);
    }

    {
        let tile = Tile::Simple {
            id: "tile_002".to_string(),
            texture: TileTexture {
                top: 2,
                bottom: 2,
                left: 2,
                right: 2,
                front: 2,
                back: 2,
            },
        };

        generate_file(&tile, path);
    }

    {
        let tile = Tile::Simple {
            id: "tile_003".to_string(),
            texture: TileTexture {
                top: 3,
                bottom: 3,
                left: 3,
                right: 3,
                front: 3,
                back: 3,
            },
        };

        generate_file(&tile, path);
    }

    println!("Tiles generated successfully.")
}
