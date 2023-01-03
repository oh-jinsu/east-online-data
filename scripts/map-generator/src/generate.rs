use std::{
    env, fs,
    io::{BufReader, BufWriter, Result},
};

use east_online_core::data::{Map, MapManifest, MapManifestItem};

pub fn generate_file(map: &Map, path: &str) -> Result<()> {
    let path = env::current_dir()?.join(format!("{}/{}.yml", path, map.id));

    let file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;

    let writer = BufWriter::new(file);

    serde_yaml::to_writer(writer, map).unwrap();

    Ok(())
}

pub fn generate_manifest(path: &str) -> Result<()> {
    let name = "manifest.yml";

    let path = env::current_dir()?.join(path);

    let files = fs::read_dir(&path)?;

    let items: Vec<MapManifestItem> = files
        .filter_map(|file| {
            let file = file.unwrap();

            if file.file_name() == name {
                return None;
            }

            let file = fs::OpenOptions::new().read(true).open(file.path()).unwrap();

            let reader = BufReader::new(file);

            let map: Map = serde_yaml::from_reader(reader).unwrap();

            let result = MapManifestItem {
                id: map.id,
                name: map.name,
                version: map.version,
            };

            Some(result)
        })
        .collect();

    let manifest = MapManifest { items };

    let path = path.join(name);

    let file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;

    let writer = BufWriter::new(file);

    serde_yaml::to_writer(writer, &manifest).unwrap();

    Ok(())
}
