use std::fmt;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{Visitor, Error, Unexpected};

#[derive(Debug, Clone)]
pub enum Value {
    Character(String),
    Date(String),
    Float(f64),
    Numeric(f64),
    Logical(bool),
    Memo(String),
    Null,
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Value::Character(ref value) => serializer.serialize_str(value),
            Value::Date(ref value) => serializer.serialize_str(value),
            Value::Float(value) => serializer.serialize_f64(value),
            Value::Logical(value) => serializer.serialize_bool(value),
            Value::Numeric(value) => serializer.serialize_f64(value),
            Value::Memo(ref value) => serializer.serialize_str(value),
            Value::Null => serializer.serialize_none(),
        }
    }
}

pub struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        formatter.write_str("Value")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Logical(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Numeric(v as f64))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Numeric(v as f64))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Float(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Character(v.to_owned()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Character(v))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Null)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        match deserializer.deserialize_option(ValueVisitor{})? {
            Value::Null => Err(Error::invalid_type(Unexpected::Option, &self)),
            value => Ok(value)
        }
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Null)
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_any(ValueVisitor{})
    }
}
