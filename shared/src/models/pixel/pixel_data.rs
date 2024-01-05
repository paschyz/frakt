use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PixelData {
    pub offset: u32,
    pub count: u32,
}
