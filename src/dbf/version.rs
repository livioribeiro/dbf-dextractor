use std::convert::TryFrom;

#[derive(Debug, Copy, Clone)]
pub enum Version {
    FoxBase,
    DBase3,
    DBase4,
    VisualFoxPro,
    FoxPro2,
}

impl TryFrom<u8> for Version {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let value = match value {
            0b0000_0010 => Version::FoxBase,
            0b0000_0011 | 0b1000_0011 => Version::DBase3,
            0b0011_0000 | 0b0011_0001 | 0b0011_0010 => Version::VisualFoxPro,
            0b0100_0011 | 0b0110_0011 | 0b1000_1011 | 0b1100_1011 => Version::DBase4,
            0b1111_0101 | 0b1111_1011 => Version::FoxPro2,
            _ => return Err("Unknown version".to_owned()),
        };

        Ok(value)
    }
}
