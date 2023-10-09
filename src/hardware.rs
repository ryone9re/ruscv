use std::sync::{Arc, Mutex};

mod cpu;
mod memory;

#[derive(Debug)]
pub struct Hardware {
    cpu: cpu::CPU,
    memory: Arc<Mutex<memory::Memory>>,
}

impl Hardware {
    pub fn new() -> Self {
        let memory = Arc::new(Mutex::new(memory::Memory::new()));
        Hardware {
            cpu: cpu::CPU::new(memory.clone()),
            memory,
        }
    }
}
