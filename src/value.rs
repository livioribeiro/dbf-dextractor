use serde::{Deserialize, Serialize, Serializer};

use crate::dbf::FieldValue;
use crate::model::{Date, Timestamp};

#[derive(Deserialize, Debug, Clone)]
pub enum Value {
    Str(String),
    Int(i32),
    Float(f64),
    Bool(bool),
    Date(Date),
    Timestamp(Timestamp),
    Bytes(Vec<u8>),
    Null,
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Value::Bool(val) => serializer.serialize_bool(val),
            Value::Str(ref val) => serializer.serialize_str(val),
            Value::Int(val) => serializer.serialize_i64(val as i64),
            Value::Float(val) => serializer.serialize_f64(val),
            Value::Date(ref val) => serializer.serialize_str(&val.to_string()),
            Value::Timestamp(ref val) => serializer.serialize_str(&val.to_string()),
            Value::Bytes(ref val) => serializer.collect_seq(val),
            Value::Null => serializer.serialize_none(),
        }
    }
}

impl From<FieldValue> for Value {
    fn from(value: FieldValue) -> Self {
        match value {
            FieldValue::Logical(val) => Value::Bool(val),
            FieldValue::Character(val) | FieldValue::Memo(val) => Value::Str(val),
            FieldValue::Integer(val) => Value::Int(val),
            FieldValue::Numeric(val) => Value::Float(val),
            FieldValue::Float(val) => Value::Float(val),
            FieldValue::Date(year, month, day) => Value::Date(Date::from((year, month, day))),
            FieldValue::Timestamp(year, month, day, hour, minute, second) => {
                Value::Timestamp(Timestamp::from((year, month, day, hour, minute, second)))
            }
            FieldValue::Binary(val) | FieldValue::General(val) => Value::Bytes(val),
            FieldValue::Null => Value::Null,
        }
    }
}
