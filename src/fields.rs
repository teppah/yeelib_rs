use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use crate::err::YeeError;

#[derive(Debug, Copy, Clone)]
pub enum PowerStatus {
    On,
    Off,
}

impl Display for PowerStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "PowerStatus({})", match self {
            Self::Off => "off",
            Self::On => "on"
        })
    }
}

impl FromStr for PowerStatus {
    type Err = YeeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            _ => Err(YeeError::ParseFieldError(format!("Failed to parse \"{}\" into PowerStatus", s)))
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub enum ColorMode {
    Color,
    ColorTemperature,
    Hsv,
}

impl Display for ColorMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ColorMode({})", match self {
            Self::Color => "color, id=1",
            Self::ColorTemperature => "color_temperature, id=2",
            Self::Hsv => "hsv, id=3"
        })
    }
}

impl FromStr for ColorMode {
    type Err = YeeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(ColorMode::Color),
            "2" => Ok(ColorMode::ColorTemperature),
            "3" => Ok(ColorMode::Hsv),
            _ => Err(YeeError::ParseFieldError(format!("Failed to parse \"{}\" into ColorMode", s)))
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Rgb {
    pub red: u8,
    pub blue: u8,
    pub green: u8,
}

impl Rgb {
    pub fn empty() -> Self {
        Rgb { red: 0, blue: 0, green: 0 }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
