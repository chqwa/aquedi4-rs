use serde::{Serialize, Deserialize};
use nom::{
    number::complete::{le_u8, le_u16, le_u32},
    multi::many_m_n,
    IResult,
    Parser,
};

use crate::types::{StdString, std_string};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AnimationFrame {
    pub header: u32,
    pub frame_index: u32,
    pub display_time: u32,
    pub exec_commands: u32,
    pub unknown2: u32,
}

fn anim_frame(input: &[u8]) -> IResult<&[u8], AnimationFrame> {
    let (input, header) = le_u32(input)?;
    let (input, frame_index) = le_u32(input)?;
    let (input, display_time) = le_u32(input)?;
    let (input, exec_commands) = le_u32(input)?;
    let (input, unknown2) = le_u32(input)?;

    Ok((input, AnimationFrame {
        header, frame_index, display_time, exec_commands, unknown2,
    }))
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Animation {
    pub header: u32,
    pub sample_list_index: u16,
    pub sample_index: u8,
    pub frame_start: u16,
    pub strings_count: u32,

    pub name: StdString,

    pub anim_frame_count: u32,
    pub anim_frames: Vec<AnimationFrame>,
}

fn animation(input: &[u8]) -> IResult<&[u8], Animation> {
    let (input, header) = le_u32(input)?;
    let (input, sample_list_index) = le_u16(input)?;
    let (input, sample_index) = le_u8(input)?;
    let (input, frame_start) = le_u16(input)?;
    let (input, strings_count) = le_u32(input)?;
    let (input, name) = std_string(input)?;
    let (input, anim_frame_count) = le_u32(input)?;
    let (input, anim_frames) = many_m_n(
        0, anim_frame_count.try_into().unwrap(), anim_frame
    ).parse(input)?;

    Ok((input, Animation {
        header, sample_list_index, sample_index, frame_start,
        strings_count, name,
        anim_frame_count, anim_frames,
    }))
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AnimeSet {
    pub header: u32,
    pub invincibility_offset: u32,
    pub block_offset: u32,
    pub flying_offset: u32,
    pub strings_count: u32,

    pub name: StdString,

    pub animation_count: u32,
    pub animations: Vec<Animation>,
}

fn animation_set(input: &[u8]) -> IResult<&[u8], AnimeSet> {
    let (input, header) = le_u32(input)?;
    let (input, invincibility_offset) = le_u32(input)?;
    let (input, block_offset) = le_u32(input)?;
    let (input, flying_offset) = le_u32(input)?;
    let (input, strings_count) = le_u32(input)?;
    let (input, name) = std_string(input)?;
    let (input, animation_count) = le_u32(input)?;
    let (input, animations) = many_m_n(
        0, animation_count.try_into().unwrap(), animation
    ).parse(input)?;

    Ok((input, AnimeSet {
        header, invincibility_offset, block_offset, flying_offset,
        strings_count, name, animation_count, animations,
    }))
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AnimeSetFile {
    pub magic: u32,
    pub count: u32,
    pub elements: Vec<AnimeSet>,
}

fn anime_set_file(input: &[u8]) -> IResult<&[u8], AnimeSetFile> {
    let (input, magic) = le_u32(input)?;
    let (input, count) = le_u32(input)?;
    let (input, elements) = many_m_n(
        0, count.try_into().unwrap(), animation_set
    ).parse(input)?;

    Ok((input, AnimeSetFile {
        magic, count, elements,
    }))
}

impl AnimeSetFile {
    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        let result = anime_set_file(data);
        match result {
            Ok((_, s)) => Ok(s),
            Err(_) => Err("Failed to parse data".to_string()),
        }
    }
}
