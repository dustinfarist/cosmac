use crate::{Chip, Instruction};
use crate::components::AddressableStorage;

macro_rules! register_eq {
    ($chip:tt, $vx:expr, $value:expr) => (assert_eq!($chip.register.get($vx), $value);)
}

#[test]
fn load_byte() {
    let mut chip = Chip::new();
    chip.execute(&Instruction::LdByte(0, 10));
    chip.execute(&Instruction::LdByte(1, 15));
    register_eq!(chip, 0, 10);
    register_eq!(chip, 1, 15);
}

#[test]
fn load_byte_overwrites_existing_value() {
    let mut chip = Chip::new();
    chip.execute(&Instruction::LdByte(0, 10));
    chip.execute(&Instruction::LdByte(0, 15));
    register_eq!(chip, 0, 15);
}

#[test]
fn bitwise_and() {
    let mut chip = Chip::with_register_values(&[10, 15]);
    chip.execute(&Instruction::And(1, 0));
    register_eq!(chip, 1, 10 & 15);
}

#[test]
fn bitwise_or() {
    let mut chip = Chip::with_register_values(&[10, 15]);
    chip.execute(&Instruction::Or(1, 0));
    register_eq!(chip, 1, 10 | 15);
}

#[test]
fn bitwise_xor() {
    let mut chip = Chip::with_register_values(&[10, 15]);
    chip.execute(&Instruction::Xor(1, 0));
    register_eq!(chip, 1, 10 ^ 15);
}

#[test]
fn add_works() {
    let mut chip = Chip::with_register_values(&[10, 15]);
    chip.execute(&Instruction::Add(1, 0));
    register_eq!(chip, 1, 25);
}

#[test]
fn add_sets_carry_flag_on_overflow() {
    let mut chip = Chip::with_register_values(&[255, 1]);
    chip.execute(&Instruction::Add(0, 1));
    register_eq!(chip, 0, 0);
    register_eq!(chip, 1, 1);
    register_eq!(chip, 0xF, 1);
}

#[test]
fn subtract_when_vx_greater_than_vy() {
    let mut chip = Chip::with_register_values(&[100, 25]);
    chip.execute(&Instruction::Sub(0, 1));
    register_eq!(chip, 0, 75);
    register_eq!(chip, 0xF, 1);
}

#[test]
fn subtract_when_vx_less_than_vy() {
    let mut chip = Chip::with_register_values(&[25, 100]);
    chip.execute(&Instruction::Sub(0, 1));
    register_eq!(chip, 0, 181); // 256 + (-75)
    register_eq!(chip, 0xF, 0);
}

#[test]
fn add_then_subtract_restores_state() {
    let mut chip = Chip::with_register_values(&[100, 25, 100]);
    chip.execute(&Instruction::Add(0, 1));
    register_eq!(chip, 0, 125);

    chip.execute(&Instruction::Sub(0, 1));
    register_eq!(chip, 0, 100);
}

#[test]
fn add_then_subtract_restores_state_with_overflows() {
    let mut chip = Chip::with_register_values(&[255, 100, 100]);
    chip.execute(&Instruction::Add(0, 1));
    register_eq!(chip, 0, 99);
    register_eq!(chip, 0xF, 1);

    chip.execute(&Instruction::Sub(0, 1));
    register_eq!(chip, 0, 255);
    register_eq!(chip, 0xF, 0);
}

#[test]
fn shift_right_with_odd_number_sets_vf_flag() {
    let mut chip = Chip::with_register_values(&[5]);
    chip.execute(&Instruction::Shr(0));
    register_eq!(chip, 0, 2);
    register_eq!(chip, 0xF, 1);
}

#[test]
fn shift_right_with_even_number_does_not_set_vf_flag() {
    let mut chip = Chip::with_register_values(&[6]);
    chip.execute(&Instruction::Shr(0));
    register_eq!(chip, 0, 3);
    register_eq!(chip, 0xF, 0);
}

#[test]
fn shift_left_then_shift_right_restores_state() {
    let mut chip = Chip::with_register_values(&[100]);
    chip.execute(&Instruction::Shl(0));
    register_eq!(chip, 0, 200);

    chip.execute(&Instruction::Shr(0));
    register_eq!(chip, 0, 100);

    chip.execute(&Instruction::Shr(0));
    register_eq!(chip, 0, 50);

    chip.execute(&Instruction::Shl(0));
    register_eq!(chip, 0, 100);
}

#[test]
fn shift_right_then_shift_left_loses_info_with_odd_number() {
    let mut chip = Chip::with_register_values(&[5]);
    chip.execute(&Instruction::Shr(0));
    register_eq!(chip, 0, 2);
    register_eq!(chip, 0xF, 1);

    chip.execute(&Instruction::Shl(0));
    register_eq!(chip, 0, 4);
    register_eq!(chip, 0xF, 0);
}

#[test]
fn shift_left_with_overflow_sets_vf_flag() {
    let mut chip = Chip::with_register_values(&[150]);
    chip.execute(&Instruction::Shl(0));
    register_eq!(chip, 0, 44);
    register_eq!(chip, 0xF, 1);
}

#[test]
fn shift_left_then_shift_right_loses_info_with_overflow() {
    let mut chip = Chip::with_register_values(&[150]);
    chip.execute(&Instruction::Shl(0));
    register_eq!(chip, 0, 44);
    register_eq!(chip, 0xF, 1);

    chip.execute(&Instruction::Shr(0));
    register_eq!(chip, 0, 22);
    register_eq!(chip, 0xF, 0);
}

#[test]
fn random_number_will_always_be_less_than_mask() {
    let mut chip = Chip::new();
    for _ in 0..100 {
        chip.execute(&Instruction::Rnd(0, 1));
        assert!(chip.register.get(0) <= 1);
    }
}

#[test]
fn add_byte_works() {
    let mut chip = Chip::with_register_values(&[100]);
    chip.execute(&Instruction::AddByte(0, 100));
    register_eq!(chip, 0, 200);
}

#[test]
fn add_byte_truncates_overflows() {
    let mut chip = Chip::with_register_values(&[255]);
    chip.execute(&Instruction::AddByte(0, 100));
    register_eq!(chip, 0, 99);
}
