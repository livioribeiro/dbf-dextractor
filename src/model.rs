use std::string::ToString;

use serde::{Deserialize, Serialize, Serializer};

#[derive(Deserialize, Clone, Debug)]
pub struct Date(u16, u8, u8);

impl Date {
    pub fn year(&self) -> u16 {
        self.0
    }

    pub fn month(&self) -> u8 {
        self.1
    }

    pub fn day(&self) -> u8 {
        self.2
    }
}

impl From<(u16, u8, u8)> for Date {
    fn from((year, month, day): (u16, u8, u8)) -> Self {
        Self(year, month, day)
    }
}

impl ToString for Date {
    fn to_string(&self) -> String {
        format!("{}-{:02}-{:02}", self.0, self.1, self.2)
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Timestamp(u16, u8, u8, u8, u8, u8);

impl Timestamp {
    pub fn year(&self) -> u16 {
        self.0
    }

    pub fn month(&self) -> u8 {
        self.1
    }

    pub fn day(&self) -> u8 {
        self.2
    }

    pub fn hour(&self) -> u8 {
        self.3
    }

    pub fn minute(&self) -> u8 {
        self.4
    }

    pub fn second(&self) -> u8 {
        self.5
    }
}

impl From<(u16, u8, u8, u8, u8, u8)> for Timestamp {
    fn from((year, month, day, hour, minute, second): (u16, u8, u8, u8, u8, u8)) -> Self {
        Self(year, month, day, hour, minute, second)
    }
}

impl ToString for Timestamp {
    fn to_string(&self) -> String {
        format!(
            "{}-{:02}-{:02}T{:02}:{:02}:{:02}",
            self.0, self.1, self.2, self.3, self.4, self.5
        )
    }
}

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
