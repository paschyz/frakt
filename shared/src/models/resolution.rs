use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Resolution {
    pub nx: u16,
    pub ny: u16,
}
