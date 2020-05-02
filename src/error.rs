use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;

use serde::export::Formatter;

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
    field_name: String,
    field_type: FieldType,
    source: Option<Box<(dyn StdError + 'static)>>,
}

impl FieldParseError {
    pub fn new<S>(field_name: S, field_type: FieldType, source: Option<Box<dyn StdError>>) -> Self
    where
        S: Into<String>,
    {
        Self {
            field_name: field_name.into(),
            field_type,
            source,
        }
    }
}

impl fmt::Display for FieldParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        if let Some(source) = &self.source {
            write!(
                f,
                "Field '{}' ({}) could not be parsed: {}",
                self.field_name, self.field_type, source
            )
        } else {
            write!(
                f,
                "Field '{}' ({}) could not be parsed",
                self.field_name, self.field_type
            )
        }
    }
}

impl StdError for FieldParseError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        if let Some(e) = &self.source {
            Some(&**e)
        } else {
            None
        }
    }
}

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
        write!(f, "Field '{}' does not exist", self.field)
    }
}

impl StdError for NoSuchFieldError {}

#[derive(Debug)]
pub struct DeserializeError {
    code: ErrorCode,
    record: usize,
    field: String,
}

impl DeserializeError {
    pub fn missing_memo_file() -> Self {
        Self {
            code: ErrorCode::MissingMemoFile,
            record: 0,
            field: "".to_owned(),
        }
    }

    pub fn field_parse<S: Into<String>>(record: usize, field: S) -> Self {
        Self {
            code: ErrorCode::FieldParse,
            record,
            field: field.into(),
        }
    }

    pub fn unexpected_end_of_record() -> Self {
        Self {
            code: ErrorCode::UnexpectedEndOfRecord,
            record: 0,
            field: "".to_owned(),
        }
    }

    pub fn expected<S: Into<String>>(field_type: FieldType, record: usize, field: S) -> Self {
        Self {
            code: ErrorCode::Expected(field_type),
            record,
            field: field.into(),
        }
    }

    pub fn unexpected_null<S: Into<String>>(record: usize, field: S) -> Self {
        Self {
            code: ErrorCode::UnexpectedNull,
            record,
            field: field.into(),
        }
    }

    pub fn expected_null<S: Into<String>>(record: usize, field: S) -> Self {
        Self {
            code: ErrorCode::ExpectedNull,
            record,
            field: field.into(),
        }
    }

    pub fn tuple_length(length: usize, record: usize) -> Self {
        Self {
            code: ErrorCode::TupleLength(length, record),
            record: 0,
            field: "".to_owned(),
        }
    }
}

#[derive(Debug)]
pub enum ErrorCode {
    Custom(String),
    Io(IoError),
    Expected(FieldType),
    TupleLength(usize, usize),
    UnexpectedNull,
    ExpectedNull,
    NoSuchField,
    FieldParse,
    MissingMemoFile,
    UnexpectedEndOfRecord,
}

impl fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Deserialize error at {}:{}", self.field, self.record)
    }
}

impl std::error::Error for DeserializeError {}

impl serde::de::Error for DeserializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Self {
            code: ErrorCode::Custom(msg.to_string()),
            record: 0,
            field: "".to_owned(),
        }
    }
}

impl From<IoError> for DeserializeError {
    fn from(e: IoError) -> Self {
        Self {
            code: ErrorCode::Io(e),
            record: 0,
            field: "".to_owned(),
        }
    }
}

impl From<NoSuchFieldError> for DeserializeError {
    fn from(e: NoSuchFieldError) -> Self {
        Self {
            code: ErrorCode::NoSuchField,
            record: 0,
            field: e.field,
        }
    }
}
