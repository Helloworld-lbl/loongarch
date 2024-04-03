use bit_field::BitField;

/// Pwcl register
#[derive(Clone, Copy)]
pub struct Pwcl {
    bits: usize,
}

impl Pwcl {
    #[inline]
    pub fn ptbase(&self) -> usize {
        self.bits.get_bits(0..5)
    }

    #[inline]
    pub fn ptwidth(&self) -> usize {
        self.bits.get_bits(5..10)
    }

    #[inline]
    pub fn dir1_base(&self) -> usize {
        self.bits.get_bits(10..15)
    }

    #[inline]
    pub fn dir1_width(&self) -> usize {
        self.bits.get_bits(15..20)
    }

    #[inline]
    pub fn dir2_base(&self) -> usize {
        self.bits.get_bits(20..25)
    }

    #[inline]
    pub fn dir2_width(&self) -> usize {
        self.bits.get_bits(25..30)
    }

    #[inline]
    pub fn ptewidth(&self) -> usize {
        self.bits.get_bits(30..32)
    }

    #[inline]
    pub fn set_ptbase(&mut self, value: usize) {
        self.bits.set_bits(0..5, value);
    }

    #[inline]
    pub fn set_ptwidth(&mut self, value: usize) {
        self.bits.set_bits(5..10, value);
    }

    #[inline]
    pub fn set_dir1_base(&mut self, value: usize) {
        self.bits.set_bits(10..15, value);
    }

    #[inline]
    pub fn set_dir1_width(&mut self, value: usize) {
        self.bits.set_bits(15..20, value);
    }

    #[inline]
    pub fn set_dir2_base(&mut self, value: usize) {
        self.bits.set_bits(20..25, value);
    }

    #[inline]
    pub fn set_dir2_width(&mut self, value: usize) {
        self.bits.set_bits(25..30, value);
    }

    #[inline]
    pub fn set_ptewidth(&mut self, value: usize) {
        self.bits.set_bits(30..32, value);
    }

    #[inline]
    pub fn write(&self) {
        unsafe {
            _write(self.bits)
        }
    }
}

read_csr_as!(Pwcl, 0x1c, __read_pwcl);
write_csr!(0x1c, __write_pwcl);