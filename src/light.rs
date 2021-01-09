use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::net::{SocketAddrV4, TcpStream, SocketAddr};

use crate::err::YeeError;
use crate::fields::{ColorMode, PowerStatus, Rgb};
use regex::Regex;

use serde_json::json;

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

    // wrapped in option for late init
    // if successfully made a Light, can always assume it is valid
    pub(crate) connection: Option<TcpStream>,
}

use lazy_static::*;
use crate::req::{Req, Transition};
use std::io::{Write, Read};
use serde_json::Value;
use serde_json::value::Value::Number;
lazy_static! {
    static ref MATCH_IP: Regex = Regex::new("yeelight://(.*)").unwrap();
}

macro_rules! get_field {
    // for strings
    ($map: expr, $field: expr) => {
        $map.get($field)
            .map(|s| s.as_ref())
            .ok_or(YeeError::FieldNotFound { field_name: stringify!($field) })
    };
    // for primitive types
    ($map: expr, $field: expr, $target_type: ty) => {
        $map.get($field)
            .ok_or(YeeError::FieldNotFound { field_name: stringify!($field) })
            .and_then(|s| {
                let s = s.as_ref();
                s.parse::<$target_type>()
                    .map_err(|e| YeeError::ParseFieldFailed { field_name: stringify!($field), source: Some(e)})
            })
    };
    // for custom FromStr types
    ($map: expr, $field: expr, $target_type: ty, $is_custom_type_marker: expr) => {
        $map.get($field)
            .ok_or(YeeError::FieldNotFound { field_name: stringify!($field) })
            .and_then(|s| {
                let s = s.as_ref();
                s.parse::<$target_type>()
            })
    };
}

impl Light {
    pub fn from_fields<S: AsRef<str>>(fields: &HashMap<&str, S>) -> Result<Light, YeeError> {
        let id = get_field!(fields, "id")?.to_string();
        let model = get_field!(fields, "model")?.to_string();
        let fw_ver = get_field!(fields, "fw_ver", u8)?;
        let power = get_field!(fields, "power", PowerStatus, true)?;
        let support: HashSet<String> = get_field!(fields, "support")?.trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        let bright = get_field!(fields, "bright", u8)?;
        let color_mode = get_field!(fields, "color_mode", ColorMode, true)?;
        let ct = get_field!(fields, "ct", u16)?;
        let rgb = get_field!(fields, "rgb", Rgb, true)?;
        let hue: u16 = get_field!(fields, "hue", u16)?;
        let sat = get_field!(fields, "sat", u8)?;
        let name = get_field!(fields, "name")?.to_string();

        let location = get_field!(fields,"Location")?;
        let captures = MATCH_IP
            .captures(location)
            .ok_or(YeeError::ParseFieldFailed { field_name: "Location", source: None })
            .and_then(|c| c
                .get(1)
                .ok_or(YeeError::ParseFieldFailed { field_name: "Location", source: None })
            )
            .and_then(|m| m
                .as_str()
                .parse::<SocketAddr>()
                .map_err(|_| YeeError::ParseFieldFailed { field_name: "Location", source: None })
            )
            ?;
        let location = match captures {
            SocketAddr::V4(v4) => v4,
            _ => panic!("Light should not have an IPv6 address")
        };

        Ok(Light { location, id, model, fw_ver, power, support, bright, color_mode, ct, rgb, hue, sat, name, connection: None })
    }

    pub(crate) fn init(&mut self) -> Result<(), YeeError> {
        if self.connection.is_some() {
            return Ok(());
        }
        let connection = TcpStream::connect(self.location)?;
        self.connection = Some(connection);
        Ok(())
    }

    pub fn set_ct_abx(&mut self, temperature: u16, transition: Transition) -> Result<(), YeeError> {
        if !self.support.contains("set_ct_abx") {
            return Err(YeeError::MethodNotSupported { method_name: "set_ct_abx" });
        }
        // SPEC IS WRONG: temperature bounds should be 2700-6500
        if !(2700..=6500).contains(&temperature) {
            return Err(YeeError::InvalidValue { field_name: "ct", value: temperature.to_string() });
        }
        let connection = self.connection.as_mut().unwrap();
        let rand_val = fastrand::i32(1..65536);
        let req = Req::new(rand_val,
                           "set_ct_abx".to_string(),
                           vec![json!(temperature), json!(transition.text()), json!(transition.value())]);
        let mut json = serde_json::to_string(&req).unwrap();
        json.push_str("\r\n");
        println!("{}", json);
        connection.write(json.as_bytes())?;

        let mut buf = [0u8; 128];
        connection.read(&mut buf)?;
        self.ct = temperature;
        Ok(())
    }

