use std::io::{Read, Seek, SeekFrom, Error as IoError};
use byteorder::{ReadBytesExt, LittleEndian};

pub struct MemoReader<R: Read + Seek> {
    reader: R,
    block_size: u16,
}

impl<R: Read + Seek> MemoReader<R> {
    pub fn from_reader(mut reader: R) -> Result<Self, IoError> {
        reader.seek(SeekFrom::Start(4))?;
        let block_size = reader.read_u16::<LittleEndian>()?;

        Ok(Self { reader, block_size })
    }

    pub fn read_memo(&mut self, index: u32) -> Result<String, IoError> {
        let offset = index as u64 * self.block_size as u64;
        self.reader.seek(SeekFrom::Start(offset))?;
        let _type = self.reader.read_u32::<LittleEndian>()?;
        let length = self.reader.read_u32::<LittleEndian>()?;
        let mut buf = vec![0u8; length as usize];
        self.reader.read_exact(&mut buf)?;
        Ok(String::from_utf8(buf).expect("memo string parse"))
    }
}