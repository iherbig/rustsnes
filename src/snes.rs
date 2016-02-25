use memory::Memory;
use cpu::CPU;
use ppu::PPU;

pub struct SNES {
    cpu: CPU,
    ppu: PPU,
    memory: Memory,
}

impl SNES {
    pub fn new(cpu: CPU, ppu: PPU, mem: Memory) -> SNES {
        SNES {
            cpu: cpu,
            ppu: ppu,
            memory: mem,
        }
    }
    pub fn run(&mut self) {
        self.cpu.run(&mut self.memory);
        self.ppu.run(&mut self.memory);
    }
}
