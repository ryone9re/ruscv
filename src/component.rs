mod register;

struct CPU {
    register: register::Registers,
    pc: u32,
}

impl CPU {
    fn new() -> Self {
        CPU {
            register: register::Registers::new(),
            pc: 0,
        }
    }
}
