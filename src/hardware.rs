mod cpu;
mod memory;

#[derive(Debug)]
pub struct Hardware {
    cpu: cpu::CPU,
    memory: memory::Memory,
}
