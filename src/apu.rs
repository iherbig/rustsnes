use memory::Memory;

pub struct APU;

impl APU {
    pub fn new() -> APU {
        APU
    }

    pub fn run(&mut self, memory: &mut Memory) {
        if memory.get_byte(0x2140) == 0xFF && memory.get_byte(0x2141) == 0xF0 {
            memory.set_byte(0x2140, 0xAA);
            memory.set_byte(0x2141, 0xBB);
        }
    }
}
