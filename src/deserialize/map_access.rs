use std::io::{Read, Seek};
use serde::de::IntoDeserializer;
use serde::de::{DeserializeSeed, MapAccess};

use super::DbfDeserializer;
use crate::error::DeserializeError;

pub struct RecordReader<'a, R: Read + Seek> {
    deserializer: &'a mut DbfDeserializer<R>,
    fields: &'a [&'a str],
    index: usize,
}

impl<'a, R: Read + Seek> RecordReader<'a, R> {
    pub fn new(deserializer: &'a mut DbfDeserializer<R>, fields: &'a [&'a str]) -> Self {
        Self {
            deserializer,
            fields,
            index: 0,
        }
    }
}

impl<'a, 'de: 'a, R: Read + Seek> MapAccess<'de> for RecordReader<'a, R> {
    type Error = DeserializeError;

    fn next_key_seed<K>(
        &mut self,
        seed: K,
    ) -> Result<Option<<K as DeserializeSeed<'de>>::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.index >= self.fields.len() {
            return Ok(None);
        }

        seed.deserialize(self.fields[self.index].into_deserializer()).map(Some)
    }

    fn next_value_seed<V>(
        &mut self,
        seed: V,
    ) -> Result<<V as DeserializeSeed<'de>>::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        self.deserializer.set_field_with_name(self.fields[self.index])?;
        self.index += 1;
        seed.deserialize(&mut *self.deserializer)
    }
}
