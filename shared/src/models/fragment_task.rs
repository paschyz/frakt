use super::{
    fractal_descriptor::FractalDescriptor, range::Range, resolution::Resolution, u8_data::U8Data,
};

#[derive(Debug, Clone)]
pub struct FragmentTask {
    pub id: U8Data,
    pub fractal: FractalDescriptor,
    pub max_iteration: u32,
    pub resolution: Resolution,
    pub range: Range,
}
