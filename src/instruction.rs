#[derive(Debug)]
pub enum Instruction {
    /// 0nnn - SYS addr
    /// Jump to a machine code routine at nnn.
    /// This instruction is only used on the old computers on which Chip-8
    /// was originally implemented. It is ignored by modern interpreters.
    Sys(u16),

    /// 00E0 - CLS
    /// Clear the display.
    Cls,

    /// 00EE - RET
    /// Return from a subroutine.
    /// The interpreter sets the program counter to the address at the top of the stack,
    /// then subtracts 1 from the stack pointer.
    Ret,

    /// 1nnn - JP addr
    /// Jump to the location nnn.
    /// The interpreter sets the program counter to nnn.
    Jp(u16),

    /// 2nnn - CALL addr
    /// Call subroutine at nnn.
    /// The interpreter increments the stack pointer,
    /// then puts the current PC on the top of the stack.
    /// The PC is then set to nnn.
    Call(u16),

    /// 3xkk - SE Vx, byte
    /// Skip next instruction if Vx = kk.
    /// The interpreter compares register Vx to kk, and
    /// if they are equal, increments the program counter by 2.
    SeByte(usize, u8),

    /// 4xkk - SNE Vx, byte
    /// Skip next instruction if Vx != kk.
    /// The interpreter compares register Vx to kk, and
    /// if they are not equal, increments the program counter by 2.
    SneByte(usize, u8),

    /// 5xkk - SE Vx, Vy
    /// Skip next instruction if Vx = Vy.
    /// The interpreter compares register Vx to register Vy, and
    /// if they are equal, increments the program counter by 2.
    Se(usize, usize),

    /// 6xkk - LD Vx, byte
    /// Set Vx = kk.
    /// The interpreter puts the value kk into register Vx.
    LdByte(usize, u8),

    /// 7xkk - ADD Vx, byte
    /// Set Vx = Vx + kk.
    /// Adds the value kk to the value of register Vx, then stores the result in Vx.
    AddByte(usize, u8),

    /// 8xy0 - LD Vx, Vy
    /// Set Vx = Vy.
    /// Stores the value of register Vy in register Vx.
    Ld(usize, usize), // Load

    /// 8xy1 - OR Vx, Vy
    /// Set Vx = Vx OR Vy.
    /// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
    Or(usize, usize),

    /// 8xy2 - AND Vx, Vy
    /// Set Vx = Vx AND Vy.
    /// Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
    And(usize, usize),

    /// 8xy3 - XOR Vx, Vy
    /// Set Vx = Vx XOR Vy.
    /// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
    Xor(usize, usize),

    /// 8xy4 - ADD Vx, Vy
    /// Set Vx = Vx + Vy, set VF = carry.
    /// The values of Vx and Vy are added together.
    /// If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
    /// Only the lowest 8 bits of the result are kept, and stored in Vx.
    Add(usize, usize),

    /// 8xy5 - SUB Vx, Vy
    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    /// If Vx > Vy, then VF is set to 1, otherwise 0.
    /// Then Vy is subtracted from Vx, and the results stored in Vx.
    Sub(usize, usize),

    /// 8xy6 - SHR Vx {, Vy}
    /// Set Vx = Vx SHR 1.
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0.
    /// Then Vx is divided by 2.
    Shr(usize), // Shift Right

    /// 8xy7 - SUBN Vx, Vy
    /// Set Vx = Vy - Vx, set VF = NOT borrow.
    /// If Vy > Vx, then VF is set to 1, otherwise 0.
    /// Then Vx is subtracted from Vy, and the results stored in Vx.
    Subn(usize, usize),

    /// 8xyE - SHL Vx {, Vy}
    /// Set Vx = Vx SHL 1.
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0.
    /// Then Vx is multiplied by 2.
    Shl(usize), // Shift Left

    /// 9xy0 - SNE Vx, Vy
    /// Skip next instruction if Vx != Vy.
    /// The values of Vx and Vy are compared, and
    /// if they are not equal, the program counter is increased by 2.
    Sne(usize, usize),

    /// Annn - LD I, addr
    /// Set I = nnn.
    /// The value of register I is set to nnn.
    Ldi(u16),

    /// Bnnn - JP V0, addr
    /// Jump to location nnn + V0.
    /// The program counter is set to nnn plus the value of V0.
    JpV0(u16),

    /// Cxkk - RND Vx, byte
    /// Set Vx = random byte AND kk.
    /// Generates a random number from 0 to 255, which is then ANDed with the value kk.
    /// The results are stored in Vx. See instruction 8xy2 for more information on AND.
    Rnd(usize, u8),

    /// Fx07 - LD Vx, DT
    /// Set Vx = delay timer value.
    /// The value of DT is placed into Vx.
    LdVxDelay(usize),

    /// Fx15 - LD DT, Vx
    /// Set delay timer = Vx
    /// DT is set equal to the value of Vx.
    LdDelayVx(usize),
}

impl Instruction {
    pub fn parse(op: u16) -> Instruction {
        let instruction = (op >> 12 & 15, (op >> 8) & 15, (op >> 4) & 15, op & 15);
        match instruction {
            (6, x, a, b) => Instruction::LdByte(x as usize, (((a << 4) + b) & 255) as u8),
            (8, x, y, 0) => Instruction::Ld(x as usize, y as usize),
            (8, x, y, 1) => Instruction::Or(x as usize, y as usize),
            (8, x, y, 2) => Instruction::And(x as usize, y as usize),
            (8, x, y, 3) => Instruction::Xor(x as usize, y as usize),
            (8, x, y, 4) => Instruction::Add(x as usize, y as usize),
            (8, x, y, 5) => Instruction::Sub(x as usize, y as usize),
            (8, x, _, 6) => Instruction::Shr(x as usize),
            (8, x, y, 7) => Instruction::Subn(x as usize, y as usize),
            (8, x, _, 0xE) => Instruction::Shl(x as usize),
            _ => unimplemented!(),
        }
    }
}
