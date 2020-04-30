use std::convert::TryFrom;
use std::fmt;

use crate::error::UnsupportedFieldTypeError;

#[derive(Clone, Debug)]
pub enum FieldType {
    Character,
    Date,
    Float,
    Numeric,
    Logical,
    Memo,
}

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            FieldType::Character => "Character",
            FieldType::Date => "Date",
            FieldType::Float => "Float",
            FieldType::Numeric => "Numeric",
            FieldType::Logical => "Logical",
            FieldType::Memo => "Memo",
        };

        f.write_str(name)
    }
}

impl TryFrom<u8> for FieldType {
    type Error = UnsupportedFieldTypeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let value = value as char;
        match value {
            'C' => Ok(FieldType::Character),
            'D' => Ok(FieldType::Date),
            'F' => Ok(FieldType::Float),
            'N' => Ok(FieldType::Numeric),
            'L' => Ok(FieldType::Logical),
            'M' => Ok(FieldType::Memo),
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
    Character(String),
    Date(String),
    Float(f64),
    Numeric(f64),
    Logical(bool),
    Memo(u32),
    Null,
}
