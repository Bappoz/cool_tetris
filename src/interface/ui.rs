// UI module - currently not used
// This module was intended for WASM/web interface
// The game currently uses SDL2 backend (see main.rs)

#[allow(dead_code)]
pub struct App {
    width: u32,
    height: u32,
}

impl App {
    #[allow(dead_code)]
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
