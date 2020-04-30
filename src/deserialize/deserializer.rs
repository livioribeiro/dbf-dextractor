use std::io::{Read, Seek};

use crate::dbf::{FieldInfo, FieldType, FieldValue, MemoReader};
use crate::error::{DeserializeError, NoSuchFieldError};
use super::parser::parse_field;

pub struct DbfDeserializer<R: Read + Seek> {
    fields: Vec<FieldInfo>,
    buffer: Vec<u8>,
    current_index: usize,
    record_count: usize,
    memo_reader: Option<MemoReader<R>>,
}

impl<R: Read + Seek> DbfDeserializer<R> {
    pub fn new(
        fields: Vec<FieldInfo>,
        record_length: usize,
        memo_reader: Option<MemoReader<R>>,
    ) -> Self {
        Self {
            fields,
            buffer: vec![0u8; record_length],
            current_index: 0,
            record_count: 0,
            memo_reader,
        }
    }

    pub fn fields(&self) -> &Vec<FieldInfo> {
        &self.fields
    }

    pub fn buffer(&mut self) -> &mut [u8] {
        &mut self.buffer
    }

    pub fn memo_reader(&mut self) -> Option<&mut MemoReader<R>> {
        self.memo_reader.as_mut()
    }

    pub fn field_count(&self) -> usize {
        self.fields.len()
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

    pub fn incr_index(&mut self) {
        self.current_index += 1;
    }

    pub fn current_field_name(&self) -> &str {
        self.fields[self.current_index].name.as_ref()
    }

    pub fn peek_field(&self) -> Option<&FieldType> {
        self.fields.get(self.current_index).map(|f| &f.field_type)
    }

    pub fn next_field(&mut self) -> Result<Option<FieldValue>, DeserializeError> {
        if !self.has_next_field() {
            return Ok(None);
        }

        let index = self.current_index;
        self.current_index += 1;

        parse_field(&self.fields[index], &self.buffer)
            .map(Some)
            .map_err(|_| self.error_field_parse())
    }

    pub fn is_next_field_null(&self) -> bool {
        super::parser::check_null(&self.fields[self.current_index], &self.buffer)
    }

    pub fn field_index(&self, name: &str) -> Option<usize> {
        self.fields.iter().position(|f| f.name == name)
    }

    pub fn set_field_with_name(&mut self, name: &str) -> Result<(), NoSuchFieldError> {
        let index = self
            .field_index(name)
            .ok_or_else(|| NoSuchFieldError::new(name))?;
        self.current_index = index;
        Ok(())
    }

    pub fn has_next_field(&self) -> bool {
        self.current_index < self.fields.len()
    }

    pub fn error_expected(&self, field_type: FieldType) -> DeserializeError {
        DeserializeError::expected(
            field_type,
            self.record_count,
            &self.fields[self.current_index - 1].name,
        )
    }

    pub fn error_unexpected_null(&self) -> DeserializeError {
        DeserializeError::unexpected_null(
            self.record_count,
            &self.fields[self.current_index - 1].name,
        )
    }

    pub fn error_expected_null(&self) -> DeserializeError {
        DeserializeError::expected_null(
            self.record_count,
            &self.fields[self.current_index - 1].name,
        )
    }

    pub fn error_field_parse(&self) -> DeserializeError {
        DeserializeError::field_parse(self.record_count, &self.fields[self.current_index - 1].name)
    }

    pub fn error_tuple_length(&self, length: usize) -> DeserializeError {
        DeserializeError::tuple_length(length, self.fields.len())
    }

    pub fn error_end_of_record(&self) -> DeserializeError {
        DeserializeError::unexpected_end_of_record()
    }

    pub fn error_missing_memo_file(&self) -> DeserializeError {
        DeserializeError::missing_memo_file()
    }
}
