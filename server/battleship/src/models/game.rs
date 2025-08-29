use serde::{Deserialize, Serialize};
use crate::models::Boards;

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub boards: Boards,
    pub winner: u8
}