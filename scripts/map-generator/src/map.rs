use east_online_core::data::Map;

pub fn generate_file(map: &Map, path: &str) {
    let path = std::env::current_dir()
        .unwrap()
        .join(format!("{}/{}.yml", path, map.id));

    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .unwrap();

    let writer = std::io::BufWriter::new(file);

    serde_yaml::to_writer(writer, map).unwrap();
}
