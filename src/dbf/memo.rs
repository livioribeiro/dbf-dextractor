use std::io::{Error as IoError, Read, Seek, SeekFrom};

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

use super::Version;

pub struct MemoReader<R: Read + Seek> {
    reader: R,
    version: Version,
    block_size: u16,
}

impl<R: Read + Seek> MemoReader<R> {
    pub fn from_reader(mut reader: R, version: Version) -> Result<Self, IoError> {
        reader.seek(SeekFrom::Current(4))?;
        let block_size = match version {
            Version::DBase3 => 512,
            Version::DBase4 => match reader.read_u16::<LittleEndian>()? {
                0 => 512,
                v => v,
            },
            _ => {
                reader.seek(SeekFrom::Current(2))?; // reserved bytes
                reader.read_u16::<BigEndian>()?
            }
        };

        Ok(Self {
            reader,
            version,
            block_size,
        })
    }

    pub fn read_memo(&mut self, index: u32) -> Result<String, IoError> {
        let offset = index as u64 * self.block_size as u64;
        self.reader.seek(SeekFrom::Start(offset))?;

        match self.version {
            Version::DBase3 => {
                let mut acc = vec![0u8; self.block_size as usize];
                let mut buf = Vec::with_capacity(self.block_size as usize);
                loop {
                    let read = self.reader.read(&mut acc)?;
                    if read == 0 {
                        break;
                    }
                    buf.append(&mut acc);
                    acc.resize(self.block_size as usize, 0u8);
                    if buf.contains(&0x1a) {
                        break;
                    }
                }
                let end = buf
                    .iter()
                    .position(|b| *b == 0x1a)
                    .unwrap_or_else(|| buf.len());
                Ok(String::from_utf8_lossy(&buf[..end]).into_owned())
            }
            Version::DBase4 => {
                self.reader.seek(SeekFrom::Current(4))?; // reserved bytes
                let length = self.reader.read_u32::<LittleEndian>()?;
                let mut buf = vec![0u8; length as usize];
                self.reader.read_exact(&mut buf)?;
                Ok(String::from_utf8_lossy(&buf).into_owned())
            }
            Version::FoxBase | Version::VisualFoxPro | Version::FoxPro2 => {
                self.reader.seek(SeekFrom::Current(4))?; // reserved bytes
                let length = self.reader.read_u32::<BigEndian>()?;
                let mut buf = vec![0u8; length as usize];
                self.reader.read_exact(&mut buf)?;
                Ok(String::from_utf8_lossy(&buf).into_owned())
            }
        }
    }
}
