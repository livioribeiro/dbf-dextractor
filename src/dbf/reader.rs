use std::error::Error;
use std::io::{Read, Seek, SeekFrom};

use super::field::{read_field_info, FieldInfo, FieldValue};
use super::header::Header;
use super::memo::MemoReader;
use super::parser;

pub struct DbfReader<R: Read + Seek> {
    reader: R,
    header: Header,
    fields: Vec<FieldInfo>,
    memo_reader: Option<MemoReader<R>>,
    buffer: Vec<u8>,
}

impl<'a, R: Read + Seek> DbfReader<R> {
    pub fn from_reader(mut reader: R, memo_reader: Option<R>) -> Result<Self, Box<dyn Error>> {
        let header = Header::from_reader(&mut reader)?;

        reader.seek(SeekFrom::Start(32))?;

        let fields = {
            let mut buf = vec![0u8; header.header_length - 32];
            reader.read_exact(&mut buf)?;
            read_field_info(&buf)?
        };

        let memo_reader = memo_reader
            .map(|r| MemoReader::from_reader(r, header.version))
            .transpose()?;

        let buffer = vec![0u8; header.record_length];

        Ok(Self {
            reader,
            header,
            fields,
            memo_reader,
            buffer,
        })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn record_length(&self) -> usize {
        self.header.record_length
    }

    pub fn fields(&self) -> &Vec<FieldInfo> {
        &self.fields
    }

    fn read_record(&mut self) -> Result<Option<()>, std::io::Error> {
        loop {
            self.buffer.resize(self.header.record_length, 0);

            let n_read = self.reader.read(&mut self.buffer)?;
            if n_read <= 1 {
                return Ok(None);
            }

            let delete_mark = self.buffer[0] as char;
            if delete_mark != '*' {
                return Ok(Some(()));
            }
        }
    }

    pub fn next_record(&mut self) -> Result<Option<Vec<FieldValue>>, Box<dyn std::error::Error>> {
        if self.read_record()?.is_none() {
            return Ok(None);
        }

        parser::parse_record(&self.fields, &self.buffer, &mut self.memo_reader)
            .map(Some)
            .map_err(From::from)
    }
}
