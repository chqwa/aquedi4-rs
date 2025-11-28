use serde::{Serialize, Deserialize};
use nom::{
    bytes::complete::take,
    number::complete::le_u32,
    multi::many_m_n,
    IResult,
    Parser,
};

use crate::types::StdString;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WorldChip {
    pub header: u32,
    pub tile_index: u32,
    pub locked: u32,
    pub graphic: u32,
    pub strings_count: u32, // 2

    pub name: StdString,
    pub unused_string: StdString,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WorldEventPage {
    pub start: u32,
    pub event_type: u32,
    pub graphic: u32,

    pub world_number: u32,
    pub pass_without_clear: u32,
    pub play_after_clear: u32,
    pub on_game_clear: u32,

    pub appearance_condition_world: u32, // 1
    pub appearance_condition_variable: u32, // dropdown
    pub appearance_condition_constant: u32, // spinner
    pub appearance_condition_comparison_content: u32, // small dropdown
    pub appearance_condition_total_score: u32,

    pub variation_setting_present: u32,
    pub variation_variable: u32,
    pub variation_constant: u32,

    pub strings_count: u32, // 2 - std::vector<std::string>

    pub world_name: StdString, // std::string
    pub start_stage: StdString, // std::string
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WorldEventBase {
    pub header: u32,
    pub placement_x: u32,
    pub placement_y: u32,

    pub strings_count: u32, // 1
    pub name: StdString,

    pub pages_count: u32,
    pub pages: Vec<WorldEventPage>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WorldMapFile {
    pub version: u32,
    pub settings_count: u32,

    pub horizontal_width: u32,
    pub vertical_width: u32,

    pub chunk_width: u32,
    pub chunk_pow: u32,

    pub initial_position_x: u32,
    pub initial_position_y: u32,

    pub background_index: u32,
    pub use_background: u32,

    pub strings_count: u32, // 2

    pub name: StdString,
    pub bg_path: StdString,

    pub tiles_types_count: u32,
    pub world_chip_data: Vec<WorldChip>,

    pub tiles_count: u32,
    pub map_chip_data: Vec<u32>,

    pub events_count: u32,
    pub event_data: Vec<WorldEventBase>,

    pub events_pal_count: u32,
    pub event_template_data: Vec<WorldEventBase>,
}

fn std_string(input: &[u8]) -> IResult<&[u8], StdString> {
    let (input, length) = le_u32(input)?;
    let (input, data) = if length > 1 {
        take(length)(input)?
    } else {
        take(0usize)(input)?
    };
    let std_string = StdString {
        length,
        data: data.to_vec(),
    };
    Ok((input, std_string))
}

fn world_chip(input: &[u8]) -> IResult<&[u8], WorldChip> {
    let (input, header) = le_u32(input)?;
    let (input, tile_index) = le_u32(input)?;
    let (input, locked) = le_u32(input)?;
    let (input, graphic) = le_u32(input)?;
    let (input, strings_count) = le_u32(input)?;
    let (input, name) = std_string(input)?;
    let (input, unused_string) = std_string(input)?;
    let world_chip = WorldChip {
        header, tile_index, locked, graphic, strings_count,
        name, unused_string,
    };
    Ok((input, world_chip))
}

fn world_event_page(input: &[u8]) -> IResult<&[u8], WorldEventPage> {
    let (input, start) = le_u32(input)?;
    let (input, event_type) = le_u32(input)?;
    let (input, graphic) = le_u32(input)?;

    let (input, world_number) = le_u32(input)?;
    let (input, pass_without_clear) = le_u32(input)?;
    let (input, play_after_clear) = le_u32(input)?;
    let (input, on_game_clear) = le_u32(input)?;

    let (input, appearance_condition_world) = le_u32(input)?; // 1
    let (input, appearance_condition_variable) = le_u32(input)?; // dropdown
    let (input, appearance_condition_constant) = le_u32(input)?; // spinner
    let (input, appearance_condition_comparison_content) = le_u32(input)?; // small dropdown
    let (input, appearance_condition_total_score) = le_u32(input)?;

    let (input, variation_setting_present) = le_u32(input)?;
    let (input, variation_variable) = le_u32(input)?;
    let (input, variation_constant) = le_u32(input)?;

    let (input, strings_count) = le_u32(input)?; // 2 - std::vector<std::string>

    let (input, world_name) = std_string(input)?; // std::string
    let (input, start_stage) = std_string(input)?; // std::string

    let world_event_page = WorldEventPage {
        start,
        event_type,
        graphic,
        world_number,
        pass_without_clear,
        play_after_clear,
        on_game_clear,
        appearance_condition_world,
        appearance_condition_variable,
        appearance_condition_constant,
        appearance_condition_comparison_content,
        appearance_condition_total_score,
        variation_setting_present,
        variation_variable,
        variation_constant,
        strings_count,
        world_name,
        start_stage,
    };

    Ok((input, world_event_page))
}

fn world_event_base(input: &[u8]) -> IResult<&[u8], WorldEventBase> {
    let (input, header) = le_u32(input)?;
    let (input, placement_x) = le_u32(input)?;
    let (input, placement_y) = le_u32(input)?;
    let (input, strings_count) = le_u32(input)?;
    let (input, name) = std_string(input)?;
    let (input, pages_count) = le_u32(input)?;
    let (input, pages) = many_m_n(0, pages_count.try_into().unwrap(), world_event_page).parse(input)?;
    let world_event_base = WorldEventBase {
        header,
        placement_x, placement_y,
        strings_count, name,
        pages_count, pages,
    };
    Ok((input, world_event_base))
}

fn world_map_file(input: &[u8]) -> IResult<&[u8], WorldMapFile> {
    let (input, version) = le_u32(input)?;
    let (input, settings_count) = le_u32(input)?;
    let (input, horizontal_width) = le_u32(input)?;
    let (input, vertical_width) = le_u32(input)?;
    let (input, chunk_width) = le_u32(input)?;
    let (input, chunk_pow) = le_u32(input)?;
    let (input, initial_position_x) = le_u32(input)?;
    let (input, initial_position_y) = le_u32(input)?;
    let (input, background_index) = le_u32(input)?;
    let (input, use_background) = le_u32(input)?;
    let (input, strings_count) = le_u32(input)?;
    let (input, name) = std_string(input)?;
    let (input, bg_path) = std_string(input)?;
    let (input, tiles_types_count) = le_u32(input)?;
    let (input, world_chip_data) = many_m_n(
        0, tiles_types_count.try_into().unwrap(), world_chip
    ).parse(input)?;
    let (input, tiles_count) = le_u32(input)?;
    let (input, map_chip_data) = many_m_n(
        0, tiles_count.try_into().unwrap(), le_u32
    ).parse(input)?;
    let (input, events_count) = le_u32(input)?;
    let (input, event_data) = many_m_n(
        0, tiles_count.try_into().unwrap(), world_event_base
    ).parse(input)?;
    let (input, events_pal_count) = le_u32(input)?;
    let (input, event_template_data) = many_m_n(
        0, tiles_count.try_into().unwrap(), world_event_base
    ).parse(input)?;
    let world_map_file = WorldMapFile {
        version, settings_count,
        horizontal_width, vertical_width,
        chunk_width, chunk_pow,
        initial_position_x, initial_position_y,
        background_index, use_background,
        strings_count, name, bg_path,
        tiles_types_count, world_chip_data,
        tiles_count, map_chip_data,
        events_count, event_data,
        events_pal_count, event_template_data,
    };
    Ok((input, world_map_file))
}

impl WorldMapFile {
    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        let result = world_map_file(data);
        match result {
            Ok((_, wm)) => Ok(wm),
            Err(_) => Err("Failed to parse data".to_string()),
        }
    }
}
