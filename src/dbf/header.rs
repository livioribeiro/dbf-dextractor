use std::convert::TryFrom;
use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use super::version::Version;
use crate::model::Date;

#[derive(Debug)]
pub struct Header {
    pub version: Version,
    pub last_update: Date,
    pub record_count: u32,
    pub header_length: usize,
    pub record_length: usize,
}

impl Header {
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Self, std::io::Error> {
        let version = Version::try_from(reader.read_u8()?).unwrap();
        let last_update = Date::new(
            reader.read_u8()? as u16 + 1900,
            reader.read_u8()?,
            reader.read_u8()?,
        );
        let record_count = reader.read_u32::<LittleEndian>()?;
        let header_length = reader.read_u16::<LittleEndian>()? as usize;
        let record_length = reader.read_u16::<LittleEndian>()? as usize;

        Ok(Self {
            version,
            last_update,
            record_count,
            header_length,
            record_length,
        })
    }
}
