use serde::{Deserialize, Serialize};

use crate::models::{
    pixel::pixel_data::PixelData, range::Range, resolution::Resolution, u8_data::U8Data,
};

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct FragmentResult {
    pub id: U8Data,
    pub resolution: Resolution,
    pub range: Range,
    pub pixels: PixelData,
}
