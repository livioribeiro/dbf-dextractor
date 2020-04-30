use std::io::{Read, Seek};
use crate::error::{DeserializeError, NoSuchFieldError};
use crate::dbf::{FieldType, FieldInfo, FieldValue, MemoReader};
use crate::value::Value;
use crate::parser::parse_field;

pub mod deserializer;
pub mod enum_access;
pub mod map_access;

pub struct DbfDeserializer<R: Read + Seek> {
    fields: Vec<FieldInfo>,
    buffer: Vec<u8>,
    current_index: usize,
    record_count: usize,
    memo_reader: Option<MemoReader<R>>,
}

impl<R: Read + Seek> DbfDeserializer<R> {
    pub fn new(fields: Vec<FieldInfo>, record_length: usize, memo_reader: Option<MemoReader<R>>) -> Self {
        Self {
            fields,
            buffer: vec![0u8; record_length],
            current_index: 0,
            record_count: 0,
            memo_reader,
        }
    }

    pub fn buffer(&mut self) -> &mut [u8] {
        &mut self.buffer
    }

    pub fn reset_buffer(&mut self) {
        for i in 0..self.buffer.len() {
            self.buffer[i] = 0;
        }
    }

    pub fn reset_index(&mut self) {
        self.current_index = 0;
        self.record_count += 1;
    }

    pub fn current_field_name(&self) -> &str {
        self.fields[self.current_index].name.as_ref()
    }

    pub fn next_field(&mut self) -> Result<Option<Value>, DeserializeError> {
        if !self.has_next_field() {
            return Ok(None);
        }

        let index = self.current_index;
        self.current_index += 1;

        let value = match parse_field(&self.fields[index], &self.buffer)
            .map_err(|_| self.error_field_parse())?
        {
            FieldValue::Character(v) => Value::Character(v),
            FieldValue::Date(v) => Value::Date(v),
            FieldValue::Float(v) => Value::Float(v),
            FieldValue::Numeric(v) => Value::Numeric(v),
            FieldValue::Logical(v) => Value::Logical(v),
            FieldValue::Null => Value::Null,
            FieldValue::Memo(v) => {
                if let Some(r) = self.memo_reader.as_mut() {
                    Value::Memo(r.read_memo(v)?)
                } else {
                    return Err(DeserializeError::missing_memo_file(self.record_count, self.current_field_name()))
                }
            }
        };

        Ok(Some(value))
    }

    pub fn is_next_field_null(&self) -> bool {
        super::parser::check_null(&self.fields[self.current_index], &self.buffer)
    }

    fn field_index(&self, name: &str) -> Option<usize> {
        self.fields.iter().position(|f| f.name == name)
    }

    pub fn set_field_with_name(&mut self, name: &str) -> Result<(), NoSuchFieldError> {
        let index = self.field_index(name).ok_or_else(|| NoSuchFieldError::new(name))?;
        self.current_index = index;
        Ok(())
    }

    pub fn has_next_field(&self) -> bool {
        self.current_index < self.fields.len()
    }

    fn error_expected(&self, field_type: FieldType) -> DeserializeError {
        DeserializeError::expected(field_type, self.record_count, &self.fields[self.current_index - 1].name)
    }

    fn error_expected_null(&self) -> DeserializeError {
        DeserializeError::expected_null(self.record_count, &self.fields[self.current_index - 1].name)
    }

    fn error_field_parse(&self) -> DeserializeError {
        DeserializeError::field_parse(self.record_count, &self.fields[self.current_index - 1].name)
    }

    fn error_tuple_length(&self, length: usize) -> DeserializeError {
        DeserializeError::tuple_length(length, self.fields.len())
    }
}
