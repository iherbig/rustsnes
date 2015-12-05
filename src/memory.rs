use std::io::Error;
use std::fs::File;
use std::io::Read;

pub struct Memory {
	ram: [u8; 4096],
}

impl Memory {
    pub fn new() -> Memory {
        Memory { ram: [0; 4096] }
    }

	pub fn get_byte(&self, address: usize) -> u8 {
		self.ram[address]
	}

	pub fn set_byte(&mut self, address: usize, value: u8) {
		self.ram[address] = value;
	}

	pub fn get_word(&self, address: usize) -> u16 {
		let lowbyte:  u8 = self.ram[address];
		let highbyte: u8 = self.ram[address+1];

		let mut result = lowbyte as u16;
		result |= (highbyte as u16) << 8;

		result
	}

	pub fn set_word(&mut self, address: usize, value: u16) {
		let lowbyte:  u8 = (value & 0x00ff) as u8;
		let highbyte: u8 = ((value & 0xff00) >> 8) as u8;

		self.ram[address]   = lowbyte;
		self.ram[address+1] = highbyte;
	}
}

