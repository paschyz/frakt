use super::{
    fractal_descriptor::FractalDescriptor, range::Range, resolution::Resolution, u8_data::U8Data,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FragmentTask {
    pub id: U8Data,
    pub fractal: FractalDescriptor,
    pub max_iteration: u32,
    pub resolution: Resolution,
    pub range: Range,
}
