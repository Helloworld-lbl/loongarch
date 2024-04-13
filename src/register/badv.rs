/// Badv Register
#[derive(Clone, Copy, Debug)]
pub struct Badv {
    bits: usize,
}

impl Badv {
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }
}

read_csr_as!(Badv, 0x7, __read_badv);