use bit_field::BitField;
/// Dmw0 Register
/// 这一组寄存器参与完成直接映射地址翻译模式。
#[derive(Clone, Copy, Debug)]
pub struct Dmw0 {
    bits: usize,
}

impl Dmw0 {
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    #[inline]
    pub fn plv0(&self) -> bool {
        self.bits.get_bit(0)
    }

    #[inline]
    pub fn plv1(&self) -> bool {
        self.bits.get_bit(1)
    }

    #[inline]
    pub fn plv2(&self) -> bool {
        self.bits.get_bit(2)
    }

    #[inline]
    pub fn plv3(&self) -> bool {
        self.bits.get_bit(3)
    }
    
    #[inline]
    pub fn mat(&self) -> usize {
        self.bits.get_bits(4..6)
    }

    #[inline]
    pub fn vseg(&self) -> usize {
        self.bits.get_bits(60..)
    }

    #[inline]
    pub fn set_plv0(&self, value: bool) {
        self.bits.set_bit(0, value)
    }

    #[inline]
    pub fn set_plv1(&self, value: bool) {
        self.bits.set_bit(1, value)
    }

    #[inline]
    pub fn set_plv2(&self, value: bool) {
        self.bits.set_bit(2, value)
    }

    #[inline]
    pub fn set_plv3(&self, value: bool) {
        self.bits.set_bit(3, value)
    }

    #[inline]
    pub fn set_mat(&self, value: usize) {
        self.bits.set_bits(4..6, value)
    }

    #[inline]
    pub fn set_vseg(&self, value: usize) {
        self.bits.set_bits(60.., value)
    }

    #[inline]
    pub fn write(self) {
        unsafe {
            _write(self.bits);
        }
    }
}

read_csr_as!(Dmw0, 0x180, __read_dmw0);
write_csr!(0x180, __write_dmw0);