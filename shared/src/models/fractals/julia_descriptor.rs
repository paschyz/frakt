use crate::models::complex::Complex;

#[derive(Debug, Clone)]
pub struct JuliaDescriptor {
    pub c: Complex,
    pub divergence_threshold_square: f64,
}
