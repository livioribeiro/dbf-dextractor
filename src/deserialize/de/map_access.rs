use serde::de::IntoDeserializer;
use serde::de::{DeserializeSeed, MapAccess};

use crate::deserialize::DbfDeserializer;
use crate::error::DeserializeError;

impl<'de> MapAccess<'de> for DbfDeserializer {
    type Error = DeserializeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        match self.peek_value() {
            Some((field, _)) => seed
                .deserialize(field.name.clone().into_deserializer())
                .map(Some),
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(self)
    }
}
