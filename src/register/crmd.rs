use bit_field::BitField;
/// Crmd Register
#[derive(Clone, Copy, Debug)]
pub struct Crmd {
    bits: usize,
}

// PLV
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PLV {
    PLV0 = 0,
    PLV1 = 1,
    PLV2 = 2,
    PLV3 = 3
}

impl Crmd {
    #[inline]
    pub fn plv(&self) -> PLV {
        match self.bits.get_bits(0..2) {
            0 => PLV::PLV0,
            1 => PLV::PLV1,
            2 => PLV::PLV2,
            3 => PLV::PLV3,
            _ => unreachable!()
        }
    }

    #[inline]
    pub fn ie(&self) -> bool {
        self.bits.get_bit(2)
    }

    #[inline]
    pub fn da(&self) -> bool {
        self.bits.get_bit(3)
    }

    #[inline]
    pub fn pg(&self) -> bool {
        self.bits.get_bit(4)
    }

    #[inline]
    pub fn datf(&self) -> usize {
        self.bits.get_bits(5..7)
    }

    #[inline]
    pub fn datm(&self) -> usize {
        self.bits.get_bits(7..9)
    }

    #[inline]
    pub fn we(&self) -> bool {
        self.bits.get_bit(9)
    }

    #[inline]
    pub fn set_da(&mut self, value: bool) {
        self.bits.set_bit(3, value);
    }

    #[inline]
    pub fn set_pg(&mut self, value: bool) {
        self.bits.set_bit(4, value);
    }

    #[inline]
    pub fn enable_da(&mut self) {
        self.set_da(true);
        self.set_pg(false);
    }

    #[inline]
    pub fn enable_pg(&mut self) {
        self.set_da(false);
        self.set_pg(true);
    }
}