use std::sync::{Arc, Mutex};
use crate::models::Boards;

#[derive(Clone)]
pub struct AppState {
    pub boards: Arc<Mutex<Boards>>,
    pub winner: Arc<Mutex<u8>>
}