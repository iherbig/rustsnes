use memory::Memory;
use cpu::CPU;
use ppu::PPU;
use apu::APU;

pub struct SNES {
    cpu: CPU,
    ppu: PPU,
    apu: APU,
    memory: Memory,
}

impl SNES {
    pub fn new(cpu: CPU, ppu: PPU, apu: APU, mem: Memory) -> SNES {
        SNES {
            cpu: cpu,
            ppu: ppu,
            apu: apu,
            memory: mem,
        }
    }
    pub fn run(&mut self) {
        self.cpu.run(&mut self.memory);
        self.ppu.run(&mut self.memory);
        self.apu.run(&mut self.memory);
    }
}
