use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Alert {
    pub id: Option<i32>,
    pub symbol: String,
    pub basis: String,
    pub ma_length: Option<u32>,
    pub value: f64,
    pub direction: String,
    pub status: String,
}
