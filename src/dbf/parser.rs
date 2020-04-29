use std::convert::TryInto;
use std::io::{Read, Seek, Error as IoError};
use std::num::ParseFloatError;

use crate::dbf::{FieldType, FieldInfo, MemoReader};
use crate::value::Value;
use crate::error::FieldParseError;

pub struct Parser<R: Read + Seek> {
    memo_reader: Option<MemoReader<R>>,
}

impl<R: Read + Seek> Parser<R> {
    pub fn new(memo_reader: Option<MemoReader<R>>) -> Self {
        Self { memo_reader }
    }

    fn parse_character(&self, buf: &[u8]) -> Value {
        Value::Character(String::from_utf8_lossy(buf).trim().to_owned())
    }

    fn parse_date(&self, buf: &[u8]) -> Value {
        Value::Date(String::from_utf8_lossy(buf).trim().to_owned())
    }

    fn parse_float(&self, buf: &[u8]) -> Result<Value, ParseFloatError> {
        let value = String::from_utf8_lossy(buf);
        dbg!(&value);
        value.trim().parse().map(Value::Float)
    }

    fn parse_numeric(&self, buf: &[u8]) -> Result<Value, ParseFloatError> {
        let value = String::from_utf8_lossy(buf);
        value.trim().parse().map(Value::Numeric)
    }

    fn parse_logic(&self, buf: &[u8]) -> Value {
        match buf[0] as char {
            't' | 'T' | 'y' | 'Y' => Value::Logical(true),
            'f' | 'F' | 'n' | 'N' => Value::Logical(false),
            _ => Value::Null,
        }
    }

    fn parse_memo(&mut self, buf: &[u8]) -> Result<Value, IoError> {
        let offset = if buf.len() == 4 {
            u32::from_le_bytes(buf.try_into().expect("parse memo field"))
        } else {
            String::from_utf8_lossy(buf).parse().expect("parse memo field")
        };

        Ok(Value::Memo(offset.to_string()))

        // if let Some(memo_reader) = self.memo_reader.as_mut() {
        //     memo_reader.read_memo(offset)
        //         .map(Value::Memo)
        //         .map_err(|e| IoError::new(ErrorKind::Other, e))
        // } else {
        //     Err(IoError::from(ErrorKind::Other))
        // }
    }

    pub fn check_null(&self, field: &FieldInfo, record_buf: &[u8]) -> bool {
        let start = field.offset;
        let end = field.offset + field.length;
        let buf = &record_buf[start..end];
    
        buf.iter().all(|b| *b == b' ')
    }

    pub fn parse_field(&mut self, field: &FieldInfo, record_buf: &[u8]) -> Result<Value, FieldParseError> {
        let start = field.offset;
        let end = field.offset + field.length;
        let buf = &record_buf[start..end];

        if buf.iter().all(|b| *b == b' ') {
            return Ok(Value::Null)
        }

        let map_e = |_| FieldParseError::new(field.name.clone(), String::from_utf8_lossy(buf).into_owned(), field.field_type.clone());

        match field.field_type {
            FieldType::Character => Ok(self.parse_character(buf)),
            FieldType::Date => Ok(self.parse_date(buf)),
            FieldType::Float => self.parse_float(buf).map_err(map_e),
            FieldType::Numeric => self.parse_numeric(buf).map_err(map_e),
            FieldType::Logical => Ok(self.parse_logic(buf)),
            FieldType::Memo => self.parse_memo(buf).map_err(|_| FieldParseError::new(field.name.clone(), String::from_utf8_lossy(buf).into_owned(), field.field_type.clone())),
        }
    }
}
