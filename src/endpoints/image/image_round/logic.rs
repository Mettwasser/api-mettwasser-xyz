use std::cmp::min;

use image::{ImageBuffer, Rgba};
use serde::{Deserialize, Serialize};

use crate::error::ApiError;

mod defaults {
    #[inline(always)]
    pub fn radius() -> u32 {
        3
    }

    #[inline(always)]
    pub fn auto() -> bool {
        false
    }
}

pub fn round(
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    params: RoundImageQueryParams,
) -> Result<(), ApiError> {
    let (width, height) = img.dimensions();

    let (tl, tr, bl, br);

    if params.auto {
        let smaller_dimension = min(width, height);
        (tl, tr, bl, br) = [smaller_dimension / 2; 4].into();
    } else {
        (tl, tr, bl, br) = params.list_corners();
        if tl + tr > width || tr + bl > width || tl + br > height || tr + bl > height {
            return Err(ApiError::new("Radius out of range!", 400));
        }
    };

    // top left
    border_radius(img, tl, |x, y| (x - 1, y - 1));
    // top right
    border_radius(img, tr, |x, y| (width - x, y - 1));
    // bottom right
    border_radius(img, bl, |x, y| (width - x, height - y));
    // bottom left
    border_radius(img, br, |x, y| (x - 1, height - y));
    Ok(())
}

fn border_radius(
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    r: u32,
    coordinates: impl Fn(u32, u32) -> (u32, u32),
) {
    if r == 0 {
        return;
    }
    let r0 = r;

    // 16x antialiasing: 16x16 grid creates 256 possible shades, great for u8!
    let r = 16 * r;

    let mut x = 0;
    let mut y = r - 1;
    let mut p: i32 = 2 - r as i32;

    // ...

    let mut alpha: u16 = 0;
    let mut skip_draw = true;

    let draw = |img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, alpha, x, y| {
        debug_assert!((1..=256).contains(&alpha));
        let pixel_alpha = &mut img[coordinates(r0 - x, r0 - y)].0[3];
        *pixel_alpha = ((alpha * *pixel_alpha as u16 + 128) / 256) as u8;
    };

    'l: loop {
        // (comments for bottom_right case:)
        // remove contents below current position
        {
            let i = x / 16;
            for j in y / 16 + 1..r0 {
                img[coordinates(r0 - i, r0 - j)].0[3] = 0;
            }
        }
        // remove contents right of current position mirrored
        {
            let j = x / 16;
            for i in y / 16 + 1..r0 {
                img[coordinates(r0 - i, r0 - j)].0[3] = 0;
            }
        }

        // draw when moving to next pixel in x-direction
        if !skip_draw {
            draw(img, alpha, x / 16 - 1, y / 16);
            draw(img, alpha, y / 16, x / 16 - 1);
            alpha = 0;
        }

        for _ in 0..16 {
            skip_draw = false;

            if x >= y {
                break 'l;
            }

            alpha += y as u16 % 16 + 1;
            if p < 0 {
                x += 1;
                p += (2 * x + 2) as i32;
            } else {
                // draw when moving to next pixel in y-direction
                if y % 16 == 0 {
                    draw(img, alpha, x / 16, y / 16);
                    draw(img, alpha, y / 16, x / 16);
                    skip_draw = true;
                    alpha = (x + 1) as u16 % 16 * 16;
                }

                x += 1;
                p -= (2 * (y - x) + 2) as i32;
                y -= 1;
            }
        }
    }

    // one corner pixel left
    if x / 16 == y / 16 {
        // column under current position possibly not yet accounted
        if x == y {
            alpha += y as u16 % 16 + 1;
        }
        let s = y as u16 % 16 + 1;
        let alpha = 2 * alpha - s * s;
        draw(img, alpha, x / 16, y / 16);
    }

    // remove remaining square of content in the corner
    let range = y / 16 + 1..r0;
    for i in range.clone() {
        for j in range.clone() {
            img[coordinates(r0 - i, r0 - j)].0[3] = 0;
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoundImageQueryParams {
    pub url: String,

    #[serde(default = "defaults::auto")]
    pub auto: bool,

    #[serde(default = "defaults::radius")]
    corner_radius: u32,

    top_left: Option<u32>,

    top_right: Option<u32>,

    bottom_left: Option<u32>,

    bottom_right: Option<u32>,
}

impl RoundImageQueryParams {
    #[inline(always)]
    pub fn top_left(&self) -> u32 {
        self.top_left.unwrap_or(self.corner_radius)
    }

    #[inline(always)]
    pub fn top_right(&self) -> u32 {
        self.top_right.unwrap_or(self.corner_radius)
    }

    #[inline(always)]
    pub fn bottom_left(&self) -> u32 {
        self.bottom_left.unwrap_or(self.corner_radius)
    }

    #[inline(always)]
    pub fn bottom_right(&self) -> u32 {
        self.bottom_right.unwrap_or(self.corner_radius)
    }

    pub fn list_corners(&self) -> (u32, u32, u32, u32) {
        (
            self.top_left(),
            self.top_right(),
            self.bottom_left(),
            self.bottom_right(),
        )
    }
}
