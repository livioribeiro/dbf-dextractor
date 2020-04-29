use std::io::{Read, Seek};
use serde::de::IntoDeserializer;
use serde::de::{DeserializeSeed, EnumAccess, VariantAccess, Visitor};

use super::DbfDeserializer;
use crate::error::DeserializeError;
use crate::value::Value;

impl<'a, 'de: 'a, R: Read + Seek> EnumAccess<'de> for &'a mut DbfDeserializer<R> {
    type Error = DeserializeError;
    type Variant = Self;

    fn variant_seed<V: DeserializeSeed<'de>>(
        self,
        seed: V,
    ) -> Result<(V::Value, Self::Variant), Self::Error> {
        let value = match self.next_field()? {
            Some(Value::Character(value)) => value,
            e => return Err(format!("invalid enum value: {:?}", e).into()),
        };
        seed.deserialize(value.into_deserializer())
            .map(|v| (v, self))
    }
}

impl<'a, 'de: 'a, R: Read + Seek> VariantAccess<'de> for &'a mut DbfDeserializer<R> {
    type Error = DeserializeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<U: DeserializeSeed<'de>>(
        self,
        seed: U,
    ) -> Result<U::Value, Self::Error> {
        seed.deserialize(self)
    }

    fn tuple_variant<V: Visitor<'de>>(
        self,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error> {
        unimplemented!()
    }

    fn struct_variant<V: Visitor<'de>>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error> {
        unimplemented!()
    }
}
