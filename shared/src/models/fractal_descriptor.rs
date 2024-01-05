use super::fractals::{julia_descriptor::JuliaDescriptor, mandelbrot::Mandelbrot};

#[derive(Debug, Clone)]
pub enum FractalDescriptor {
    Julia(JuliaDescriptor),
    Mandelbrot(Mandelbrot),
}
