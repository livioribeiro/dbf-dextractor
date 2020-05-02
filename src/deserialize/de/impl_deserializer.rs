use serde::de::{Deserializer, Visitor};

use super::datetime_access::{DateAccess, TimestampAccess};
use crate::dbf::{FieldType, FieldValue};
use crate::deserialize::DbfDeserializer;
use crate::error::DeserializeError;

impl<'a, 'de: 'a> Deserializer<'de> for &'a mut DbfDeserializer {
    type Error = DeserializeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.is_next_value_null() == Some(true) {
            self.next_value();
            return visitor.visit_none();
        }

        match self.peek_field() {
            Some(FieldType::Logical) => self.deserialize_bool(visitor),
            Some(FieldType::Character) => self.deserialize_string(visitor),
            Some(FieldType::Integer) => self.deserialize_i32(visitor),
            Some(FieldType::Numeric) => self.deserialize_f64(visitor),
            Some(FieldType::Float) => self.deserialize_f64(visitor),
            Some(FieldType::Date) => self.deserialize_seq(visitor),
            Some(FieldType::Timestamp) => self.deserialize_seq(visitor),
            Some(FieldType::Memo) => self.deserialize_string(visitor),
            Some(FieldType::Binary) => self.deserialize_byte_buf(visitor),
            Some(FieldType::General) => self.deserialize_byte_buf(visitor),
            None => Err(DeserializeError::unexpected_end_of_record()),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value() {
            Some(FieldValue::Logical(value)) => visitor.visit_bool(value),
            Some(FieldValue::Null) => Err(self.error_unexpected_null()),
            Some(_) => Err(self.error_expected(FieldType::Logical)),
            None => Err(self.error_end_of_record()),
        }
    }

    fn deserialize_i8<V>(self, _isitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value() {
            Some(FieldValue::Integer(value)) => visitor.visit_i32(value),
            Some(FieldValue::Null) => Err(self.error_unexpected_null()),
            Some(_) => Err(self.error_expected(FieldType::Integer)),
            None => Err(self.error_end_of_record()),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value() {
            Some(FieldValue::Numeric(value)) => visitor.visit_i64(value.trunc() as i64),
            Some(FieldValue::Null) => Err(self.error_unexpected_null()),
            Some(_) => Err(self.error_expected(FieldType::Numeric)),
            None => Err(self.error_end_of_record()),
        }
    }

    fn deserialize_u8<V>(self, _isitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value() {
            Some(FieldValue::Numeric(value)) if value < 0.0 => Err(self.error_field_parse()),
            Some(FieldValue::Numeric(value)) => visitor.visit_u64(value.trunc() as u64),
            Some(FieldValue::Null) => Err(self.error_unexpected_null()),
            Some(_) => Err(self.error_expected(FieldType::Numeric)),
            None => Err(self.error_end_of_record()),
        }
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value() {
            Some(FieldValue::Float(value)) | Some(FieldValue::Numeric(value)) => {
                visitor.visit_f64(value)
            }
            Some(FieldValue::Null) => Err(self.error_unexpected_null()),
            Some(_) => Err(self.error_expected(FieldType::Float)),
            None => Err(self.error_end_of_record()),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value() {
            Some(FieldValue::Character(value)) if value.len() != 1 => Err(self.error_field_parse()),
            Some(FieldValue::Character(value)) => visitor.visit_char(value.chars().next().unwrap()), // dbase uses only ascii
            Some(FieldValue::Null) => Err(self.error_unexpected_null()),
            Some(_) => Err(self.error_expected(FieldType::Character)),
            None => Err(self.error_end_of_record()),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value() {
            Some(FieldValue::Character(value)) | Some(FieldValue::Memo(value)) => {
                visitor.visit_string(value)
            }
            Some(FieldValue::Null) => Err(self.error_unexpected_null()),
            Some(_) => Err(self.error_expected(FieldType::Character)),
            None => Err(self.error_end_of_record()),
        }
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value() {
            Some(FieldValue::Binary(value)) | Some(FieldValue::General(value)) => {
                visitor.visit_byte_buf(value)
            }
            Some(FieldValue::Null) => Err(self.error_unexpected_null()),
            Some(_) => Err(self.error_expected(FieldType::Character)),
            None => Err(self.error_end_of_record()),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.is_next_value_null() == Some(true) {
            self.next_value();
            return visitor.visit_none();
        }

        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.is_next_value_null() == Some(true) {
            self.next_value();
            return visitor.visit_unit();
        }

        Err(self.error_expected_null())
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value() {
            Some(FieldValue::Date(year, month, day)) => {
                visitor.visit_seq(DateAccess::new(year, month, day))
            }
            Some(FieldValue::Timestamp(year, month, day, hour, minute, second)) => {
                visitor.visit_seq(TimestampAccess::new(year, month, day, hour, minute, second))
            }
            Some(_) => {
                Err(self.error_expected(FieldType::Date))
            }
            None => Err(self.error_unexpected_null()),
        }
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let record_length = self.field_count();
        if len > record_length {
            return Err(self.error_tuple_length(len));
        }
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(self)
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.next_value();
        visitor.visit_unit()
    }
}
