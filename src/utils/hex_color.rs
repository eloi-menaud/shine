use iced::Color;
use regex::Regex;
use serde::{Deserialize, Deserializer, de};
use std::str::FromStr;

const HEX_REGEX: &str = r"^#?([[:xdigit:]]{6})([[:xdigit:]]{2})?$";

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HexColor(pub Color);

impl From<HexColor> for Color {
    fn from(hex: HexColor) -> Self {
        hex.0
    }
}

impl<'de> Deserialize<'de> for HexColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let regexp = Regex::new(r"^#?([[:xdigit:]]{6})([[:xdigit:]]{2})?$").unwrap();

        let caps = regexp.captures(&s).ok_or_else(|| {
            de::Error::custom("Invalid Hex Color format : waiting #RRGGBB or #RRGGBBAA, get '{s}'")
        })?;

        let full_hex = format!(
            "{}{}",
            caps.get(1).unwrap().as_str(),
            caps.get(2).map(|m| m.as_str()).unwrap_or("ff")
        );

        let r = u8::from_str_radix(&full_hex[0..2], 16).map_err(de::Error::custom)?;
        let g = u8::from_str_radix(&full_hex[2..4], 16).map_err(de::Error::custom)?;
        let b = u8::from_str_radix(&full_hex[4..6], 16).map_err(de::Error::custom)?;
        let a = u8::from_str_radix(&full_hex[6..8], 16).map_err(de::Error::custom)?;

        Ok(HexColor(Color::from_rgba(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            a as f32 / 255.0,
        )))
    }
}
