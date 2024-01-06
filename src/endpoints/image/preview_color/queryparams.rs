use serde::{Deserialize, Serialize};

use super::{hex_color::HexColor, preview_size::PreviewSize};

mod defaults {
    #[inline(always)]
    pub fn preview_size() -> u8 {
        1
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
