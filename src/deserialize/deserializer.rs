use std::collections::VecDeque;

use crate::dbf::{FieldInfo, FieldType, FieldValue};
use crate::error::DeserializeError;

pub struct DbfDeserializer {
    fields: Vec<FieldInfo>,
    values: VecDeque<(FieldInfo, FieldValue)>,
    record_count: usize,
}

impl DbfDeserializer {
    pub fn new(fields: Vec<FieldInfo>) -> Self {
        Self {
            fields,
            values: VecDeque::new(),
            record_count: 0,
        }
    }

    pub fn fields(&self) -> &Vec<FieldInfo> {
        &self.fields
    }

    pub fn field_count(&self) -> usize {
        self.fields.len()
    }

    pub fn peek_value(&self) -> Option<&(FieldInfo, FieldValue)> {
        self.values.front()
    }

    pub fn set_values(&mut self, values: Vec<FieldValue>) {
        self.values = self
            .fields
            .clone()
            .into_iter()
            .zip(values.into_iter())
            .collect();
        self.record_count += 1;
    }

    pub fn peek_field(&self) -> Option<&FieldType> {
        self.fields
            .get(self.fields.len() - self.values.len())
            .map(|f| &f.field_type)
    }

    pub fn next_value(&mut self) -> Option<FieldValue> {
        self.values.pop_front().map(|(_, value)| value)
    }

    pub fn is_next_value_null(&self) -> Option<bool> {
        self.values.get(0).map(|(_, v)| match v {
            FieldValue::Null => true,
            _ => false,
        })
    }

    pub fn has_next_field(&self) -> bool {
        !self.values.is_empty()
    }

    fn current_field_name(&self) -> &str {
        &self.fields[self.fields.len() - self.values.len() - 1].name
    }

    pub fn error_expected(&self, field_type: FieldType) -> DeserializeError {
        DeserializeError::expected(field_type, self.record_count, self.current_field_name())
    }

    pub fn error_unexpected_null(&self) -> DeserializeError {
        DeserializeError::unexpected_null(self.record_count, self.current_field_name())
    }

    pub fn error_expected_null(&self) -> DeserializeError {
        DeserializeError::expected_null(self.record_count, self.current_field_name())
    }

    pub fn error_field_parse(&self) -> DeserializeError {
        DeserializeError::field_parse(self.record_count, self.current_field_name())
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
