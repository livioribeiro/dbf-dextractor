use std::io::{Read, Seek};
use crate::error::{FieldParseError, NoSuchFieldError};
use crate::dbf::{FieldInfo, Parser};
use crate::value::Value;

pub mod deserializer;
pub mod enum_access;
pub mod map_access;

pub struct DbfDeserializer<R: Read + Seek> {
    fields: Vec<FieldInfo>,
    parser: Parser<R>,
    buffer: Vec<u8>,
    current_index: usize,
}

impl<R: Read + Seek> DbfDeserializer<R> {
    pub fn new(fields: Vec<FieldInfo>, record_length: usize, parser: Parser<R>) -> Self {
        Self {
            fields,
            parser,
            buffer: vec![0u8; record_length],
            current_index: 0,
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
    }

    pub fn current_field_name(&self) -> &str {
        self.fields[self.current_index].name.as_ref()
    }

    pub fn next_field(&mut self) -> Result<Option<Value>, FieldParseError> {
        if !self.has_next_field() {
            return Ok(None);
        }

        let index = self.current_index;
        self.current_index += 1;

        self.parser.parse_field(&self.fields[index], &self.buffer).map(Some)
    }

    pub fn is_next_field_null(&self) -> bool {
        self.parser.check_null(&self.fields[self.current_index], &self.buffer)
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
}
