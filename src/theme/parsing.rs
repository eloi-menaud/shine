use eframe::egui;

use serde::{self, Deserialize};
use std::sync::OnceLock;

use crate::types::HexColor;



static THEME: OnceLock<Theme> = OnceLock::new();

pub fn get_theme() -> &'static Theme {
    THEME.get().expect("Theme must be initialized")
}


#[derive(Debug, Default)]
pub struct Theme {
    pub(crate) text: Text
}
impl Theme {
    pub fn init_static(self) {
        THEME.set(self).unwrap();
    }
}


#[derive(Debug, Deserialize, Default)]
pub struct Text{
    #[serde(default)]
    pub heading: Heading,
    
    #[serde(default)]
    pub body: Body,
}

#[derive(Debug, Deserialize)]
pub struct Heading {
    /// font color in hex (#RGB, #RRGGBB, #RRGGGBBAA)
    pub color: HexColor,
    
    /// font size
    pub size: f32,
}
impl Default for Heading {
    fn default() -> Self {
        Self { 
            color: HexColor(egui::Color32::WHITE),
            size: 32.0,
        }
    }
}


#[derive(Debug, Deserialize)]
pub struct Body {
    /// font color in hex (#RGB, #RRGGBB, #RRGGGBBAA)
    color: HexColor,
    
    /// font size
    size: f32,
}
impl Default for Body {
    fn default() -> Self {
        Self { 
            color: HexColor(egui::Color32::WHITE),
            size: 16.0
        }
    }
}
