use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Boards {
    pub size: u8,
    pub player1: Vec<u8>,
    pub player2: Vec<u8>,
}