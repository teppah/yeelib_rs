use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::net::SocketAddrV4;

use crate::err::YeeError;
use crate::fields::{ColorMode, PowerStatus, Rgb};

#[derive(Debug, Eq)]
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

macro_rules! get {
    ($map: expr, $target: expr) => {
        match $map.get($target) {
            None => return Err(YeeError::ParseFieldError(format!("Did not find required status \"{}\"", $target))),
            Some(val) => val.as_ref()
        }
    };
}

impl Light {
    pub fn from_fields<S: AsRef<str>>(fields: &HashMap<&str, S>, location: SocketAddrV4) -> Result<Light, YeeError> {
        let id: String = get!(fields, "id").to_string();
        let model: String = get!(fields, "model").to_string();
        let fw_ver = get!(fields, "fw_ver").parse::<u8>()?;
        let power: PowerStatus = get!(fields, "power").parse()?;
        let support: HashSet<String> = get!(fields, "support").trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        let bright: u8 = get!(fields, "bright").parse()?;
        let color_mode: ColorMode = get!(fields, "color_mode").parse()?;
        let ct: u16 = get!(fields, "ct").parse()?;
        let rgb: Rgb = get!(fields, "rgb").parse()?;
        let hue: u16 = get!(fields, "hue").parse()?;
        let sat: u8 = get!(fields, "sat").parse()?;
        let name: String = get!(fields, "name").to_string();
        Ok(Light { location, id, model, fw_ver, power, support, bright, color_mode, ct, rgb, hue, sat, name })
    }

    pub fn location(&self) -> &SocketAddrV4 {
        &self.location
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn model(&self) -> &str {
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

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Hash for Light {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.id.as_bytes());
    }
}

impl PartialEq for Light {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap};
    use std::net::{Ipv4Addr, SocketAddrV4};

    use super::*;

    macro_rules! map {
        ($($key:expr => $value: expr), *) => {{
            let mut map = HashMap::new();
            $(map.insert($key,$value);)*
            map
        }};
    }

    pub(crate) fn get_map() -> HashMap<&'static str, &'static str> {
        let mut m: HashMap<&str, &str> =
            map!(
            "id" => "0x1234",
            "model" => "floor",
            "fw_ver" => "40", // can fail
            "power" => "on", // can fail
            "bright" => "34", // can fail
            "color_mode" => "2", // can fail
            "ct" => "0", // can fail
            "rgb" => "657930", // 0A0A0A, can fail
            "hue" => "314", // can fail
            "sat" => "12", // can fail
            "name" => "room_light"
            );
        let support = "get_power set_power get_rgb set_rgb";
        m.insert("support", support);
        m
    }

    #[test]
    fn get_correct_location() -> anyhow::Result<()> {
        // given
        let map = get_map();
        let addr = SocketAddrV4::new(Ipv4Addr::new(192, 168, 0, 42), 1234);

        // when
        let light = Light::from_fields(&map, addr)?;

        // then
        assert_eq!(light.location(), &addr);
        Ok(())
    }

    macro_rules! generate_getter_tests {
        () => {};
        ($field:ident, $($tail: tt)*) => {
            #[test]
            fn $field() -> anyhow::Result<()> {
                use std::net::{Ipv4Addr, SocketAddrV4};

                // given
                let map = get_map();
                let addr = SocketAddrV4::new(Ipv4Addr::new(192, 168, 0, 42), 1234);

                // when
                let light = Light::from_fields(&map, addr)?;

                // then
                assert_eq!(map.get(stringify!($field)).unwrap(), &light.$field().to_string());
                Ok(())
            }
        };
        ($field:ident => $expected: expr, $($tail: tt)*) => {
            #[test]
            fn $field() -> anyhow::Result<()> {
                use std::net::{Ipv4Addr, SocketAddrV4};

                // given
                let map = get_map();
                let addr = SocketAddrV4::new(Ipv4Addr::new(192, 168, 0, 42), 1234);

                // when
                let light = Light::from_fields(&map, addr)?;

                // then
                assert_eq!(&$expected, light.$field());
                Ok(())
            }
        };

    }

    mod test_get_parse {
        use super::*;

        generate_getter_tests!(
            id,
            model,
            fw_ver,
            power,
            bright,
            color_mode => ColorMode::ColorTemperature,
            ct,
            rgb => Rgb { red: 10, green: 10, blue: 10 },
            hue,
            sat,
            name, );
    }

    macro_rules! generate_parse_fail_tests {
        ($($field:ident), *) => {
            $(
                #[test]
                fn $field() {
                    use std::net::{Ipv4Addr, SocketAddrV4};

                    // given
                    let mut map = get_map();
                    let addr = SocketAddrV4::new(Ipv4Addr::new(192, 168, 0, 42), 1234);
                    map.remove(stringify!($field)).unwrap();

                    // when
                    let fail = Light::from_fields(&map, addr);

                    // then
                    assert!(fail.is_err());
                }
            )*
        };
    }

    mod test_parse_fail {
        use super::*;

        generate_parse_fail_tests!(
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
            name);
    }

    #[test]
    fn get_correct_support() -> anyhow::Result<()> {
        // given
        let map = get_map();
        let addr = SocketAddrV4::new(Ipv4Addr::new(192, 168, 0, 42), 1234);
        let expected_fields: HashSet<String> = map.get("support").unwrap().split_whitespace().map(|s| s.to_string()).collect();

        // when
        let light = Light::from_fields(&map, addr)?;

        // then
        let support = light.support();
        assert_eq!(&expected_fields, support);
        Ok(())
    }
}
