use serde::{Deserialize, Serialize};

use crate::models::complex::Complex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JuliaDescriptor {
    pub c: Complex,
    pub divergence_threshold_square: f64,
}
