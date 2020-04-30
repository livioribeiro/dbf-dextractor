use std::io::{Read, Seek};
use serde::de::{Deserializer, Visitor};

use crate::dbf::FieldType;
use crate::value::Value;
use crate::error::DeserializeError;
use super::DbfDeserializer;
use super::map_access::RecordReader;

impl<'a, 'de: 'a, R: Read + Seek> Deserializer<'de> for &'a mut DbfDeserializer<R> {
    type Error = DeserializeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_field()? {
            Some(Value::Character(value)) => visitor.visit_string(value),
            Some(Value::Date(value)) => visitor.visit_string(value),
            Some(Value::Float(value)) => visitor.visit_f64(value),
            Some(Value::Logical(value)) => visitor.visit_bool(value),
            Some(Value::Numeric(value)) => visitor.visit_f64(value),
            Some(Value::Memo(value)) => visitor.visit_string(value),
            Some(Value::Null) => visitor.visit_none(),
            None => Err(DeserializeError::unexpected_end_of_record()),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_field()? {
            Some(Value::Logical(value)) => visitor.visit_bool(value),
            _ => Err(self.error_expected(FieldType::Logical)),
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
            Some(Value::Numeric(value)) => visitor.visit_i64(value.trunc() as i64),
            _ => Err(self.error_expected(FieldType::Numeric)),
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
            Some(Value::Numeric(value)) if value < 0.0 => Err(self.error_field_parse()),
            Some(Value::Numeric(value)) => visitor.visit_u64(value.trunc() as u64),
            _ => Err(self.error_expected(FieldType::Numeric))
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
            Some(Value::Float(value)) | Some(Value::Numeric(value)) => visitor.visit_f64(value),
            _ => Err(self.error_expected(FieldType::Float)),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_field()? {
            Some(Value::Character(value)) if value.len() != 1 => Err(self.error_field_parse()),
            Some(Value::Character(value)) =>  visitor.visit_char(value.chars().next().unwrap()), // dbase uses only ascii
            _ => Err(self.error_expected(FieldType::Logical))
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
            Some(Value::Character(value)) | Some(Value::Date(value)) => visitor.visit_string(value),
            _ => Err(self.error_expected(FieldType::Character)),
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
            self.current_index += 1;
            return visitor.visit_none();
        }

        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.is_next_field_null() {
            self.current_index += 1;
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
        let record_length = self.fields.len();
        if len > record_length {
            return Err(self.error_tuple_length(len))
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
        let fields: Vec<_> = self.fields.iter().map(|f| f.name.to_owned()).collect();
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
