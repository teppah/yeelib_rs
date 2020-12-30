use std::collections::HashSet;
use std::convert::TryFrom;
use std::str::FromStr;
use std::net::{SocketAddrV4};
use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Light {
    location: SocketAddrV4,
    id: String,
    model: String,
    fw_ver: u8,
    support: HashSet<String>,
    power: PowerStatus,
    bright: u8,
    color_mode: ColorMode,

    // only valid for ColorMode::ColorTemperature
    ct: u16,

    // only valid for ColorMode::Color
    rgb: Rgb,

    // only valid for ColorMode::Hsv
    hue: u16,
    // only valid for ColorMode::Hsv
    sat: u8,

    name: String,
}

impl Light {
    pub fn new(location: SocketAddrV4,
               id: String,
               model: String,
               fw_ver: u8,
               support: HashSet<String>,
               power: PowerStatus,
               bright: u8,
               color_mode: ColorMode,
               ct: u16,
               rgb: Rgb,
               hue: u16,
               sat: u8,
               name: String) -> Self {
        Light {
            location,
            id,
            model,
            fw_ver,
            support,
            power,
            bright,
            color_mode,
            ct,
            rgb,
            hue,
            sat,
            name,
        }
    }
    pub fn location(&self) -> &SocketAddrV4 {
        &self.location
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn model(&self) -> &String {
        &self.model
    }

    pub fn fw_ver(&self) -> u8 {
        self.fw_ver
    }

    pub fn support(&self) -> &HashSet<String> {
        &self.support
    }

    pub fn power(&self) -> &PowerStatus {
        &self.power
    }

    pub fn bright(&self) -> u8 {
        self.bright
    }

    pub fn color_mode(&self) -> &ColorMode {
        &self.color_mode
    }

    pub fn ct(&self) -> u16 {
        self.ct
    }

    pub fn rgb(&self) -> &Rgb {
        &self.rgb
    }

    pub fn hue(&self) -> u16 {
        self.hue
    }

    pub fn sat(&self) -> u8 {
        self.sat
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl Hash for Light {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.id.as_bytes());
    }
}


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
pub struct ParseStateError(String);

impl FromStr for PowerStatus {
    type Err = ParseStateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            _ => Err(ParseStateError(String::from(format!("Failed to parse \"{}\" into PowerStatus", s))))
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

impl<N: Into<u8>> From<N> for ColorMode {
    fn from(val: N) -> Self {
        Self::from_number(val).unwrap()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Rgb {
    pub red: u8,
    pub blue: u8,
    pub green: u8,
}

impl Rgb {
    fn empty() -> Self {
        Rgb { red: 0, blue: 0, green: 0 }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}