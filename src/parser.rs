use std::convert::TryInto;
use std::num::ParseFloatError;

use crate::dbf::{FieldType, FieldInfo};
use crate::dbf::FieldValue;
use crate::error::FieldParseError;

fn parse_character(buf: &[u8]) -> FieldValue {
    FieldValue::Character(String::from_utf8_lossy(buf).trim().to_owned())
}

fn parse_date(buf: &[u8]) -> FieldValue {
    FieldValue::Date(String::from_utf8_lossy(buf).trim().to_owned())
}

fn parse_float(buf: &[u8]) -> Result<FieldValue, ParseFloatError> {
    let value = String::from_utf8_lossy(buf);
    value.trim().parse().map(FieldValue::Float)
}

fn parse_numeric(buf: &[u8]) -> Result<FieldValue, ParseFloatError> {
    let value = String::from_utf8_lossy(buf);
    value.trim().parse().map(FieldValue::Numeric)
}

fn parse_logic(buf: &[u8]) -> FieldValue {
    match buf[0] as char {
        't' | 'T' | 'y' | 'Y' => FieldValue::Logical(true),
        'f' | 'F' | 'n' | 'N' => FieldValue::Logical(false),
        _ => FieldValue::Null,
    }
}

fn parse_memo(buf: &[u8]) -> Result<FieldValue, FieldParseError> {
    let offset = if buf.len() == 4 {
        u32::from_le_bytes(buf.try_into().expect("parse memo field"))
    } else {
        String::from_utf8_lossy(buf).trim().parse().expect("parse memo field")
    };

    Ok(FieldValue::Memo(offset))
}

pub fn check_null(field: &FieldInfo, record_buf: &[u8]) -> bool {
    let start = field.offset;
    let end = field.offset + field.length;
    let buf = &record_buf[start..end];

    buf.iter().all(|b| *b == b' ')
}

pub fn parse_field(field: &FieldInfo, record_buf: &[u8]) -> Result<FieldValue, FieldParseError> {
    let start = field.offset;
    let end = field.offset + field.length;
    let buf = &record_buf[start..end];

    if buf.iter().all(|b| *b == b' ') {
        return Ok(FieldValue::Null)
    }

    let map_e = |_| FieldParseError::new(field.name.clone(), field.field_type.clone());

    match field.field_type {
        FieldType::Character => Ok(parse_character(buf)),
        FieldType::Date => Ok(parse_date(buf)),
        FieldType::Float => parse_float(buf).map_err(map_e),
        FieldType::Numeric => parse_numeric(buf).map_err(map_e),
        FieldType::Logical => Ok(parse_logic(buf)),
        FieldType::Memo => parse_memo(buf),
    }
}
