use bit_field::BitField;

/// Tlbrehi register
#[derive(Clone, Copy)]
pub struct Tlbrehi {
    bits: usize,
}

impl Tlbrehi {
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    #[inline]
    pub fn ps(&self) -> usize {
        self.bits.get_bits(0..6)
    }

    #[inline]
    pub fn set_ps(&mut self, value: usize) {
        self.bits.set_bits(0..6, value);
    }

    #[inline]
    pub fn write_ps(&mut self, ps: usize) {
        self.set_ps(ps);
        self.write();
    }

    #[inline]
    pub fn write(&self) {
        unsafe {
            _write(self.bits)
        }
    }
}

read_csr_as!(Tlbrehi, 0x8e, __read_tlbrehi);
write_csr!(0x8e, __write_tlbrehi);

pub fn write_ps_to_tlbrehi(ps: usize) {
    let mut tlbrehi = read();
    tlbrehi.write_ps(ps);
}