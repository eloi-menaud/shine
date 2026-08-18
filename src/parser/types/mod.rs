use iced::{Color, Length, Padding, Radians, alignment::{self, Horizontal, Vertical}, border::Radius, widget::rule::FillMode::Padded};
use regex::Regex;

pub trait AttrFromStr: Sized {
    fn attr_from_str(s: &str,) -> Result<Self, String>;
}


// -------- NEW --------

pub type Pixel = f32;
impl AttrFromStr for Pixel{
    fn attr_from_str(s: &str,) -> Result<Self, String> {
        match s.parse::<f32>() {
            Ok(u) => Ok(u),
            Err(e) => Err(format!("Can't parse attribute value '{s}' into pixel : {e}")),
        }
    }
}



// -------- ICED --------


impl AttrFromStr for Length {
    fn attr_from_str(s: &str,) -> Result<Self, String> {
        match s {
            "fill" => Ok(Length::Fill),
            "shrink" => Ok(Length::Shrink),
            pixel => {
                Ok(Length::Fixed(Pixel::attr_from_str(pixel)?))
            }
        }
    }
}


impl AttrFromStr for Vertical{
    fn attr_from_str(s: &str) -> Result<Self, String> {
        match s {
            "top" => Ok(Vertical::Top),
            "center" => Ok(Vertical::Center),
            "bottom" => Ok(Vertical::Bottom),
            invalid => Err(invalid.to_string())
        }
    }
}


impl AttrFromStr for Horizontal{
    fn attr_from_str(s: &str) -> Result<Self, String> {
        match s {
            "left" => Ok(Horizontal::Left),
            "center" => Ok(Horizontal::Center),
            "right" => Ok(Horizontal::Right),
            invalid => Err(invalid.to_string())
        }
    }
}


struct Quadrant {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32
}
impl AttrFromStr for Quadrant {
    fn attr_from_str(s: &str,) -> Result<Self, String> {
        let re_details = r"^ *(\d+) +(\d+) +(\d+) +(\d+) *$";
        let re_axis = r"^ *(\d+) +(\d+) *$";
        let re_global = r"^ *(\d+) *$";
        if let Some(caps) = Regex::new(re_details).unwrap().captures(s) {
            Ok(Self{
                top: caps[1].parse().map_err(|e| format!("Invalid 'top right bottom left' : Invalid top value '{}' : {e}", &caps[1]))?,
                right: caps[2].parse().map_err(|e| format!("Invalid 'top right bottom left' : Invalid right value '{}' : {e}", &caps[2]))?,
                bottom: caps[3].parse().map_err(|e| format!("Invalid 'top right bottom left' : Invalid bottom value '{}' : {e}", &caps[3]))?,
                left: caps[4].parse().map_err(|e| format!("Invalid 'top right bottom left' : Invalid padding value '{}' : {e}", &caps[4]))?,
            })
        }
        else if let Some(caps) = Regex::new(re_axis).unwrap().captures(s) {
            let vertical : f32 = caps[1].parse().map_err(|e| format!("Invalid 'vertical horizontal' : Invalid vertical (top & bottom) value '{}' : {e}", &caps[1]))?;
            let horizontal : f32 = caps[2].parse().map_err(|e| format!("Invalid 'vertical horizontal' : Invalid horizontal (left & right) value '{}' : {e}", &caps[2]))?;
            Ok(Self{
                top: vertical,
                right: horizontal,
                bottom: vertical,
                left: horizontal,
            })
        }
        else if let Some(caps) = Regex::new(re_global).unwrap().captures(s) {
            let global : f32 = caps[1].parse().map_err(|e| format!("Invalid 'global' (top, bottom, left, right) value '{}' : {e}", &caps[1]))?;
            Ok(Self{
                top: global,
                right: global,
                bottom: global,
                left: global,
            })
        }
        else {
            return Err(format!("Invalid format, get '{s}' waiting 'top right bottom left' ({re_details}) , 'vertical horizontal' ({re_axis}) or 'global' ({re_global})"));
        }
    }
}

impl AttrFromStr for Padding{
    fn attr_from_str(s: &str,) -> Result<Self, String> {
        let q = Quadrant::attr_from_str(s).map_err(|e| format!("Invalid padding : {e}"))?;
        Ok(Self{
            top: q.top,
            right: q.right,
            bottom: q.bottom,
            left: q.left
        })
    }
}
impl AttrFromStr for Radius {
    fn attr_from_str(s: &str,) -> Result<Self, String> {
        let q = Quadrant::attr_from_str(s).map_err(|e| format!("Invalid radius : {e}"))?;
        Ok(Self{
            top_left: q.top,
            top_right: q.right,
            bottom_right: q.bottom,
            bottom_left: q.left
        })
    }
}



impl AttrFromStr for Color {
    fn attr_from_str(s: &str,) -> Result<Self, String> {
        let hex = match s.strip_prefix('#'){
            Some(hex) => hex,
            None => return Err(format!("Ivalid color format: get '{s}', waiting #rrggbb or #rrggbbaa"))
        };

        
        if hex.len() != 6 && hex.len() != 8 {
            return Err(format!("Ivalid color format: get '{s}', waiting #rrggbb or #rrggbbaa"));
        }
    
        let num = u32::from_str_radix(hex, 16).map_err(|e| format!("Ivalid color format for value '{s}' : {e}"))?;
    
        let (r,g,b,a) : (f32,f32,f32,f32) = if hex.len() == 6 {
            let r = ((num >> 16) & 0xFF) as f32 / 255.0;
            let g = ((num >> 8) & 0xFF) as f32 / 255.0;
            let b = (num & 0xFF) as f32 / 255.0;
            (r, g, b, 1.0)
        } else {
            let r = ((num >> 24) & 0xFF) as f32 / 255.0;
            let g = ((num >> 16) & 0xFF) as f32 / 255.0;
            let b = ((num >> 8) & 0xFF) as f32 / 255.0;
            let a = (num & 0xFF) as f32 / 255.0;
            (r, g, b, a)
        };
            
        Ok(Color::from_rgba(r, g, b, a))
    }
}

