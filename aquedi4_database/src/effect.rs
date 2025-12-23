use serde::{Serialize, Deserialize};
use nom::{
    number::complete::le_u32,
    multi::many_m_n,
    IResult,
    Parser,
};

use crate::types::{StdString, std_string};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EffectAnimation {
    pub header: u32,
    pub start: u32,
    pub end: u32,
    pub unknown: u32,
}

fn effect_animation(input: &[u8]) -> IResult<&[u8], EffectAnimation> {
    let (input, header) = le_u32(input)?;
    let (input, start) = le_u32(input)?;
    let (input, end) = le_u32(input)?;
    let (input, unknown) = le_u32(input)?;

    Ok((input, EffectAnimation {
        header, start, end, unknown
    }))
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Effect {
    pub header: u32,
    pub is_name_same_path: u32,
    pub width: u32,
    pub height: u32,
    pub is_giant: u32,
    pub strings_count: u32,  // always 2?
    pub name: StdString,
    pub path: StdString,
    pub animation_count: u32,
    pub animations: Vec<EffectAnimation>,
}

fn effect(input: &[u8]) -> IResult<&[u8], Effect> {
    let (input, header) = le_u32(input)?;
    let (input, is_name_same_path) = le_u32(input)?;
    let (input, width) = le_u32(input)?;
    let (input, height) = le_u32(input)?;
    let (input, is_giant) = le_u32(input)?;
    let (input, strings_count) = le_u32(input)?;
    let (input, name) = std_string(input)?;
    let (input, path) = std_string(input)?;
    let (input, animation_count) = le_u32(input)?;
    let (input, animations) = many_m_n(
        0, animation_count.try_into().unwrap(), effect_animation
    ).parse(input)?;

    Ok((input, Effect {
        header, is_name_same_path, width, height, is_giant,
        strings_count, name, path, animation_count, animations,
    }))
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EffectFile {
    pub magic: u32,
    pub count: u32,
    pub elements: Vec<Effect>,
}

fn effect_file(input: &[u8]) -> IResult<&[u8], EffectFile> {
    let (input, magic) = le_u32(input)?;
    let (input, count) = le_u32(input)?;
    let (input, elements) = many_m_n(
        0, count.try_into().unwrap(), effect
    ).parse(input)?;

    Ok((input, EffectFile {
        magic, count, elements,
    }))
}

impl EffectFile {
    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        let result = effect_file(data);
        match result {
            Ok((_, s)) => Ok(s),
            Err(_) => Err("Failed to parse data".to_string()),
        }
    }
}
