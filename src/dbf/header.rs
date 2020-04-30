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
        let last_updated = (reader.read_u8()?, reader.read_u8()?, reader.read_u8()?);
        let record_count = reader.read_u32::<LittleEndian>()?;
        let header_length = reader.read_u16::<LittleEndian>()? as usize;
        let record_length = reader.read_u16::<LittleEndian>()? as usize;

        dbg!(version);

        Ok(Self {
            version,
            last_update: last_updated.into(),
            record_count,
            header_length,
            record_length,
        })
    }
}
