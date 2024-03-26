// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
#![no_std]
#![cfg_attr(feature = "inline-asm", feature(asm_const))]
extern crate bare_metal;
#[macro_use]
extern crate bitflags;
extern crate bit_field;

pub mod register;
pub mod consts;
pub mod ipi;
