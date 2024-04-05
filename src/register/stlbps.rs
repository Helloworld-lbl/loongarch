use bit_field::BitField;

/// Stlbps register
#[derive(Clone, Copy)]
pub struct Stlbps {
    bits: usize,
}

impl Stlbps {
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }
    
    #[inline]
    pub fn ps(&self) -> usize {
        self.bits.get_bits(0..6)
    }

    #[inline]
    pub fn set_ps(&mut self, value: usize){
        self.bits.set_bits(0..6, value);
    }

    #[inline]
    pub fn write(&self) {
        unsafe {
            _write(self.bits)
        }
    }
}

read_csr_as!(Stlbps, 0x1e, __read_stlbps);
write_csr!(0x1e, __write_stlbps);