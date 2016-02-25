#![allow(dead_code, unused_variables)]

use std::env;
use std::fs::File;
use std::io::Read;

mod cpu;
mod ppu;
mod memory;
mod modes;
mod snes;

fn main() {
    let rom = { 
        let romname = env::args().nth(1).unwrap();
        read_bin(romname)
    };

    let mem = memory::Memory::new(rom);
    let ppu = ppu::PPU::new();
    let cpu = cpu::CPU::new(&mem);
    let mut snes = snes::SNES::new(cpu, ppu, mem);

    snes.run();
}

fn read_bin(rom_path: String) -> Vec<u8> {
    let mut rom = Vec::new();
    let mut file = File::open(&rom_path);

    if let Ok(ref mut the_file) = file {
        let _res = the_file.read_to_end(&mut rom);
    } else {
        panic!("Could not open the file {}", rom_path);
    }

    rom
}

