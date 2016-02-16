use memory::Memory;
use cpu::CPU;

pub struct SNES {
    cpu: CPU,
    memory: Memory,
}

impl SNES {
    pub fn new(cpu: CPU, mem: Memory) -> SNES {
        SNES {
            cpu: cpu,
            memory: mem,
        }
    }
    pub fn run(&mut self) {
        self.cpu.run(&mut self.memory);
    }
}
