use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}
