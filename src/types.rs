use std::str::FromStr;
use serde::{Deserialize, Deserializer};
use eframe::egui::Color32;


#[derive(Debug, Clone, Copy, Default)]
pub struct WindowSize {
    pub width: i32,
    pub height: i32,
}

impl FromStr for WindowSize {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (w_str, h_str) = s.split_once('x')
            .ok_or_else(|| format!("Invalid format: '{}'. Waiting 'WxH'", s))?;
        
        let width = w_str.parse().map_err(|_| format!("Invalid Width: {}. Waiting a int", w_str))?;
        let height = h_str.parse().map_err(|_| format!("Invalid Height: {}. Waiting a int", h_str))?;
        
        Ok(WindowSize { width, height })
    }
}



#[derive(Debug, Clone)]
pub struct HexColor(pub Color32);

impl<'de> Deserialize<'de> for HexColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hex = String::deserialize(deserializer)?;
        
        // Parsing simple : "#RRGGBBAA"
        let hex = hex.trim_start_matches('#');
        
        let full_hex = match hex.len() {
            3 => &format!("{0}{0}{1}{1}{2}{2}",&hex[0..1],&hex[1..2],&hex[2..3]),
            6 => &format!("{}AA",&hex),
            8 => hex,
            e => return Err(serde::de::Error::custom("Format hex invalide, attendu #RRGGBB"))
        };

        let r = u8::from_str_radix(&full_hex[0..2], 16).map_err(serde::de::Error::custom)?;
        let g = u8::from_str_radix(&full_hex[2..4], 16).map_err(serde::de::Error::custom)?;
        let b = u8::from_str_radix(&full_hex[4..6], 16).map_err(serde::de::Error::custom)?;
        let a = u8::from_str_radix(&full_hex[6..8], 16).map_err(serde::de::Error::custom)?;

        Ok(HexColor(Color32::from_rgba_unmultiplied(r, g, b, a)))
    }
}