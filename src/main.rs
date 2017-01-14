extern crate chip_8;

use chip_8::{Chip, Instruction};

fn main() {
    let mut chip = Chip::new();
    let instructions = [Instruction::LdByte(0, 100),
                        Instruction::Ld(1, 0),
                        Instruction::Shl(0),
                        Instruction::Shr(1),
                        Instruction::Sub(0, 1),
                        Instruction::Add(1, 0),
                        Instruction::LdByte(2, 57),
                        Instruction::Xor(1, 2),
                        Instruction::Ld(3, 1),
                        Instruction::And(3, 2),
                        Instruction::Rnd(5, 255),
                        Instruction::Rnd(5, 10)];

    for ins in &instructions {
        chip.execute(ins);
    }
}
