use aquedi4_database::world_map::WorldMapFile;

use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;

#[test]
fn world_map_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    data_path.push("tests/resources/WorldMap.dat");

    let mut f = File::open(data_path)?;
    let mut buf = Vec::new();
    let _ = f.read_to_end(&mut buf)?;

    let wm = WorldMapFile::from_bytes(&buf)?;
    assert_eq!(wm.version, 1020);
    assert_eq!(wm.settings_count, 8);
    assert_eq!(wm.horizontal_width, 20);
    assert_eq!(wm.vertical_width, 15);

    Ok(())
}
