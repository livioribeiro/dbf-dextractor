use std::convert::TryFrom;
use std::fmt;

use crate::error::UnsupportedFieldTypeError;

pub fn read_field_info(buf: &[u8]) -> Result<Vec<FieldInfo>, UnsupportedFieldTypeError> {
    buf.chunks(32)
        .scan(1usize, |acc_offset, info| {
            if info.len() < 32 {
                return None;
            }

            if info[0] == b'\r' {
                return None;
            }

            let name_bytes = &info[0..=10];
            let field_type = info[11];
            let length = info[16] as usize;
            let offset = *acc_offset;
            *acc_offset += length;

            match FieldInfo::new(name_bytes, field_type, length, offset) {
                Ok(value) => Some(Ok(value)),
                Err(e) => Some(Err(e)),
            }
        })
        .collect()
}

#[derive(Clone, Debug)]
pub enum FieldType {
    Binary,
    Character,
    Date,
    Float,
    General,
    Integer,
    Numeric,
    Logical,
    Memo,
    Timestamp,
}

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            FieldType::Binary => "Binary",
            FieldType::Character => "Character",
            FieldType::Date => "Date",
            FieldType::Float => "Float",
            FieldType::General => "General",
            FieldType::Integer => "Integer",
            FieldType::Numeric => "Numeric",
            FieldType::Logical => "Logical",
            FieldType::Memo => "Memo",
            FieldType::Timestamp => "Timestamp",
        };

        f.write_str(name)
    }
}

impl TryFrom<u8> for FieldType {
    type Error = UnsupportedFieldTypeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let value = value as char;
        match value {
            'B' => Ok(FieldType::Binary),
            'C' => Ok(FieldType::Character),
            'D' => Ok(FieldType::Date),
            'F' => Ok(FieldType::Float),
            'G' => Ok(FieldType::General),
            'I' => Ok(FieldType::Integer),
            'N' => Ok(FieldType::Numeric),
            'L' => Ok(FieldType::Logical),
            'M' => Ok(FieldType::Memo),
            'T' => Ok(FieldType::Timestamp),
            _ => Err(UnsupportedFieldTypeError(value)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FieldInfo {
    pub name: String,
    pub field_type: FieldType,
    pub length: usize,
    pub offset: usize,
}

impl FieldInfo {
    pub fn new(
        name_bytes: &[u8],
        field_type: u8,
        length: usize,
        offset: usize,
    ) -> Result<Self, UnsupportedFieldTypeError> {
        let name = name_bytes
            .iter()
            .take_while(|b| **b != 0u8)
            .map(|b| *b as char)
            .fold(String::new(), |mut acc, val| {
                acc.push(val);
                acc
            });

        let field_type = FieldType::try_from(field_type)?;

        Ok(Self {
            name,
            field_type,
            length,
            offset,
        })
    }
}

#[derive(Debug)]
pub enum FieldValue {
    Binary(Vec<u8>),
    Character(String),
    Date(u16, u8, u8),
    Float(f64),
    General(Vec<u8>),
    Integer(i32),
    Numeric(f64),
    Logical(bool),
    Memo(String),
    Timestamp(u16, u8, u8, u8, u8, u8),
    Null,
}
