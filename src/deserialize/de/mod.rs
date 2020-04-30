use std::io::{Read, Seek};

use serde::de::{Deserializer, Visitor};

use crate::dbf::{FieldType, FieldValue};
use crate::error::DeserializeError;

mod map_access;
mod enum_access;

use map_access::RecordReader;
use super::DbfDeserializer;

impl<'a, 'de: 'a, R: Read + Seek> Deserializer<'de> for &'a mut DbfDeserializer<R> {
    type Error = DeserializeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.is_next_field_null() {
            self.incr_index();
            return visitor.visit_none();
        }

        match self.peek_field() {
            Some(FieldType::Character) => self.deserialize_string(visitor),
            Some(FieldType::Date) => self.deserialize_string(visitor),
            Some(FieldType::Float) => self.deserialize_f64(visitor),
            Some(FieldType::Logical) => self.deserialize_bool(visitor),
            Some(FieldType::Numeric) => self.deserialize_f64(visitor),
            Some(FieldType::Memo) => self.deserialize_string(visitor),
            None => Err(DeserializeError::unexpected_end_of_record()),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_field()? {
            Some(FieldValue::Logical(value)) => visitor.visit_bool(value),
            Some(FieldValue::Null) => Err(self.error_unexpected_null()),
            Some(_) => Err(self.error_expected(FieldType::Logical)),
            None => Err(self.error_end_of_record()),
        }
    }

    fn deserialize_i8<V>(self, _isitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i32<V>(self, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_field()? {
            Some(FieldValue::Numeric(value)) => visitor.visit_i64(value.trunc() as i64),
            Some(FieldValue::Null) => Err(self.error_unexpected_null()),
            Some(_) => Err(self.error_expected(FieldType::Numeric)),
            None => Err(self.error_end_of_record()),
        }
    }

    fn deserialize_u8<V>(self, _isitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_field()? {
            Some(FieldValue::Numeric(value)) if value < 0.0 => Err(self.error_field_parse()),
            Some(FieldValue::Numeric(value)) => visitor.visit_u64(value.trunc() as u64),
            Some(FieldValue::Null) => Err(self.error_unexpected_null()),
            Some(_) => Err(self.error_expected(FieldType::Numeric)),
            None => Err(self.error_end_of_record()),
        }
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_field()? {
            Some(FieldValue::Float(value)) | Some(FieldValue::Numeric(value)) => {
                visitor.visit_f64(value)
            }
            Some(FieldValue::Null) => Err(self.error_unexpected_null()),
            Some(_) => Err(self.error_expected(FieldType::Float)),
            None => Err(self.error_end_of_record()),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_field()? {
            Some(FieldValue::Character(value)) if value.len() != 1 => Err(self.error_field_parse()),
            Some(FieldValue::Character(value)) => visitor.visit_char(value.chars().next().unwrap()), // dbase uses only ascii
            Some(FieldValue::Null) => Err(self.error_unexpected_null()),
            Some(_) => Err(self.error_expected(FieldType::Character)),
            None => Err(self.error_end_of_record()),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_field()? {
            Some(FieldValue::Character(value)) => visitor.visit_string(value),
            Some(FieldValue::Date(value)) => visitor.visit_string(value),
            Some(FieldValue::Memo(value)) => {
                if let Some(r) = self.memo_reader() {
                    let value = r.read_memo(value)?;
                    visitor.visit_string(value)
                } else {
                    Err(self.error_missing_memo_file())
                }
            }
            Some(FieldValue::Null) => Err(self.error_unexpected_null()),
            Some(_) => Err(self.error_expected(FieldType::Character)),
            None => Err(self.error_end_of_record()),
        }
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.is_next_field_null() {
            self.incr_index();
            return visitor.visit_none();
        }

        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.is_next_field_null() {
            self.incr_index();
            return visitor.visit_unit();
        }

        Err(self.error_expected_null())
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple<V>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
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
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let fields: Vec<_> = self.fields().iter().map(|f| f.name.to_owned()).collect();
        let fields: Vec<_> = fields.iter().map(|f| f.as_ref()).collect();
        visitor.visit_map(&mut RecordReader::new(self, &fields))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(RecordReader::new(self, fields))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(self)
    }

    fn deserialize_identifier<V>(
        self,
        _visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.next_field()?;
        visitor.visit_unit()
    }
}
