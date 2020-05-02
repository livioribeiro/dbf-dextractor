use serde::{Deserialize, Serialize, Serializer};

use crate::dbf::FieldValue;
use crate::model::{Date, Timestamp};

#[derive(Deserialize, Debug, Clone)]
pub enum Value {
    Str(String),
    Int(i32),
    Num(f64),
    Bool(bool),
    Date(Date),
    Timestamp(Timestamp),
    ByteBuf(Vec<u8>),
    Null,
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Value::Str(ref val) => serializer.serialize_str(val),
            Value::Int(val) => serializer.serialize_i32(val),
            Value::Num(val) => serializer.serialize_f64(val),
            Value::Bool(val) => serializer.serialize_bool(val),
            Value::Date(ref val) => serializer.serialize_str(&val.to_string()),
            Value::Timestamp(ref val) => serializer.serialize_str(&val.to_string()),
            Value::ByteBuf(ref val) => serializer.collect_seq(val),
            Value::Null => serializer.serialize_none(),
        }
    }
}

impl From<FieldValue> for Value {
    fn from(value: FieldValue) -> Self {
        match value {
            FieldValue::Binary(val) | FieldValue::General(val) => Value::ByteBuf(val),
            FieldValue::Character(val) | FieldValue::Memo(val) => Value::Str(val),
            FieldValue::Date(year, month, day) => Value::Date(Date::from((year, month, day))),
            FieldValue::Float(val) => Value::Num(val),
            FieldValue::Integer(val) => Value::Int(val),
            FieldValue::Logical(val) => Value::Bool(val),
            FieldValue::Numeric(val) => Value::Num(val),
            FieldValue::Timestamp(year, month, day, hour, minute, second) => {
                Value::Timestamp(Timestamp::from((year, month, day, hour, minute, second)))
            }
            FieldValue::Null => Value::Null,
        }
    }
}
