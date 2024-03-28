use bit_field::BitField;

/// CPUCFG register
#[derive(Clone, Copy)]
pub struct Cpucfg {}

impl Cpucfg {
    #[inline]
    unsafe fn read(word: usize) -> usize {
        match () {
            #[cfg(all(feature = "inline-asm"))]
            () => {
                let r: usize;
                core::arch::asm!("cpucfg {0}, {1}", out(reg) r, in(reg) word);
                r
            }

            #[cfg(all(not(feature = "inline-asm")))]
            () => {
                extern "C" {
                    fn __read_cpucfg() -> usize;
                }

                __read_cpucfg()
            }
        }
    }

    #[inline]
    pub unsafe fn get_cc_freq() -> usize {
        Self::read(4)
    }

    #[inline]
    pub unsafe fn get_cc_mul() -> usize {
        Self::read(5).get_bits(0..16) as usize
    }

    #[inline]
    pub unsafe fn get_cc_div() -> usize {
        Self::read(5).get_bits(16..32) as usize
    }

    pub unsafe fn get_sc_freq() -> usize {
        Self::get_cc_freq() * Self::get_cc_mul() / Self::get_cc_div()
    }
}
