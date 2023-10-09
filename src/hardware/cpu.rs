mod registers;

use core::fmt::Debug;
use std::sync::{Arc, Mutex};

use crate::instructions::InstructionSize;

pub(super) struct CPU {
    registers: registers::Registers<u64>,
    memory_bus: Arc<Mutex<dyn MemoryBus>>,
}

impl CPU {
    pub(super) fn new(bus: Arc<Mutex<dyn MemoryBus>>) -> Self {
        CPU {
            registers: registers::Registers::new(),
            memory_bus: bus,
        }
    }
}

pub trait MemoryBus {
    fn load(&self, address: usize) -> InstructionSize;
    fn store(&mut self, address: usize, value: InstructionSize);
}

impl Debug for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CPU")
            .field("registers", &self.registers)
            .finish()
    }
}
