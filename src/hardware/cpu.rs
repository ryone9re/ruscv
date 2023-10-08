mod instructions;
mod registers;

#[derive(Debug)]
pub(super) struct CPU {
    registers: registers::Registers<u64>,
}
