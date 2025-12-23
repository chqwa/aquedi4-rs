use aquedi4_database::world_map::WorldMapFile;
use aquedi4_database::anime_set::AnimeSetFile;
use aquedi4_database::effect::EffectFile;
use aquedi4_database::bgm::BgmFile;
use aquedi4_database::sound::SoundFile;

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

#[test]
fn anim_set_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    data_path.push("tests/resources/AnimeSet.dat");

    let mut f = File::open(data_path)?;
    let mut buf = Vec::new();
    let _ = f.read_to_end(&mut buf)?;

    let bf = AnimeSetFile::from_bytes(&buf)?;
    assert_eq!(bf.magic, 1020);
    assert_eq!(bf.count, 3);
    assert_eq!(bf.elements[0].invincibility_offset, 7);

    Ok(())
}

#[test]
fn effect_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    data_path.push("tests/resources/Effect.dat");

    let mut f = File::open(data_path)?;
    let mut buf = Vec::new();
    let _ = f.read_to_end(&mut buf)?;

    let bf = EffectFile::from_bytes(&buf)?;
    assert_eq!(bf.magic, 1020);
    assert_eq!(bf.count, 18);
    assert_eq!(bf.elements[17].path.length, 23);
    assert_eq!(bf.elements[17].animations[0].start, 0);
    assert_eq!(bf.elements[17].animations[0].end, 1);

    Ok(())
}

#[test]
fn bgm_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    data_path.push("tests/resources/Bgm.dat");

    let mut f = File::open(data_path)?;
    let mut buf = Vec::new();
    let _ = f.read_to_end(&mut buf)?;

    let bf = BgmFile::from_bytes(&buf)?;
    assert_eq!(bf.magic, 1020);
    assert_eq!(bf.count, 27);
    assert_eq!(bf.count as usize, bf.elements.len());

    Ok(())
}

#[test]
fn sound_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    data_path.push("tests/resources/Sound.dat");

    let mut f = File::open(data_path)?;
    let mut buf = Vec::new();
    let _ = f.read_to_end(&mut buf)?;

    let bf = SoundFile::from_bytes(&buf)?;
    assert_eq!(bf.magic, 1020);
    assert_eq!(bf.count, 41);
    assert_eq!(bf.count as usize, bf.elements.len());

    Ok(())
}

