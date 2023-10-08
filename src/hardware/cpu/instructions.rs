macro_rules! opcode {
    ($insn:expr) => {
        $insn & 0b1111111
    };
}

macro_rules! rd {
    ($insn:expr) => {
        ($insn >> 7) & 0b11111
    };
}

macro_rules! rs1 {
    ($insn:expr) => {
        ($insn >> 15) & 0b11111
    };
}

macro_rules! funct3 {
    ($insn:expr) => {
        ($insn >> 12) & 0b111
    };
}

macro_rules! rs2 {
    ($insn:expr) => {
        ($insn >> 20) & 0b11111
    };
}

macro_rules! funct7 {
    ($insn:expr) => {
        ($insn >> 25) & 0b1111111
    };
}

macro_rules! imm_i {
    ($insn:expr) => {
        (($insn as u32) >> 20) as u32
    };
}

macro_rules! imm_s {
    ($insn:expr) => {
        ((($insn as u32) >> 20) & !0b11111) | (($insn >> 7) & 0b11111)
    };
}

macro_rules! imm_b {
    ($insn:expr) => {
        ((($insn as u32) << 19) & 0b100000000000) | // bit 11
        (($insn << 4) & 0b1111110000) |             // bits 4-9
        (($insn >> 20) & 0b11110) |                 // bits 1-4
        ((($insn as u32) >> 31) & 0b1)              // bit 0
    };
}

macro_rules! imm_u {
    ($insn:expr) => {
        $insn & !0b1111111
    };
}

// J-type
macro_rules! imm_j {
    ($insn:expr) => {
        ((($insn as u32) << 11) & 0b100000000000) | // bit 11
        (($insn >> 9) & 0b11111111110) |            // bits 1-10
        (($insn >> 20) & 0b11111111110000000000)    // bits 12-19
    };
}

type Opcode = u8;

pub struct RType {
    opcode: Opcode,
    rd: u8,
    funct3: u8,
    rs1: u8,
    rs2: u8,
    funct7: u8,
}

impl From<u32> for RType {
    fn from(value: u32) -> Self {
        RType {
            opcode: opcode!(value) as u8,
            rd: rd!(value) as u8,
            funct3: funct3!(value) as u8,
            rs1: rs1!(value) as u8,
            rs2: rs2!(value) as u8,
            funct7: funct7!(value) as u8,
        }
    }
}

pub struct IType {
    opcode: Opcode,
    rd: u8,
    funct3: u8,
    rs1: u8,
    imm: u16,
}

impl From<u32> for IType {
    fn from(value: u32) -> Self {
        IType {
            opcode: opcode!(value) as u8,
            rd: rd!(value) as u8,
            funct3: funct3!(value) as u8,
            rs1: rs1!(value) as u8,
            imm: imm_i!(value) as u16,
        }
    }
}

pub struct SType {
    opcode: Opcode,
    imm: u16,
    funct3: u8,
    rs1: u8,
    rs2: u8,
}

impl From<u32> for SType {
    fn from(value: u32) -> Self {
        SType {
            opcode: opcode!(value) as u8,
            imm: imm_s!(value) as u16,
            funct3: funct3!(value) as u8,
            rs1: rs1!(value) as u8,
            rs2: rs2!(value) as u8,
        }
    }
}

pub struct BType {
    opcode: Opcode,
    imm: u16,
    funct3: u8,
    rs1: u8,
    rs2: u8,
}

impl From<u32> for BType {
    fn from(value: u32) -> Self {
        BType {
            opcode: opcode!(value) as u8,
            imm: imm_b!(value) as u16,
            funct3: funct3!(value) as u8,
            rs1: rs1!(value) as u8,
            rs2: rs2!(value) as u8,
        }
    }
}

pub struct UType {
    opcode: Opcode,
    rd: u8,
    imm: u32,
}

impl From<u32> for UType {
    fn from(value: u32) -> Self {
        UType {
            opcode: opcode!(value) as u8,
            rd: rd!(value) as u8,
            imm: imm_u!(value),
        }
    }
}

pub struct JType {
    opcode: Opcode,
    rd: u8,
    imm: u32,
}

impl From<u32> for JType {
    fn from(value: u32) -> Self {
        JType {
            opcode: opcode!(value) as u8,
            rd: rd!(value) as u8,
            imm: imm_j!(value),
        }
    }
}

pub enum Instruction {
    R(RType),
    I(IType),
    S(SType),
    B(BType),
    U(UType),
    J(JType),
}

impl From<u32> for Instruction {
    fn from(value: u32) -> Self {
        value.into()
    }
}
