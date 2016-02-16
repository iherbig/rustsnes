#![allow(dead_code, unused_variables)]

use std::env;
use std::fs::File;
use std::path::Path;
use std::io::Read;

mod cpu;
mod memory;
mod modes;
mod snes;

fn main() {
    let rom = { 
        let romname = env::args().nth(1).unwrap();
        read_bin(romname)
    };

    let mem = memory::Memory::new(rom);
    let cpu = cpu::CPU::new(&mem);
    let mut snes = snes::SNES::new(cpu, mem);

    snes.run();
}

fn read_bin<P: AsRef<Path>>(rom_name: P) -> Vec<u8> {
    let mut rom = Vec::new();
    let mut file = File::open(rom_name).unwrap();

    file.read_to_end(&mut rom).unwrap();

    rom
}

