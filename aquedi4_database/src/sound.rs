use serde::{Serialize, Deserialize};
use nom::{
    number::complete::le_u32,
    multi::many_m_n,
    IResult,
    Parser,
};

use crate::types::{StdString, std_string};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SoundElement {
    pub header: u32,
    pub is_name_same_path: u32,
    pub strings_count: u32,  // always 2?

    pub name: StdString,
    pub path: StdString,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SoundFile {
    pub magic: u32,
    pub count: u32,
    pub elements: Vec<SoundElement>,
}

fn bgm_element(input: &[u8]) -> IResult<&[u8], SoundElement> {
    let (input, header) = le_u32(input)?;
    let (input, is_name_same_path) = le_u32(input)?;
    let (input, strings_count) = le_u32(input)?;
    let (input, name) = std_string(input)?;
    let (input, path) = std_string(input)?;


    Ok((input, SoundElement {
        header, is_name_same_path, strings_count, name, path,
    }))
}

fn bgm_file(input: &[u8]) -> IResult<&[u8], SoundFile> {
    let (input, magic) = le_u32(input)?;
    let (input, count) = le_u32(input)?;
    let (input, elements) = many_m_n(
        0, count.try_into().unwrap(), bgm_element
    ).parse(input)?;

    Ok((input, SoundFile {
        magic, count, elements,
    }))
}

impl SoundFile {
    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        let result = bgm_file(data);
        match result {
            Ok((_, s)) => Ok(s),
            Err(_) => Err("Failed to parse data".to_string()),
        }
    }
}
