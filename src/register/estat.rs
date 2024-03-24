//! scause register

use bit_field::BitField;
use core::mem::size_of;

/// Estat register
#[derive(Clone, Copy)]
pub struct Estat {
    bits: usize,
}

/// Trap Cause
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Trap {
    Interrupt(Interrupt),
    Exception(Exception),
}

/// Interrupt
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Interrupt {
    UserSoft,
    VirtualSupervisorSoft,
    SupervisorSoft,
    UserTimer,
    VirtualSupervisorTimer,
    SupervisorTimer,
    UserExternal,
    VirtualSupervisorExternal,
    SupervisorExternal,
    Unknown,
}

/// Exception
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Exception {
    INT,
    PIL,
    PIS,
    PIF,
    PME,
    PNR,
    PNX,
    PPI,
    ADEF,
    ADEM,
    ALE,
    BCE,
    SYS,
    BRK,
    INE,
    IPE,
    FPD,
    SXD,
    ASXD,
    FPE,
    VFPE,
    WPEF,
    WPEM,
    BTD,
    BTE,
    GSPR,
    HVC,
    GCSC,
    GCHC,
    Unknown,
}

// impl Interrupt {
//     pub fn from(nr: usize) -> Self {
//         match nr {
//             0 => Interrupt::UserSoft,
//             1 => Interrupt::SupervisorSoft,
//             2 => Interrupt::VirtualSupervisorSoft,
//             4 => Interrupt::UserTimer,
//             5 => Interrupt::SupervisorTimer,
//             6 => Interrupt::VirtualSupervisorTimer,
//             8 => Interrupt::UserExternal,
//             9 => Interrupt::SupervisorExternal,
//             10 => Interrupt::VirtualSupervisorExternal,
//             _ => Interrupt::Unknown,
//         }
//     }
// }

impl Exception {
    pub fn from(Ecode: usize, EsubCode: usize) -> Self {
        match Ecode {
            0x0 => Exception::INT,
            0x1 => Exception::PIL,
            0x2 => Exception::PIS,
            0x3 => Exception::PIF,
            0x4 => Exception::PME,
            0x5 => Exception::PNR,
            0x6 => Exception::PNX,
            0x7 => Exception::PPI,
            0x8 => {
                match EsubCode {
                    0 => Exception::ADEF,
                    1 => Exception::ADEM,
                    _ => Exception::Unknown,
                }
            },
            0x9 => Exception::ALE,
            0xa => Exception::BCE,
            0xb => Exception::SYS,
            0xc => Exception::BRK,
            0xd => Exception::INE,
            0xe => Exception::IPE,
            0xf => Exception::FPD,
            0x10 => Exception::SXD,
            0x11 => Exception::ASXD,
            0x12 => {
                match EsubCode {
                    0 => Exception::FPE,
                    1 => Exception::VFPE,
                    _ => Exception::Unknown,
                }
            },
            0x13 => {
                match EsubCode {
                    0 => Exception::WPEF,
                    1 => Exception::WPEM,
                    _ => Exception::Unknown,
                }
            },
            0x14 => Exception::BTD,
            0x15 => Exception::BTE,
            0x16 => Exception::GSPR,
            0x17 => Exception::HVC,
            0x18 => {
                match EsubCode {
                    0 => Exception::GCSC,
                    1 => Exception::GCHC,
                    _ => Exception::Unknown,
                }
            },
            _ => Exception::Unknown,
        }
    }
}

impl Estat {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    #[inline]
    pub fn is_rw(&self) -> usize {
        self.bits.get_bits(0..2)
    }

    #[inline]
    pub fn is_r(&self) -> usize {
        self.bits.get_bits(2..13)
    }

    #[inline]
    pub fn ecode(&self) -> usize {
        self.bits.get_bits(16..22)
    }

    #[inline]
    pub fn esubcode(&self) -> usize {
        self.bits.get_bits(22..31)
    }

    // /// Returns the code field
    // pub fn code(&self) -> usize {
    //     let bit = 1 << (size_of::<usize>() * 8 - 1);
    //     self.bits & !bit
    // }

    /// Trap Cause
    #[inline]
    pub fn cause(&self) -> Trap {
        Trap::Exception(Exception::from(self.ecode(), self.esubcode()))
    }

    // /// Is trap cause an interrupt.
    // #[inline]
    // pub fn is_interrupt(&self) -> bool {
    //     self.bits.get_bit(size_of::<usize>() * 8 - 1)
    // }

    // /// Is trap cause an exception.
    // #[inline]
    // pub fn is_exception(&self) -> bool {
    //     !self.is_interrupt()
    // }
}

read_csr_as!(Estat, 0x5, __read_scause);