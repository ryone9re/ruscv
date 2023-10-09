use crate::instructions::InstructionSize;

use super::cpu::MemoryBus;

#[derive(Debug)]
pub(super) struct Memory {
    memory: Vec<InstructionSize>,
}

impl Memory {
    fn default_memory_size() -> usize {
        1024
    }

    pub(super) fn new() -> Self {
        Self {
            memory: vec![0; Memory::default_memory_size()],
        }
    }
}

impl MemoryBus for Memory {
    fn load(&self, address: usize) -> InstructionSize {
        // 範囲外アクセスはパニックすべき
        *self.memory.get(address).unwrap()
    }

    fn store(&mut self, address: usize, value: InstructionSize) {
        // 範囲外アクセスはパニックすべき
        *self.memory.get_mut(address).unwrap() = value;
    }
}
