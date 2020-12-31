use crate::fields::{ColorMode, PowerStatus, Rgb};
use std::net::SocketAddrV4;
use std::collections::{HashSet, HashMap};
use std::hash::{Hasher, Hash};

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

macro_rules! get {
    ($map: expr, $target: expr) => {
        match $map.get($target) {
            None => anyhow::bail!("Did not find required status \"{}\"", $target),
            Some(val) => val.as_ref()
        }
    };
}

impl Light {
    pub(crate) fn from_hashmap<S: AsRef<str>>(map: &HashMap<&str, S>, location: SocketAddrV4) -> anyhow::Result<Light> {
        let id: String = get!(map, "id").to_string();
        let model: String = get!(map, "model").to_string();
        let fw_ver: u8 = get!(map, "fw_ver").parse()?;
        let power: PowerStatus = get!(map, "power").parse()?;
        let support: HashSet<String> = get!(map, "support").trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        let bright: u8 = get!(map, "bright").parse()?;
        let color_mode: ColorMode = get!(map, "bright").parse()?;
        let ct: u16 = get!(map, "ct").parse()?;
        // TODO: implement rgb
        let rgb = Rgb::empty();
        let hue: u16 = get!(map, "hue").parse()?;
        let sat: u8 = get!(map, "sat").parse()?;
        let name: String = get!(map, "name").to_string();
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

impl Eq for Light {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
