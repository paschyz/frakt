use super::point::Point;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Range {
    pub min: Point,
    pub max: Point,
}
