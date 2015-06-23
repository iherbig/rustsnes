use std::io::Error;
use std::fs::File;
use std::io::Read;

pub struct Memory {
	ram: [u8; 131072],
	rom: Vec<u8>,
}

impl Memory {
	pub fn new(rom: File) -> Memory {
		let mut memory = Memory { ram: [0; 131072], rom: Vec::new() };
		match memory.load_rom(rom) {
			Ok(_) => memory,
			Err(e) => panic!("Could not initialize ROM: {}", e),
		}
	}

	fn load_rom(&mut self, rom: File) -> Result<(), Error> {
		for byte in rom.bytes() {
			match byte {
				Ok(b) => {
					self.rom.push(b);
				},
				Err(e) => return Err(e),
			}
		}

		Ok(())
	}

	pub fn get_word_from_ram(&self, address: usize) -> u16 {
		let lowbyte:  u8 = self.ram[address];
		let highbyte: u8 = self.ram[address+1];

		let mut result = lowbyte as u16;
		result |= (highbyte as u16) << 8;

		result
	}

	pub fn get_word_from_rom(&self, address: usize) -> u16 {
		let lowbyte:  u8 = self.rom[address];
		let highbyte: u8 = self.rom[address+1];

		let mut result = lowbyte as u16;
		result |= (highbyte as u16) << 8;

		result
	}

	pub fn set_ram_word(&mut self, address: usize, value: u16) {
		let lowbyte:  u8 = (value & 0x00ff) as u8;
		let highbyte: u8 = ((value & 0xff00) >> 8) as u8;

		self.ram[address]   = lowbyte;
		self.ram[address+1] = highbyte;
	}

	pub fn set_rom_word(&mut self, address: usize, value: u16) {
		let lowbyte:  u8 = (value & 0x00ff) as u8;
		let highbyte: u8 = ((value & 0xff00) >> 8) as u8;

		self.rom[address]   = lowbyte;
		self.rom[address+1] = highbyte;
	}
}

