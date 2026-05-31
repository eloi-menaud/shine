use eframe::egui::{self, Color32};

use serde::{self, Deserialize};
use std::sync::OnceLock;

use crate::{theme::parsing::get_theme, types::HexColor};


pub trait Heading {
    fn themed_heading(&mut self, text: impl Into<String>);
}
impl Heading for egui::Ui {
    fn themed_heading(&mut self, text: impl Into<String>) {
        
        let theme = get_theme();
        
        self.heading(
            egui::RichText::new(text)
                .color(theme.text.heading.color.0)
                .size(theme.text.heading.size)
        );
        self.add_space(theme.text.heading.size / 2.0);
    }
}