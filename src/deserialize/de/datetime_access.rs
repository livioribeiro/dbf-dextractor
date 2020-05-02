use serde::de::{DeserializeSeed, IntoDeserializer, SeqAccess};

use crate::error::DeserializeError;

pub struct DateAccess {
    year: Option<u16>,
    month: Option<u8>,
    day: Option<u8>,
}

impl DateAccess {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Self {
            year: Some(year),
            month: Some(month),
            day: Some(day),
        }
    }
}

impl<'de> SeqAccess<'de> for DateAccess {
    type Error = DeserializeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if let Some(value) = self.year.take() {
            seed.deserialize(value.into_deserializer()).map(Some)
        } else if let Some(value) = self.month.take() {
            seed.deserialize(value.into_deserializer()).map(Some)
        } else if let Some(value) = self.day.take() {
            seed.deserialize(value.into_deserializer()).map(Some)
        } else {
            Ok(None)
        }
    }
}

pub struct TimestampAccess {
    year: Option<u16>,
    month: Option<u8>,
    day: Option<u8>,
    hour: Option<u8>,
    minute: Option<u8>,
    second: Option<u8>,
}

impl TimestampAccess {
    pub fn new(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Self {
        Self {
            year: Some(year),
            month: Some(month),
            day: Some(day),
            hour: Some(hour),
            minute: Some(minute),
            second: Some(second),
        }
    }
}

impl<'de> SeqAccess<'de> for TimestampAccess {
    type Error = DeserializeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if let Some(value) = self.year.take() {
            seed.deserialize(value.into_deserializer()).map(Some)
        } else if let Some(value) = self.month.take() {
            seed.deserialize(value.into_deserializer()).map(Some)
        } else if let Some(value) = self.day.take() {
            seed.deserialize(value.into_deserializer()).map(Some)
        } else if let Some(value) = self.hour.take() {
            seed.deserialize(value.into_deserializer()).map(Some)
        } else if let Some(value) = self.minute.take() {
            seed.deserialize(value.into_deserializer()).map(Some)
        } else if let Some(value) = self.second.take() {
            seed.deserialize(value.into_deserializer()).map(Some)
        } else {
            Ok(None)
        }
    }
}
