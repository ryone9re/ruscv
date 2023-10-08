pub(self) use num::Unsigned;

#[derive(Debug)]
pub(super) struct Registers<T>
where
    T: Unsigned + Copy,
{
    // Hardwired zero
    x0: T,
    /// Return address
    x1: T,
    // Stack pointer
    x2: T,
    // Global pointer
    x3: T,
    // Thread pointer
    x4: T,
    // Temporary register 0
    x5: T,
    // Temporary register 1
    x6: T,
    // Saved register 0
    x7: T,
    // Saved register 0 / frame pointer
    x8: T,
    // Saved register 1
    x9: T,
    // function argument 0 / return value 0
    x10: T,
    // function argument 1 / return value 1
    x11: T,
    // function argument 2
    x12: T,
    // function argument 3
    x13: T,
    // function argument 4
    x14: T,
    // function argument 5
    x15: T,
    // function argument 6
    x16: T,
    // function argument 7
    x17: T,
    // Saved register 2
    x18: T,
    // Saved register 3
    x19: T,
    // Saved register 4
    x20: T,
    // Saved register 5
    x21: T,
    // Saved register 6
    x22: T,
    // Saved register 7
    x23: T,
    // Saved register 8
    x24: T,
    // Saved register 9
    x25: T,
    // Saved register 10
    x26: T,
    // Saved register 11
    x27: T,
    // Temporary register 3
    x28: T,
    // Temporary register 4
    x29: T,
    // Temporary register 5
    x30: T,
    // Temporary register 6
    x31: T,
    // Program counter
    pc: T,
}

