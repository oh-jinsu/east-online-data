use east_online_core::data::Tile;

pub fn generate_file(tile: &Tile, path: &str) {
    let id = match tile {
        Tile::Simple { id, .. } => id,
        Tile::Complex { id, .. } => id,
    };

    let path = std::env::current_dir()
        .unwrap()
        .join(format!("{}/{}.yml", path, id));

    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .unwrap();

    let writer = std::io::BufWriter::new(file);

    serde_yaml::to_writer(writer, tile).unwrap();
}
