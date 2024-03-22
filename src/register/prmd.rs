//! sstatus register

pub use super::crmd::PLV;
use bit_field::BitField;
use core::mem::size_of;

/// Prmd Register
#[derive(Clone, Copy, Debug)]
pub struct Prmd {
    bits: usize,
}

impl Prmd {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    // PPLV
    #[inline]
    pub fn pplv(&self) -> PLV {
        match self.bits.get_bits(0..2) {
            0 => PLV::PLV0,
            1 => PLV::PLV1,
            2 => PLV::PLV2,
            3 => PLV::PLV3,
            _ => unreachable!(),
        }
    }

    // PIE
    #[inline]
    pub fn pie(&self) -> bool {
        self.bits.get_bit(2)
    }

    // PWE
    #[inline]
    pub fn pwe(&self) -> bool {
        self.bits.get_bit(3)
    }

    // /// Whether either the FS field or XS field
    // /// signals the presence of some dirty state
    // #[inline]
    // pub fn sd(&self) -> bool {
    //     self.bits.get_bit(size_of::<usize>() * 8 - 1)
    // }

    #[inline]
    pub fn set_pplv(&mut self, val: PLV) {
        self.bits.set_bits(0..2, val as usize);
    }

    #[inline]
    pub fn set_pie(&mut self, val: bool) {
        self.bits.set_bit(2, val);
    }

    #[inline]
    pub fn set_pwe(&mut self, val: bool) {
        self.bits.set_bit(3, val);
    }
}

read_csr_as!(Prmd, 0x1, __read_sstatus);
write_csr!(0x1, __write_sstatus);