impl<T> Registers<T>
where
    T: Unsigned + Copy,
{
    pub(super) fn new() -> Self {
        Self {
            x0: T::zero(),
            x1: T::zero(),
            x2: T::zero(),
            x3: T::zero(),
            x4: T::zero(),
            x5: T::zero(),
            x6: T::zero(),
            x7: T::zero(),
            x8: T::zero(),
            x9: T::zero(),
            x10: T::zero(),
            x11: T::zero(),
            x12: T::zero(),
            x13: T::zero(),
            x14: T::zero(),
            x15: T::zero(),
            x16: T::zero(),
            x17: T::zero(),
            x18: T::zero(),
            x19: T::zero(),
            x20: T::zero(),
            x21: T::zero(),
            x22: T::zero(),
            x23: T::zero(),
            x24: T::zero(),
            x25: T::zero(),
            x26: T::zero(),
            x27: T::zero(),
            x28: T::zero(),
            x29: T::zero(),
            x30: T::zero(),
            x31: T::zero(),
            pc: T::zero(),
        }
    }

    pub(super) fn get_x0(&self) -> T {
        self.x0
    }

    pub(super) fn get_zero(&self) -> T {
        self.x0
    }

    pub(super) fn get_x1(&self) -> T {
        self.x1
    }

    pub(super) fn set_x1(&mut self, value: T) {
        self.x1 = value
    }

    pub fn get_ra(&self) -> T {
        self.x1
    }

    pub(super) fn set_ra(&mut self, value: T) {
        self.x1 = value
    }

    pub(super) fn get_x2(&self) -> T {
        self.x2
    }

    pub(super) fn set_x2(&mut self, value: T) {
        self.x2 = value
    }

    pub(super) fn get_sp(&self) -> T {
        self.x2
    }

    pub(super) fn set_sp(&mut self, value: T) {
        self.x2 = value
    }

    pub(super) fn get_x3(&self) -> T {
        self.x3
    }

    pub(super) fn set_x3(&mut self, value: T) {
        self.x3 = value
    }

    pub(super) fn get_gp(&self) -> T {
        self.x3
    }

    pub(super) fn set_gp(&mut self, value: T) {
        self.x3 = value
    }

    pub(super) fn get_x4(&self) -> T {
        self.x4
    }

    pub(super) fn set_x4(&mut self, value: T) {
        self.x4 = value
    }

    pub(super) fn get_tp(&self) -> T {
        self.x4
    }

    pub(super) fn set_tp(&mut self, value: T) {
        self.x4 = value
    }

    pub(super) fn get_x5(&self) -> T {
        self.x5
    }

    pub(super) fn set_x5(&mut self, value: T) {
        self.x5 = value
    }

    pub(super) fn get_t0(&self) -> T {
        self.x5
    }

    pub(super) fn set_t0(&mut self, value: T) {
        self.x5 = value
    }

    pub(super) fn get_x6(&self) -> T {
        self.x6
    }

    pub(super) fn set_x6(&mut self, value: T) {
        self.x6 = value
    }

    pub(super) fn get_t1(&self) -> T {
        self.x6
    }

    pub(super) fn set_t1(&mut self, value: T) {
        self.x6 = value
    }

    pub(super) fn get_x7(&self) -> T {
        self.x7
    }

    pub(super) fn set_x7(&mut self, value: T) {
        self.x7 = value
    }

    pub(super) fn get_t2(&self) -> T {
        self.x7
    }

    pub(super) fn set_t2(&mut self, value: T) {
        self.x7 = value
    }

    pub(super) fn get_x8(&self) -> T {
        self.x8
    }

    pub(super) fn set_x8(&mut self, value: T) {
        self.x8 = value
    }

    pub(super) fn get_s0(&self) -> T {
        self.x8
    }

    pub(super) fn set_s0(&mut self, value: T) {
        self.x8 = value
    }

    pub(super) fn get_fp(&self) -> T {
        self.x8
    }

    pub(super) fn set_fp(&mut self, value: T) {
        self.x8 = value
    }

    pub(super) fn get_x9(&self) -> T {
        self.x9
    }

    pub(super) fn set_x9(&mut self, value: T) {
        self.x9 = value
    }

    pub(super) fn get_s1(&self) -> T {
        self.x9
    }

    pub(super) fn set_s1(&mut self, value: T) {
        self.x9 = value
    }

    pub(super) fn get_x10(&self) -> T {
        self.x10
    }

    pub(super) fn set_x10(&mut self, value: T) {
        self.x10 = value
    }

    pub(super) fn get_a0(&self) -> T {
        self.x10
    }

    pub(super) fn set_a0(&mut self, value: T) {
        self.x10 = value
    }

    pub(super) fn get_x11(&self) -> T {
        self.x11
    }

    pub(super) fn set_x11(&mut self, value: T) {
        self.x11 = value
    }

    pub(super) fn get_a1(&self) -> T {
        self.x11
    }

    pub(super) fn set_a1(&mut self, value: T) {
        self.x11 = value
    }

    pub(super) fn get_x12(&self) -> T {
        self.x12
    }

    pub(super) fn set_x12(&mut self, value: T) {
        self.x12 = value
    }

    pub(super) fn get_a2(&self) -> T {
        self.x12
    }

    pub(super) fn set_a2(&mut self, value: T) {
        self.x12 = value
    }

    pub(super) fn get_x13(&self) -> T {
        self.x13
    }

    pub(super) fn set_x13(&mut self, value: T) {
        self.x13 = value
    }

    pub(super) fn get_a3(&self) -> T {
        self.x13
    }

    pub(super) fn set_a3(&mut self, value: T) {
        self.x13 = value
    }

    pub(super) fn get_x14(&self) -> T {
        self.x14
    }

    pub(super) fn set_x14(&mut self, value: T) {
        self.x14 = value
    }

    pub(super) fn get_a4(&self) -> T {
        self.x14
    }

    pub(super) fn set_a4(&mut self, value: T) {
        self.x14 = value
    }

    pub(super) fn get_x15(&self) -> T {
        self.x15
    }

    pub(super) fn set_x15(&mut self, value: T) {
        self.x15 = value
    }

    pub(super) fn get_a5(&self) -> T {
        self.x15
    }

    pub(super) fn set_a5(&mut self, value: T) {
        self.x15 = value
    }

    pub(super) fn get_x16(&self) -> T {
        self.x16
    }

    pub(super) fn set_x16(&mut self, value: T) {
        self.x16 = value
    }

    pub(super) fn get_a6(&self) -> T {
        self.x16
    }

    pub(super) fn set_a6(&mut self, value: T) {
        self.x16 = value
    }

    pub(super) fn get_x17(&self) -> T {
        self.x17
    }

    pub(super) fn set_x17(&mut self, value: T) {
        self.x17 = value
    }

    pub(super) fn get_a7(&self) -> T {
        self.x17
    }

    pub(super) fn set_a7(&mut self, value: T) {
        self.x17 = value
    }

    pub(super) fn get_x18(&self) -> T {
        self.x18
    }

    pub(super) fn set_x18(&mut self, value: T) {
        self.x18 = value
    }

    pub(super) fn get_s2(&self) -> T {
        self.x18
    }

    pub(super) fn set_s2(&mut self, value: T) {
        self.x18 = value
    }

    pub(super) fn get_x19(&self) -> T {
        self.x19
    }

    pub(super) fn set_x19(&mut self, value: T) {
        self.x19 = value
    }

    pub(super) fn get_s3(&self) -> T {
        self.x19
    }

    pub(super) fn set_s3(&mut self, value: T) {
        self.x19 = value
    }

    pub(super) fn get_x20(&self) -> T {
        self.x20
    }

    pub(super) fn set_x20(&mut self, value: T) {
        self.x20 = value
    }

    pub(super) fn get_s4(&self) -> T {
        self.x20
    }

    pub(super) fn set_s4(&mut self, value: T) {
        self.x20 = value
    }

    pub(super) fn get_x21(&self) -> T {
        self.x21
    }

    pub(super) fn set_x21(&mut self, value: T) {
        self.x21 = value
    }

    pub(super) fn get_s5(&self) -> T {
        self.x21
    }

    pub(super) fn set_s5(&mut self, value: T) {
        self.x21 = value
    }

    pub(super) fn get_x22(&self) -> T {
        self.x22
    }

    pub(super) fn set_x22(&mut self, value: T) {
        self.x22 = value
    }

    pub(super) fn get_s6(&self) -> T {
        self.x22
    }

    pub(super) fn set_s6(&mut self, value: T) {
        self.x22 = value
    }

    pub(super) fn get_x23(&self) -> T {
        self.x23
    }

    pub(super) fn set_x23(&mut self, value: T) {
        self.x23 = value
    }

    pub(super) fn get_s7(&self) -> T {
        self.x23
    }

    pub(super) fn set_s7(&mut self, value: T) {
        self.x23 = value
    }

    pub(super) fn get_x24(&self) -> T {
        self.x24
    }

    pub(super) fn set_x24(&mut self, value: T) {
        self.x24 = value
    }

    pub(super) fn get_s8(&self) -> T {
        self.x24
    }

    pub(super) fn set_s8(&mut self, value: T) {
        self.x24 = value
    }

    pub(super) fn get_x25(&self) -> T {
        self.x25
    }

    pub(super) fn set_x25(&mut self, value: T) {
        self.x25 = value
    }

    pub(super) fn get_s9(&self) -> T {
        self.x25
    }

    pub(super) fn set_s9(&mut self, value: T) {
        self.x25 = value
    }

    pub(super) fn get_x26(&self) -> T {
        self.x26
    }

    pub(super) fn set_x26(&mut self, value: T) {
        self.x26 = value
    }

    pub(super) fn get_s10(&self) -> T {
        self.x26
    }

    pub(super) fn set_s10(&mut self, value: T) {
        self.x26 = value
    }

    pub(super) fn get_x27(&self) -> T {
        self.x27
    }

    pub(super) fn set_x27(&mut self, value: T) {
        self.x27 = value
    }

    pub(super) fn get_s11(&self) -> T {
        self.x27
    }

    pub(super) fn set_s11(&mut self, value: T) {
        self.x27 = value
    }

    pub(super) fn get_x28(&self) -> T {
        self.x28
    }

    pub(super) fn set_x28(&mut self, value: T) {
        self.x28 = value
    }

    pub(super) fn get_t3(&self) -> T {
        self.x28
    }

    pub(super) fn set_t3(&mut self, value: T) {
        self.x28 = value
    }

    pub(super) fn get_x29(&self) -> T {
        self.x29
    }

    pub(super) fn set_x29(&mut self, value: T) {
        self.x29 = value
    }

    pub(super) fn get_t4(&self) -> T {
        self.x29
    }

    pub(super) fn set_t4(&mut self, value: T) {
        self.x29 = value
    }

    pub(super) fn get_x30(&self) -> T {
        self.x30
    }

    pub(super) fn set_x30(&mut self, value: T) {
        self.x30 = value
    }

    pub(super) fn get_t5(&self) -> T {
        self.x30
    }

    pub(super) fn set_t5(&mut self, value: T) {
        self.x30 = value
    }

    pub(super) fn get_x31(&self) -> T {
        self.x31
    }

    pub(super) fn set_x31(&mut self, value: T) {
        self.x31 = value
    }

    pub(super) fn get_t6(&self) -> T {
        self.x31
    }

    pub(super) fn set_t6(&mut self, value: T) {
        self.x31 = value
    }

    pub(super) fn get_pc(&self) -> T {
        self.pc
    }

    pub(super) fn set_pc(&mut self, value: T) {
        self.pc = value
    }

    pub(super) fn increment_pc(&mut self) {
        self.pc = self.pc + T::one()
    }
}