    pub fn set_bright(&mut self, brightness: u8, transition: Transition) -> Result<(), YeeError> {
        if !self.support.contains("set_bright") {
            return Err(YeeError::MethodNotSupported { method_name: "set_bright" });
        }
        if !(1..=100).contains(&brightness) {
            return Err(YeeError::InvalidValue { field_name: "bright", value: brightness.to_string() });
        }
        let connection = self.connection.as_mut().unwrap();
        let rand_val = fastrand::i32(1..65536);
        let req = Req::new(rand_val,
                           "set_bright".to_string(),
                           vec![json!(brightness), json!(transition.text()), json!(transition.value())]);
        let mut json = serde_json::to_string(&req).unwrap();
        json.push_str("\r\n");
        connection.write(json.as_bytes())?;

        let mut buf = [0u8; 128];
        connection.read(&mut buf)?;
        self.bright = brightness;
        Ok(())
    }

    pub fn set_rgb(&mut self, rgb: Rgb, transition: Transition) -> Result<(), YeeError> {
        if !self.support.contains("set_rgb") {
            return Err(YeeError::MethodNotSupported { method_name: "set_rgb" });
        }
        let connection = self.connection.as_mut().unwrap();
        let rand_val = fastrand::i32(1..65536);
        let req = Req::new(rand_val,
                           "set_rgb".to_string(),
                           vec![json!(rgb.get_num()), json!(transition.text()), json!(transition.value())]);
        let mut json = serde_json::to_string(&req).unwrap();
        json.push_str("\r\n");
        connection.write(json.as_bytes())?;

        let mut buf = [0u8; 128];
        connection.read(&mut buf)?;
        println!("{}", String::from_utf8_lossy(&buf));
        self.rgb = rgb;
        Ok(())
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

impl Eq for Light {}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap};
    use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, IpAddr};

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
            "name" => "room_light",
            "Location" => "yeelight://127.0.0.1:13454"
            );
        let support = "get_power set_power get_rgb set_rgb";
        m.insert("support", support);
        m
    }

    #[test]
    fn get_correct_location() -> anyhow::Result<()> {
        // given
        let map = get_map();
        let expected_addr = match SocketAddr::new(IpAddr::from(Ipv4Addr::LOCALHOST), 13454) {
            SocketAddr::V4(v4) => v4,
            _ => unreachable!()
        };

        // when
        let light = Light::from_fields(&map)?;

        // then
        assert_eq!(*light.location(), expected_addr);
        Ok(())
    }

    macro_rules! generate_getter_tests {
        () => {};
        ($field:ident, $($tail: tt)*) => {
            #[test]
            fn $field() -> anyhow::Result<()> {

                // given
                let map = get_map();

                // when
                let light = Light::from_fields(&map)?;

                // then
                assert_eq!(map.get(stringify!($field)).unwrap(), &light.$field().to_string());
                Ok(())
            }
            generate_getter_tests!($($tail)*);
        };
        ($field:ident => $expected: expr, $($tail: tt)*) => {
            #[test]
            fn $field() -> anyhow::Result<()> {

                // given
                let map = get_map();

                // when
                let light = Light::from_fields(&map)?;

                // then
                assert_eq!(&$expected, light.$field());
                Ok(())
            }
            generate_getter_tests!($($tail)*);
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

                    // given
                    let mut map = get_map();
                    map.remove(stringify!($field)).unwrap();

                    // when
                    let fail = Light::from_fields(&map);

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
        let expected_fields: HashSet<String> = map.get("support").unwrap().split_whitespace().map(|s| s.to_string()).collect();

        // when
        let light = Light::from_fields(&map)?;

        // then
        let support = light.support();
        assert_eq!(&expected_fields, support);
        Ok(())
    }

    #[test]
    fn correctly_connects() -> anyhow::Result<()> {
        // given
        let map = get_map();
        let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 13454);
        let _fake_listener = TcpListener::bind(addr)?;

        // when
        let mut light = Light::from_fields(&map)?;
        light.init()?;

        // then
        assert!(light.connection.is_some());
        Ok(())
    }
}
