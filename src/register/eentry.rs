//! stvec register

use bit_field::BitField;

/// stvec register
#[derive(Clone, Copy, Debug)]
pub struct Eentry {
    bits: usize,
}

impl Eentry {
    /// Returns the contents of the register as raw bits
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// VPN
    pub fn vpn(&self) -> usize {
        self.bits.get_bits(12..)
    }
}

read_csr_as!(Eentry, 0xc, __read_stvec);
write_csr!(0xc, __write_stvec);

/// Writes the CSR
#[inline]
pub unsafe fn write(vpn: usize) {
    _write(vpn << 12);
}