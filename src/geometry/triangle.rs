use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Triangle {
    pub v1: usize,
    pub v2: usize,
    pub v3: usize,
}
