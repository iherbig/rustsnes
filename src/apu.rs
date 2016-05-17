use memory::Memory;

pub struct APU;

impl APU {
    pub fn new() -> APU {
        APU
    }

    pub fn run(&mut self, memory: &mut Memory) {
        if memory.get_byte(0x2140) == 0xff {
            memory.set_byte(0x2140, 0xAA);
        }
    }
}
