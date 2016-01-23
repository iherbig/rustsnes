#![allow(dead_code, unused_variables)]

pub mod cpu;
mod memory;
mod flags;
mod modes;

#[test]
fn test_memory() {
    let mut mem = memory::Memory::new();

    for i in 1..11 {
        mem.set_byte(i, i as u8);
    }

    for i in 1..11 {
        assert_eq!(i, mem.get_byte(i) as usize);
    }

    assert_eq!(0, mem.get_byte(12) as usize);
}

#[test]
fn test_adc_absolute() {
    use flags::Flags::*;

    let mut mem = memory::Memory::new();

    mem.set_byte(0x001000, 0x6D);
    mem.set_byte(0x001001, 0x10);
    mem.set_byte(0x001002, 0x03);
    mem.set_byte(0x001003, 0x2E);
    mem.set_byte(0x001004, 0x01);

    let mut cpu = cpu::CPU::new(mem);

    cpu.set_origin(0x001000);
    cpu.set_acc(40);

    cpu.execute(1);

    assert_eq!(cpu.check_acc() as usize, 342);
    assert!(!cpu.check_flag(NegativeFlag));
    assert!(!cpu.check_flag(OverflowFlag));
    assert!(!cpu.check_flag(ZeroFlag));
    assert!(!cpu.check_flag(CarryFlag));
}

#[test]
fn test_adc_absolute_nc() {
    use flags::Flags::*;

    let mut mem = memory::Memory::new();

    mem.set_byte(0x001000, 0x6D);
    mem.set_byte(0x001001, 0x10);
    mem.set_byte(0x001002, 0x03);
    mem.set_byte(0x001003, 0x00);
    mem.set_byte(0x001004, 0x40);

    let mut cpu = cpu::CPU::new(mem);

    cpu.set_origin(0x001000);
    cpu.set_acc(0x4000);

    cpu.execute(1);

    assert_eq!(cpu.check_acc() as usize, 0x8000);
    assert!(cpu.check_flag(NegativeFlag));
    assert!(cpu.check_flag(OverflowFlag));
    assert!(!cpu.check_flag(ZeroFlag));
    assert!(!cpu.check_flag(CarryFlag));
}

#[test]
fn test_adc_absolute_v() {
    use flags::Flags::*;

    let mut mem = memory::Memory::new();

    mem.set_byte(0x001000, 0x6D);
    mem.set_byte(0x001001, 0x10);
    mem.set_byte(0x001002, 0x03);
    mem.set_byte(0x001003, 0x00);
    mem.set_byte(0x001004, 0x80);

    let mut cpu = cpu::CPU::new(mem);

    cpu.set_origin(0x001000);
    cpu.set_acc(0x8000);

    cpu.execute(1);

    assert_eq!(cpu.check_acc() as usize, 0);
    assert!(!cpu.check_flag(NegativeFlag));
    assert!(!cpu.check_flag(OverflowFlag));
    assert!(!cpu.check_flag(ZeroFlag));
    assert!(cpu.check_flag(CarryFlag));
}

#[test]
fn test_adc_absolute_z() {
    use flags::Flags::*;

    let mut mem = memory::Memory::new();

    mem.set_byte(0x001000, 0x6D);
    mem.set_byte(0x001001, 0x10);
    mem.set_byte(0x001002, 0x03);
    mem.set_byte(0x001003, 0x00);
    mem.set_byte(0x001004, 0x00);

    let mut cpu = cpu::CPU::new(mem);

    cpu.set_origin(0x001000);
    cpu.set_acc(0x0000);

    cpu.execute(1);

    assert_eq!(cpu.check_acc() as usize, 0);
    assert!(!cpu.check_flag(NegativeFlag));
    assert!(!cpu.check_flag(OverflowFlag));
    assert!(cpu.check_flag(ZeroFlag));
    assert!(!cpu.check_flag(CarryFlag));
}
