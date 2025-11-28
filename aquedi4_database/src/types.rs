use std::fmt;
use serde::{Serialize, Deserialize};
use encoding_rs::SHIFT_JIS;
use nom::{
    bytes::complete::take,
    number::complete::le_u32,
    IResult,
};


#[derive(Default, Clone, Serialize, Deserialize)]
pub struct StdString {
    pub length: u32,
    pub data: Vec<u8>,
}

impl fmt::Debug for StdString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (cow, _, _) = SHIFT_JIS.decode(&self.data);
        f.debug_struct("StdString")
            .field("length", &self.length)
            .field("data", &cow)
            .finish()
    }
}

pub fn std_string(input: &[u8]) -> IResult<&[u8], StdString> {
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
