use emu::memory as mem;

pub struct Video {
	pub p: u8,
}

impl Video {
	pub fn new() -> Video {
		Video {
			p: 1,
		}
	}
	
	pub fn execute(&self, memory: &mut mem::Memory, cycles: u8) {
		println!("Drawing stuff!");
	}
}

