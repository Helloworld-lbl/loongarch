use bit_field::BitField;
/// Ticlr Register
#[derive(Clone, Copy, Debug)]
pub struct Ticlr {
    bits: usize,
}

impl Ticlr {
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }
    
    #[inline]
    pub fn set_clr(&mut self) {
        self.bits.set_bit(0, true);
    }

    #[inline]
    pub fn write(&self) {
        unsafe { _write(self.bits) };
    }

    #[inline]
    pub fn clear() {
        let mut ticlr = read();
        ticlr.set_clr();
        ticlr.write();
    }
}

read_csr_as!(Ticlr, 0x44, __read_sstatus);
write_csr!(0x44, __write_sstatus);
