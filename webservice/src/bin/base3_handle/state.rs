use std::sync::Mutex;

pub struct AppState {
    pub health: String,
    pub count: Mutex<u32>,
}
