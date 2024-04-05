use bit_field::BitField;

/// Pwcl register
#[derive(Clone, Copy)]
pub struct Pwch {
    bits: usize,
}

impl Pwch {
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }
    
    #[inline]
    pub fn dir3_base(&self) -> usize {
        self.bits.get_bits(0..6)
    }

    #[inline]
    pub fn dir3_width(&self) -> usize {
        self.bits.get_bits(6..12)
    }

    #[inline]
    pub fn dir4_base(&self) -> usize {
        self.bits.get_bits(12..18)
    }

    #[inline]
    pub fn dir4_width(&self) -> usize {
        self.bits.get_bits(18..24)
    }

    #[inline]
    pub fn set_dir3_base(&mut self, value: usize) {
        self.bits.set_bits(0..6, value);
    }

    #[inline]
    pub fn set_dir3_width(&mut self, value: usize) {
        self.bits.set_bits(6..12, value);
    }

    #[inline]
    pub fn set_dir4_base(&mut self, value: usize) {
        self.bits.set_bits(12..18, value);
    }

    #[inline]
    pub fn set_dir4_width(&mut self, value: usize) {
        self.bits.set_bits(18..24, value);
    }

    #[inline]
    pub fn write(&self) {
        unsafe {
            _write(self.bits)
        }
    }
}

read_csr_as!(Pwch, 0x1d, __read_pwch);
write_csr!(0x1d, __write_pwch);