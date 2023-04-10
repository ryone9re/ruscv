use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

pub struct Registers {
    reg: [u32; 32],
}

impl Registers {
    pub fn new() -> Registers {
        Registers { reg: [0; 32] }
    }

    pub fn get_zero(&self) -> u32 {
        if self.reg[0] != 0 {
            panic!("x0 is not zero");
        }
        self.reg[0]
    }

    pub fn set_ra(&mut self, value: u32) {
        self.reg[1] = value;
    }

    pub fn get_ra(&self) -> u32 {
        self.reg[1]
    }

    pub fn set_sp(&mut self, value: u32) {
        self.reg[2] = value;
    }

    pub fn get_sp(&self) -> u32 {
        self.reg[2]
    }

    pub fn set_gp(&mut self, value: u32) {
        self.reg[3] = value;
    }

    pub fn get_gp(&self) -> u32 {
        self.reg[3]
    }

    pub fn set_tp(&mut self, value: u32) {
        self.reg[4] = value;
    }

    pub fn get_tp(&self) -> u32 {
        self.reg[4]
    }

    pub fn set_t0(&mut self, value: u32) {
        self.reg[5] = value;
    }

    pub fn get_t0(&self) -> u32 {
        self.reg[5]
    }

    pub fn set_t1(&mut self, value: u32) {
        self.reg[6] = value;
    }

    pub fn get_t1(&self) -> u32 {
        self.reg[6]
    }

    pub fn set_t2(&mut self, value: u32) {
        self.reg[7] = value;
    }

    pub fn get_t2(&self) -> u32 {
        self.reg[7]
    }

    pub fn set_s0(&mut self, value: u32) {
        self.reg[8] = value;
    }

    pub fn get_s0(&self) -> u32 {
        self.reg[8]
    }

    pub fn set_s1(&mut self, value: u32) {
        self.reg[9] = value;
    }

    pub fn get_s1(&self) -> u32 {
        self.reg[9]
    }

    pub fn set_a0(&mut self, value: u32) {
        self.reg[10] = value;
    }

    pub fn get_a0(&self) -> u32 {
        self.reg[10]
    }

    pub fn set_a1(&mut self, value: u32) {
        self.reg[11] = value;
    }

    pub fn get_a1(&self) -> u32 {
        self.reg[11]
    }

    pub fn set_a2(&mut self, value: u32) {
        self.reg[12] = value;
    }

    pub fn get_a2(&self) -> u32 {
        self.reg[12]
    }

    pub fn set_a3(&mut self, value: u32) {
        self.reg[13] = value;
    }

    pub fn get_a3(&self) -> u32 {
        self.reg[13]
    }

    pub fn set_a4(&mut self, value: u32) {
        self.reg[14] = value;
    }

    pub fn get_a4(&self) -> u32 {
        self.reg[14]
    }

    pub fn set_a5(&mut self, value: u32) {
        self.reg[15] = value;
    }

    pub fn get_a5(&self) -> u32 {
        self.reg[15]
    }

    pub fn set_a6(&mut self, value: u32) {
        self.reg[16] = value;
    }

    pub fn get_a6(&self) -> u32 {
        self.reg[16]
    }

    pub fn set_a7(&mut self, value: u32) {
        self.reg[17] = value;
    }

    pub fn get_a7(&self) -> u32 {
        self.reg[17]
    }

    pub fn set_s2(&mut self, value: u32) {
        self.reg[18] = value;
    }

    pub fn get_s2(&self) -> u32 {
        self.reg[18]
    }

    pub fn set_s3(&mut self, value: u32) {
        self.reg[19] = value;
    }

    pub fn get_s3(&self) -> u32 {
        self.reg[19]
    }

    pub fn set_s4(&mut self, value: u32) {
        self.reg[20] = value;
    }

    pub fn get_s4(&self) -> u32 {
        self.reg[20]
    }

    pub fn set_s5(&mut self, value: u32) {
        self.reg[21] = value;
    }

    pub fn get_s5(&self) -> u32 {
        self.reg[21]
    }

    pub fn set_s6(&mut self, value: u32) {
        self.reg[22] = value;
    }

    pub fn get_s6(&self) -> u32 {
        self.reg[22]
    }

    pub fn set_s7(&mut self, value: u32) {
        self.reg[23] = value;
    }

    pub fn get_s7(&self) -> u32 {
        self.reg[23]
    }

    pub fn set_s8(&mut self, value: u32) {
        self.reg[24] = value;
    }

    pub fn get_s8(&self) -> u32 {
        self.reg[24]
    }

    pub fn set_s9(&mut self, value: u32) {
        self.reg[25] = value;
    }

    pub fn get_s9(&self) -> u32 {
        self.reg[25]
    }

    pub fn set_s10(&mut self, value: u32) {
        self.reg[26] = value;
    }

    pub fn get_s10(&self) -> u32 {
        self.reg[26]
    }

    pub fn set_s11(&mut self, value: u32) {
        self.reg[27] = value;
    }

    pub fn get_s11(&self) -> u32 {
        self.reg[27]
    }

    pub fn set_t3(&mut self, value: u32) {
        self.reg[28] = value;
    }

    pub fn get_t3(&self) -> u32 {
        self.reg[28]
    }

    pub fn set_t4(&mut self, value: u32) {
        self.reg[29] = value;
    }

    pub fn get_t4(&self) -> u32 {
        self.reg[29]
    }

    pub fn set_t5(&mut self, value: u32) {
        self.reg[30] = value;
    }

    pub fn get_t5(&self) -> u32 {
        self.reg[30]
    }

    pub fn set_t6(&mut self, value: u32) {
        self.reg[31] = value;
    }

    pub fn get_t6(&self) -> u32 {
        self.reg[31]
    }
}

impl Index<usize> for Registers {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.reg[index]
    }
}

impl IndexMut<usize> for Registers {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.reg[index]
    }
}

impl Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let abis = [
            "zero", " ra ", " sp ", " gp ", " tp ", " t0 ", " t1 ", " t2 ", " s0 ", " s1 ", " a0 ",
            " a1 ", " a2 ", " a3 ", " a4 ", " a5 ", " a6 ", " a7 ", " s2 ", " s3 ", " s4 ", " s5 ",
            " s6 ", " s7 ", " s8 ", " s9 ", " s10", " s11", " t3 ", " t4 ", " t5 ", " t6 ",
        ];

        let mut disp = String::from("");

        for (i, abi) in abis.iter().enumerate() {
            disp = format!(
                "{}\n{}",
                disp,
                format_args!("{}(x{}): {:32}", *abi, i, self[i])
            );
        }

        write!(f, "{}", disp)
    }
}
