extern crate rand;

#[macro_use]
mod macros;

mod chip;
mod instruction;
pub mod components;

#[cfg(test)]
mod tests;

pub use crate::chip::Chip;
pub use crate::instruction::Instruction;
