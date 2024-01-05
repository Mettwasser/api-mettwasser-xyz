use image::{ImageBuffer, Rgb};
use serde::{Deserialize, Serialize};

mod defaults {
    #[inline(always)]
    pub fn preview_size() -> u8 {
        1
    }
}

pub struct HexColor {
    red: u8,
    green: u8,
    blue: u8,
}

impl From<u32> for HexColor {
    fn from(value: u32) -> Self {
        let red = ((value & 0xff0000) >> 16) as u8;
        let green = ((value & 0x00ff00) >> 8) as u8;
        let blue = (value & 0x0000ff) as u8;

        Self { red, green, blue }
    }
}

impl From<String> for HexColor {
    fn from(mut hex: String) -> Self {
        hex = hex.replace('#', "");

        let number = u32::from_str_radix(&hex, 16).unwrap_or(0);
        HexColor::from(number)
    }
}

pub enum PreviewSize {
    Small,
    Medium,
    Large,
}

impl From<u8> for PreviewSize {
    fn from(value: u8) -> Self {
        use PreviewSize::*;

        match value {
            1 => Small,
            2 => Medium,
            3 => Large,
            _ => Small,
        }
    }
}

impl From<PreviewSize> for (u32, u32) {
    fn from(val: PreviewSize) -> Self {
        use PreviewSize::*;

        match val {
            Small => (128, 128),
            Medium => (256, 256),
            Large => (512, 512),
        }
    }
}

impl HexColor {
    pub fn into_preview(self, prev_size: PreviewSize) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let (width, height) = prev_size.into();
        let mut img = ImageBuffer::new(width, height);

        // Fill the image with the specified color
        for (_, _, pixel) in img.enumerate_pixels_mut() {
            *pixel = Rgb([self.red, self.green, self.blue]);
        }

        img
    }
}

#[derive(Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct PreviewColorQueryParams {
    hex: String,

    #[serde(default = "defaults::preview_size")]
    size: u8,
}

impl From<PreviewColorQueryParams> for (HexColor, PreviewSize) {
    fn from(val: PreviewColorQueryParams) -> Self {
        (val.hex.into(), val.size.into())
    }
}
