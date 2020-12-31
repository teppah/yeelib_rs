use std::fmt::{Display, Formatter};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum PowerStatus {
    On,
    Off,
}

impl Display for PowerStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Off => "off",
            Self::On => "on"
        })
    }
}

#[derive(Debug)]
pub struct ParseFieldError(String);

impl Display for ParseFieldError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse field error: {}", self.0)
    }
}

impl std::error::Error for ParseFieldError {}


impl FromStr for PowerStatus {
    type Err = ParseFieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            _ => Err(ParseFieldError(String::from(format!("Failed to parse \"{}\" into PowerStatus", s))))
        }
    }
}

impl From<&str> for PowerStatus {
    fn from(s: &str) -> Self {
        Self::from_str(&s).unwrap()
    }
}


#[derive(Debug, Copy, Clone)]
pub enum ColorMode {
    Color,
    ColorTemperature,
    Hsv,
}

impl ColorMode {
    pub fn from_number<N: Into<u8>>(n: N) -> Option<ColorMode> {
        let n = n.into();
        match n {
            1 => Some(Self::Color),
            2 => Some(Self::ColorTemperature),
            3 => Some(Self::Hsv),
            _ => None
        }
    }
}

impl FromStr for ColorMode {
    type Err = ParseFieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number = match s.parse::<u8>() {
            Ok(val) => val,
            Err(e) => return Err(ParseFieldError(format!("Failed to parse \"{}\" into u8: {}", s, e)))
        };
        match Self::from_number(number) {
            Some(val) => Ok(val),
            None => Err(ParseFieldError(format!("Failed to parse \"{}\" into ColorMode", s)))
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
