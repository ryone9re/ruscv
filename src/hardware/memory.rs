#[derive(Debug)]
pub(super) struct Memory {
    memory: Vec<u32>,
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
