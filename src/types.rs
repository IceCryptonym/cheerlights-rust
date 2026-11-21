use std::str::FromStr;

use serde::{Deserialize, de::value::StrDeserializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    Red,
    Green,
    Blue,
    Cyan,
    White,
    Oldlace,
    Purple,
    Magenta,
    Yellow,
    Orange,
    Pink,
}

#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn to_rgb(&self) -> Rgb {
        match self {
            Color::Red => Rgb::new(255, 0, 0),
            Color::Green => Rgb::new(0, 128, 0),
            Color::Blue => Rgb::new(0, 0, 255),
            Color::Cyan => Rgb::new(0, 255, 255),
            Color::White => Rgb::new(255, 255, 255),
            Color::Oldlace => Rgb::new(253, 245, 230),
            Color::Purple => Rgb::new(128, 0, 128),
            Color::Magenta => Rgb::new(255, 0, 255),
            Color::Yellow => Rgb::new(255, 255, 0),
            Color::Orange => Rgb::new(255, 165, 0),
            Color::Pink => Rgb::new(255, 192, 203),
        }
    }
}

impl FromStr for Color {
    type Err = serde::de::value::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::deserialize(StrDeserializer::new(s))
    }
}

impl Rgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}
