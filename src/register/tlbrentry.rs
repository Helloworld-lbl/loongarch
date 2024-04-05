use bit_field::BitField;

/// Tlbrentry register
#[derive(Clone, Copy)]
pub struct Tlbrentry {
    bits: usize,
}

impl Tlbrentry {
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }
    
    #[inline]
    pub fn ppn(&self) -> usize {
        self.bits.get_bits(12..)
    }

    #[inline]
    pub fn set_ppn(&mut self, value: usize) {
        self.bits.set_bits(12.., value);
    }

    #[inline]
    pub fn write_pa(&mut self, pa: usize) {
        self.set_ppn(pa >> 12);
        self.write();
    }

    #[inline]
    pub fn write(&self) {
        unsafe {
            _write(self.bits)
        }
    }
}

read_csr_as!(Tlbrentry, 0x1c, __read_tlbrentry);
write_csr!(0x1c, __write_tlbrentry);

pub fn write_pa_to_tlbrentry(pa: usize) {
    let mut tlbrentry = read();
    tlbrentry.write_pa(pa);
}