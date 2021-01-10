use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use crate::err::YeeError;

const HEX_FFFFFF: u32 = 16777215;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PowerStatus {
    On,
    Off,
}

impl PowerStatus {
    pub fn flip(&self) -> PowerStatus {
        match self {
            Self::Off => Self::On,
            Self::On => Self::Off
        }
    }
}

impl Display for PowerStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
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
            _ => Err(YeeError::ParseFieldFailed { field_name: "power_status", source: None })
        }
    }
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ColorMode {
    Color,
    ColorTemperature,
    Hsv,
}

impl Display for ColorMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({})", match self {
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
            _ => Err(YeeError::ParseFieldFailed { field_name: "color_mode", source: None })
        }
    }
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Rgb {
    pub red: u8,
    pub blue: u8,
    pub green: u8,
}

impl Rgb {
    pub fn empty() -> Self {
        Rgb { red: 0, blue: 0, green: 0 }
    }

    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Rgb { red, green, blue }
    }

    pub fn get_num(&self) -> u32 {
        self.red as u32 * 65536 + self.green as u32 * 256 + self.blue as u32
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let hex = self.red as u32 * 256 * 256 + self.green as u32 * 256 + self.blue as u32;
        write!(f, "#{:x}", hex)
    }
}

impl FromStr for Rgb {
    type Err = YeeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = match s.parse::<u32>() {
            Ok(v) => v,
            Err(e) => { return Err(YeeError::ParseFieldFailed { source: Some(e), field_name: "rgb" }); }
        };
        if !(0..=HEX_FFFFFF).contains(&val) {
            Err(YeeError::ParseFieldFailed { field_name: "rgb", source: None })
        } else {
            // https://math.stackexchange.com/questions/1635999/algorithm-to-convert-integer-to-3-variables-rgb
            let blue = (val % 256) as u8;
            let green = (((val - blue as u32) / 256) % 256) as u8;
            let red = ((((val - blue as u32) / 256) - green as u32) / 256) as u8;
            Ok(Rgb { red, green, blue })
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_parse_power_status() -> anyhow::Result<()> {
        // given
        let correct_1 = "on";
        let correct_2 = "off";

        // when
        let result_1 = correct_1.parse::<PowerStatus>()?;
        let result_2 = correct_2.parse::<PowerStatus>()?;

        // then
        assert_eq!(result_1, PowerStatus::On);
        assert_eq!(result_2, PowerStatus::Off);
        Ok(())
    }

    #[test]
    fn incorrect_parse_power_status() {
        // given
        let incorrect = "ofon";

        // when
        let incorrect_parsed = incorrect.parse::<PowerStatus>();

        // then
        assert!(incorrect_parsed.is_err());
    }

    #[test]
    fn correct_parse_color_mode() -> anyhow::Result<()> {
        // given
        let correct_1 = "1";
        let correct_2 = "2";
        let correct_3 = "3";

        // when
        let parsed_1 = correct_1.parse::<ColorMode>()?;
        let parsed_2 = correct_2.parse::<ColorMode>()?;
        let parsed_3 = correct_3.parse::<ColorMode>()?;

        // then
        assert_eq!(parsed_1, ColorMode::Color);
        assert_eq!(parsed_2, ColorMode::ColorTemperature);
        assert_eq!(parsed_3, ColorMode::Hsv);

        Ok(())
    }

    #[test]
    fn incorrect_parse_color_mode() {
        // given
        let incorrect = "55";

        // when
        let incorrect_parsed = incorrect.parse::<ColorMode>();

        // then
        assert!(incorrect_parsed.is_err());
    }

    #[test]
    fn correct_parse_rgb() -> anyhow::Result<()> {
        // given
        let rgb_1 = "1518204"; // hex: 172A7C
        let rgb_2 = "16777215"; // hex: FFFFFF
        let rgb_3 = "0";

        // when
        let parsed_1 = rgb_1.parse::<Rgb>()?;
        let parsed_2 = rgb_2.parse::<Rgb>()?;
        let parsed_3 = rgb_3.parse::<Rgb>()?;

        // then
        assert_eq!(parsed_1.red, 23);
        assert_eq!(parsed_1.blue, 124);
        assert_eq!(parsed_1.green, 42);

        assert_eq!(parsed_2.red, 255);
        assert_eq!(parsed_2.blue, 255);
        assert_eq!(parsed_2.green, 255);

        assert_eq!(parsed_3.red, 0);
        assert_eq!(parsed_3.blue, 0);
        assert_eq!(parsed_3.green, 0);

        Ok(())
    }

    #[test]
    fn incorrect_parse_rgb() {
        // given
        let incorrect_1 = "-5";
        let incorrect_2 = "564123564";
        let incorrect_3 = "fsdkl";

        // when
        let parsed_1 = incorrect_1.parse::<Rgb>();
        let parsed_2 = incorrect_2.parse::<Rgb>();
        let parsed_3 = incorrect_3.parse::<Rgb>();

        // then
        assert!(parsed_1.is_err());
        assert!(parsed_2.is_err());
        assert!(parsed_3.is_err());
    }
}
