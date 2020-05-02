use std::convert::TryInto;
use std::error::Error as StdError;
use std::io::{Read, Seek};

use crate::dbf::{FieldInfo, FieldType, FieldValue, MemoReader};
use crate::error::FieldParseError;

pub fn parse_record<R>(
    fields: &[FieldInfo],
    buf: &[u8],
    memo_reader: &mut Option<MemoReader<R>>,
) -> Result<Vec<FieldValue>, FieldParseError>
where
    R: Read + Seek,
{
    fields
        .iter()
        .map(|f| parse_field(f, buf, memo_reader))
        .collect()
}

fn parse_field<R>(
    field: &FieldInfo,
    record_buf: &[u8],
    memo_reader: &mut Option<MemoReader<R>>,
) -> Result<FieldValue, FieldParseError>
where
    R: Read + Seek,
{
    let start = field.offset;
    let end = field.offset + field.length;
    let buf = &record_buf[start..end];

    if buf.iter().all(|b| *b == b' ') || buf.iter().all(|b| *b == b'\0') {
        return Ok(FieldValue::Null);
    }

    let map_e = |e| FieldParseError::new(field.name.clone(), field.field_type.clone(), Some(e));

    match field.field_type {
        FieldType::Logical => Ok(parse_logic(buf)),
        FieldType::Character => Ok(parse_character(buf)),
        FieldType::Integer => parse_integer(buf).map_err(map_e),
        FieldType::Numeric => parse_numeric(buf).map_err(map_e),
        FieldType::Float => parse_float(buf).map_err(map_e),
        FieldType::Date => parse_date(buf).map_err(map_e),
        FieldType::Timestamp => parse_timestamp(buf).map_err(map_e),
        FieldType::Memo => parse_memo(buf, memo_reader).map_err(map_e),
        FieldType::Binary => parse_binary(buf, memo_reader).map_err(map_e),
        FieldType::General => parse_general(buf, memo_reader).map_err(map_e),
    }
}

fn memo_index(buf: &[u8]) -> Result<u32, Box<dyn StdError>> {
    if buf.len() == 4 {
        Ok(u32::from_le_bytes(buf.try_into()?))
    } else {
        String::from_utf8_lossy(buf)
            .trim()
            .parse()
            .map_err(From::from)
    }
}

fn parse_logic(buf: &[u8]) -> FieldValue {
    match buf[0] as char {
        't' | 'T' | 'y' | 'Y' => FieldValue::Logical(true),
        'f' | 'F' | 'n' | 'N' => FieldValue::Logical(false),
        _ => FieldValue::Null,
    }
}

fn parse_character(buf: &[u8]) -> FieldValue {
    FieldValue::Character(String::from_utf8_lossy(buf).trim().to_owned())
}

fn parse_integer(buf: &[u8]) -> Result<FieldValue, Box<dyn StdError>> {
    let value = i32::from_be_bytes(buf.try_into()?);
    Ok(FieldValue::Integer(value))
}

fn parse_numeric(buf: &[u8]) -> Result<FieldValue, Box<dyn StdError>> {
    let value = String::from_utf8_lossy(buf);
    value.trim().parse().map(FieldValue::Numeric).map_err(From::from)
}

fn parse_float(buf: &[u8]) -> Result<FieldValue, Box<dyn StdError>> {
    let value = String::from_utf8_lossy(buf);
    value.trim().parse().map(FieldValue::Float).map_err(From::from)
}

fn parse_date(buf: &[u8]) -> Result<FieldValue, Box<dyn StdError>> {
    let value = String::from_utf8_lossy(buf);
    let year = value[0..4].parse()?;
    let month = value[4..6].parse()?;
    let day = value[6..8].parse()?;
    Ok(FieldValue::Date(year, month, day))
}

// https://en.wikipedia.org/wiki/Julian_day#Julian_or_Gregorian_calendar_from_Julian_day_number
#[allow(non_snake_case, clippy::many_single_char_names)]
fn from_julian_day_to_gregorian_calender(julian_day: u32) -> (u16, u8, u8) {
    let y = 4716;
    let j = 1401;
    let m = 2;
    let n = 12;
    let r = 4;
    let p = 1461;
    let v = 3;
    let u = 5;
    let s = 153;
    let w = 2;
    let B = 274_277;
    let C = -38;

    let J = julian_day as i32;

    let f = J + j + (((4 * J + B) / 146_097) * 3) / 4 + C;
    let e = r * f + v;
    let g = e % p / r;
    let h = u * g + w;
    let day = (h % s) / u + 1;
    let month = (h / s + m) % n + 1;
    let year = (e / p) - y + (n + m - month) / n;

    (year as u16, month as u8, day as u8)
}

fn from_time_part_to_time(time_part: u32) -> (u8, u8, u8) {
    let mut time_part = time_part as i64;
    let hour = time_part / 3_600_000;
    time_part -= hour * 3_600_000;
    let minute = time_part / 60_000;
    time_part -= minute * 60_000;
    let second = time_part / 1000;

    (hour as u8, minute as u8, second as u8)
}

fn parse_timestamp(buf: &[u8]) -> Result<FieldValue, Box<dyn StdError>> {
    let date_part = u32::from_le_bytes((&buf[..4]).try_into()?);
    let time_part = u32::from_le_bytes((&buf[4..]).try_into()?);

    let (year, month, day) = from_julian_day_to_gregorian_calender(date_part);
    let (hour, minute, second) = from_time_part_to_time(time_part);

    Ok(FieldValue::Timestamp(
        year, month, day, hour, minute, second,
    ))
}

fn parse_memo<R>(
    buf: &[u8],
    memo_reader: &mut Option<MemoReader<R>>,
) -> Result<FieldValue, Box<dyn StdError>>
where
    R: Read + Seek,
{
    if let Some(reader) = memo_reader.as_mut() {
        let index = memo_index(buf)?;
        let value = reader.read_memo(index)?;
        Ok(FieldValue::Memo(
            String::from_utf8_lossy(&value).into_owned(),
        ))
    } else {
        Ok(FieldValue::Null)
    }
}

fn parse_binary<R>(
    buf: &[u8],
    memo_reader: &mut Option<MemoReader<R>>,
) -> Result<FieldValue, Box<dyn StdError>>
where
    R: Read + Seek,
{
    if let Some(reader) = memo_reader.as_mut() {
        let index = memo_index(buf)?;
        let value = reader.read_memo(index)?;
        Ok(FieldValue::Binary(value))
    } else {
        Ok(FieldValue::Null)
    }
}

fn parse_general<R>(
    buf: &[u8],
    memo_reader: &mut Option<MemoReader<R>>,
) -> Result<FieldValue, Box<dyn StdError>>
where
    R: Read + Seek,
{
    if let Some(reader) = memo_reader.as_mut() {
        let index = memo_index(buf)?;
        let value = reader.read_memo(index)?;
        Ok(FieldValue::General(value))
    } else {
        Ok(FieldValue::Null)
    }
}
