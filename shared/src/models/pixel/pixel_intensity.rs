use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PixelIntensity {
    pub zn: f32,
    pub count: f32,
}
