use serde::{Deserialize, Serialize};

use super::{julia_descriptor::JuliaDescriptor, mandelbrot::Mandelbrot};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FractalDescriptor {
    Julia(JuliaDescriptor),
    Mandelbrot(Mandelbrot),
}
