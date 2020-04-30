use std::error::Error;
use std::io::{Read, Seek, SeekFrom};
use std::marker::PhantomData;

use serde::de::DeserializeOwned;

use super::memo::MemoReader;
use super::{FieldInfo, Header};
use crate::deserialize::DbfDeserializer;
use crate::error::UnsupportedFieldTypeError;

pub struct DbfReader<R: Read + Seek> {
    reader: R,
    header: Header,
    deserializer: DbfDeserializer<R>,
}

fn read_field_info(buf: &[u8]) -> Result<Vec<FieldInfo>, UnsupportedFieldTypeError> {
    buf.chunks(32)
        .scan(1usize, |acc_offset, info| {
            if info.len() < 32 {
                return None;
            }

            let name_bytes = &info[0..=10];
            let field_type = info[11];
            let length = info[16] as usize;
            let offset = *acc_offset;
            *acc_offset += length;

            let field = match FieldInfo::new(name_bytes, field_type, length, offset) {
                Ok(value) => value,
                Err(e) => return Some(Err(e)),
            };

            Some(Ok(field))
        })
        .collect()
}

impl<'a, R: Read + Seek> DbfReader<R> {
    pub fn from_reader(mut reader: R, memo_reader: Option<R>) -> Result<Self, Box<dyn Error>> {
        let header = Header::from_reader(&mut reader)?;

        reader.seek(SeekFrom::Current(20))?;

        let fields = {
            let mut buf = vec![0u8; header.header_length - 32];
            reader.read_exact(&mut buf)?;
            read_field_info(&buf)?
        };

        let record_length = header.record_length;
        let memo_reader_opt = memo_reader
            .map(|r| MemoReader::from_reader(r, header.version))
            .transpose()?;
        let deserializer = DbfDeserializer::new(fields, record_length, memo_reader_opt);

        Ok(Self {
            reader,
            header,
            deserializer,
        })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    fn next_record(&mut self) -> Result<Option<()>, std::io::Error> {
        loop {
            self.deserializer.reset_buffer();

            let buffer = self.deserializer.buffer();
            let n_read = self.reader.read(buffer)?;
            if n_read <= 1 {
                return Ok(None);
            }

            let delete_mark = buffer[0] as char;
            if delete_mark != '*' {
                self.deserializer.reset_index();
                return Ok(Some(()));
            }
        }
    }

    pub fn read_record<T: DeserializeOwned>(
        &mut self,
    ) -> Result<Option<T>, Box<dyn std::error::Error>> {
        if self.next_record()?.is_none() {
            return Ok(None);
        }

        T::deserialize(&mut self.deserializer)
            .map(Some)
            .map_err(From::from)
    }

    pub fn records<T>(self) -> RecordIterator<R, T>
    where
        T: DeserializeOwned,
    {
        RecordIterator::new(self)
    }
}

pub struct RecordIterator<R, T>
where
    R: Read + Seek,
    T: DeserializeOwned,
{
    parser: DbfReader<R>,
    _type: PhantomData<T>,
}

impl<R: Read + Seek, T: DeserializeOwned> RecordIterator<R, T> {
    pub fn new(parser: DbfReader<R>) -> Self {
        Self {
            parser,
            _type: PhantomData,
        }
    }
}

impl<R: Read + Seek, T: DeserializeOwned> Iterator for RecordIterator<R, T> {
    type Item = Result<T, Box<dyn std::error::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parser.read_record().transpose()
    }
}
