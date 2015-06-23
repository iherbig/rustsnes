use emu::memory as mem;

pub struct Audio {
	pub p: u8,
}

impl Audio {
	pub fn new() -> Audio {
		Audio {
			p: 1,
		}
	}

	pub fn execute(&self, memory: &mut mem::Memory, cycles: u8) {
		println!("Make sound!");
	}
}

