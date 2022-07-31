use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex}; // 1.3.1
pub struct Program {}

impl Program {
    pub fn new() -> Program {
        Program {}
    }
    pub fn initialize(&self) {
        println!("Program Initialized");
    }
}

// program singleton
pub static G_PROGRAM: Lazy<Arc<Mutex<Program>>> =
    Lazy::new(|| Arc::new(Mutex::new(Program::new())));
