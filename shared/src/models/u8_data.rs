use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct U8Data {
    pub offset: u32,
    pub count: u32,
}
