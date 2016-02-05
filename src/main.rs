#![allow(dead_code, unused_variables)]

use std::env;
use std::fs::File;
use std::path::Path;
use std::io::Read;

pub mod cpu;
mod memory;
mod flags;
mod modes;

fn main() {
    let rom = { 
        let romname = env::args().nth(1).unwrap();
        read_bin(romname)
    };

    let mem = memory::Memory::new(rom);
    let cpu = cpu::CPU::new(mem);
}

fn read_bin<P: AsRef<Path>>(rom_name: P) -> Vec<u8> {
    let mut rom = Vec::new();
    let mut file = File::open(rom_name).unwrap();

    file.read_to_end(&mut rom).unwrap();

    rom
}

#[allow(unused_imports)]
mod tests {
    use super::memory;
    use super::cpu;
    
    #[test]
    fn test_memory() {
        let mut mem = memory::Memory::new(vec![]);

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

        let mut mem = memory::Memory::new(vec![]);

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

        let mut mem = memory::Memory::new(vec![]);

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

        let mut mem = memory::Memory::new(vec![]);

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

        let mut mem = memory::Memory::new(vec![]);

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
}
