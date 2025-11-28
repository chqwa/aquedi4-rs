use std::fmt;
use serde::{Serialize, Deserialize};
use encoding_rs::SHIFT_JIS;

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

