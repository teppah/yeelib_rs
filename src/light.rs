use std::collections::HashSet;
use std::convert::TryFrom;
use std::str::FromStr;
use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct Light {
    pub ip_addr: Ipv4Addr,
    pub id: String,
    pub model: String,
    // ignore fw_ver for now
    pub support: HashSet<String>,
    pub power: PowerStatus,
    pub bright: u8,
    pub color_mode: ColorMode,

    // only valid for ColorMode::ColorTemperature
    pub ct: u16,

    // only valid for ColorMode::Color
    pub rgb: Rgb,

    // only valid for ColorMode::Hsv
    pub hue: u16,
    // only valid for ColorMode::Hsv
    pub sat: u8,

    pub name: String,
}


#[derive(Debug, Copy, Clone)]
pub enum PowerStatus {
    On,
    Off,
}

impl ToString for PowerStatus {
    fn to_string(&self) -> String {
        match self {
            Self::On => "on",
            Self::Off => "off"
        }.to_string()
    }
}

#[derive(Debug)]
pub struct ParsePowerStatusError(String);

impl FromStr for PowerStatus {
    type Err = ParsePowerStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            _ => Err(ParsePowerStatusError(String::from(format!("Failed to parse \"{}\" into PowerStatus", s))))
        }
    }
}

impl From<String> for PowerStatus {
    fn from(s: String) -> Self {
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
    pub fn get_from_number<N: Into<u8>>(n: N) -> Option<ColorMode> {
        let n = n.into();
        match n {
            1 => Some(Self::Color),
            2 => Some(Self::ColorTemperature),
            3 => Some(Self::Hsv),
            _ => None
        }
    }
}

impl<N: Into<u8>> From<N> for ColorMode {
    fn from(val: N) -> Self {
        Self::get_from_number(val).unwrap()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Rgb {
    red: u8,
    blue: u8,
    green: u8,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}