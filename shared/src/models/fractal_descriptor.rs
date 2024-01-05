use super::fractals::{julia_descriptor::JuliaDescriptor, mandelbrot::Mandelbrot};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FractalDescriptor {
    Julia(JuliaDescriptor),
    Mandelbrot(Mandelbrot),
}
