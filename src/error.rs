use serde::export::Formatter;
use std::error::Error as StdError;
use std::fmt;

use crate::dbf::FieldType;

#[derive(Debug)]
pub struct UnsupportedFieldTypeError(pub char);

impl fmt::Display for UnsupportedFieldTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Unsupported field type: {}", self.0)
    }
}

impl StdError for UnsupportedFieldTypeError {}

#[derive(Debug)]
pub struct FieldParseError {
    field: String,
    value: String,
    field_type: FieldType,
}

impl FieldParseError {
    pub fn new<S>(field: S, value: S, field_type: FieldType) -> Self
    where
        S: Into<String>,
        // V: Into<Vec<u8>>,
    {
        Self {
            field_type,
            field: field.into(),
            value: value.into(),
        }
    }
}

impl fmt::Display for FieldParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "Field '{}' ({}) could not be parsed from '{:?}'",
            self.field, self.field_type, self.value
        )
    }
}

impl StdError for FieldParseError {}

#[derive(Debug)]
pub struct NoSuchFieldError {
    field: String,
}

impl NoSuchFieldError {
    pub fn new<S: Into<String>>(field: S) -> Self {
        Self {
            field: field.into(),
        }
    }
}

impl fmt::Display for NoSuchFieldError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Field not found: {}", self.field)
    }
}

impl StdError for NoSuchFieldError {}

#[derive(Debug)]
pub struct DeserializeError {
    message: String,
    source: Option<Box<dyn StdError>>,
}

impl fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Deserialize Error: {}", self.message)
    }
}

impl std::error::Error for DeserializeError {}
impl serde::de::Error for DeserializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Self {
            message: format!("{}", msg),
            source: None,
        }
    }
}

impl<S: Into<String>> From<S> for DeserializeError {
    fn from(e: S) -> Self {
        Self {
            message: e.into(),
            source: None,
        }
    }
}

impl From<FieldParseError> for DeserializeError {
    fn from(e: FieldParseError) -> Self {
        Self {
            message: format!("failed to parse field {}", &e.field),
            source: Some(e.into()),
        }
    }
}

impl From<NoSuchFieldError> for DeserializeError {
    fn from(e: NoSuchFieldError) -> Self {
        Self {
            message: format!("no field with name {}", &e.field),
            source: Some(e.into()),
        }
    }
}
