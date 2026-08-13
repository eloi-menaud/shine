use iced::alignment::{Horizontal, Vertical};

use crate::parser::AttrFromStr;






impl AttrFromStr for Vertical{
    fn attr_from_str(s: &str) -> Result<Self, &str> {
        match s {
            "top" => Ok(Vertical::Top),
            "center" => Ok(Vertical::Center),
            "bottom" => Ok(Vertical::Bottom),
            invalid => Err(invalid)
        }
    }
}

impl AttrFromStr for Horizontal{
    fn attr_from_str(s: &str) -> Result<Self, &str> {
        match s {
            "left" => Ok(Horizontal::Left),
            "center" => Ok(Horizontal::Center),
            "right" => Ok(Horizontal::Right),
            invalid => Err(invalid)
        }
    }
}


