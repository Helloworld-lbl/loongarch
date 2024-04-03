use bit_field::BitField;
/// Pgdl Register
/// 该寄存器用于配置高半地址空间的全局目录的基址。要求全局目录的基址一定是 4KB 边界地址对齐的，所以该寄存器的最低 12 位软件不可配置，只读恒为 0。
#[derive(Clone, Copy, Debug)]
pub struct Pgdh {
    bits: usize,
}

impl Pgdh {
    #[inline]
    pub fn base(self) -> usize {
        self.bits.get_bits(12..)
    }

    #[inline]
    pub fn set_base(&mut self, value: usize) {
        self.bits.set_bits(12.., value);
    }

    #[inline]
    pub fn write_base_pa(&mut self, pa: usize) {
        self.set_base(pa >> 12);
        self.write();
    }

    #[inline]
    pub fn write(self) {
        unsafe {
            _write(self.bits);
        }
    }
}

read_csr_as!(Pgdh, 0x1a, __read_pgdh);
write_csr!(0x1a, __write_pgdh);

pub fn write_pa_to_pgdh(pa: usize) {
    let mut pgdh = read();
    pgdh.write_base_pa(pa);
}