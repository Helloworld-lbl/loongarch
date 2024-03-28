use crate::consts::TCFG_INITVAL_LEN;
use bit_field::BitField;

/// TCFG register
#[derive(Clone, Copy)]
pub struct Tcfg {
    bits: usize,
}

impl Tcfg {
    // 0
    // En
    // RW
    // 定时器使能位。仅当该位为 1 时，定时器才会进行倒计时自减，并在减为 0 值时置起
    // 定时中断信号。
    #[inline]
    pub fn en(&self) -> bool {
        self.bits.get_bit(0)
    }

    // 1
    // Periodic
    // RW
    // 定时器循环模式控制位。若该位为 1，定时器在倒计时自减至 0 时，在置起定时中断
    // 信号的同时，还会自动定时器重新装载成 InitVal 域中配置的初始值，然后再下一个
    // 时钟周期继续自减。若该位为 0，定时器在倒计时自减至 0 时，将停止计数直至软件
    // 再次配置该定时器。
    #[inline]
    pub fn periodic(&self) -> bool {
        self.bits.get_bit(1)
    }

    // n-1:2
    // InitVal
    // RW
    // 定时器倒计时自减计数的初始值。要求该初始值必须是 4 的整倍数。硬件将自动在该
    // 域数值的最低位补上两比特 0 后再使用。
    #[inline]
    pub fn initval(&self) -> usize {
        self.bits.get_bits(2..2 + TCFG_INITVAL_LEN)
    }

    #[inline]
    pub fn set_en(&mut self) {
        self.bits.set_bit(0, true);
    }

    #[inline]
    pub fn set_periodic(&mut self) {
        self.bits.set_bit(1, true);
    }

    #[inline]
    pub fn clear_periodic(&mut self) {
        self.bits.set_bit(1, false);
    }

    #[inline]
    pub fn set_initval(&mut self, val: usize) {
        self.bits.set_bits(2..2 + TCFG_INITVAL_LEN, val);
    }

    #[inline]
    pub unsafe fn write(&self) {
        _write(self.bits);
    }

    pub unsafe fn init_trigger(&mut self, initval: usize) {
        self.set_periodic();
        self.set_initval(initval >> 2);
        self.set_en();
        self.write();
    }
}

read_csr_as!(Tcfg, 0x41, __read_tcfg);
write_csr!(0x41, __write_tcfg);