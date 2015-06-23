use std::intrinsics;

use emu::memory as mem;

mod flags;

pub struct CPU {
	accumulator:      u16,
	index_x:          u16,
	index_y:          u16,
	stack_pointer:    u16,
	data_bank:        u16,
	direct_page:      u16,
	program_bank:     u16,
	processor_status: u16,
	program_counter:  u16,
}

impl CPU {
	pub fn new() -> CPU {
		CPU {
			accumulator:      0,
			index_x:          0,
			index_y:          0,
			stack_pointer:    0,
			data_bank:        0,
			direct_page:      0,
			program_bank:     0,
			processor_status: 0,
			program_counter:  0,
		}
	}
	
	pub fn execute(&self, memory: &mut mem::Memory) -> u8 {
		println!("Execute!");
		
		memory.set_ram_word(0xfff0, 234);
		let result = memory.get_word_from_ram(0xfff0);
		
		println!("Retrieved {:08b}", result);

		let first_two = [memory.get_word_from_rom(0), memory.get_word_from_rom(1)];

		println!("First two words in ROM are: {:?}", first_two);

		1
	}

	fn check_flag(&self, mask: flags::Flags) -> Result<bool, &'static str> {
		match mask {
			flags::Flags::IndexRegisterSizeFlag => {
				match self.check_flag(flags::Flags::NativeModeFlag) {
					Ok(true)  => Ok(self.processor_status & flags::Flags::IndexRegisterSizeFlag == 1_u16),
					Ok(false) => Err("Cannot check index registers size flag when not CPU is in emulator mode."),
					Err(_)    => unreachable!(),
				}
			},
			flags::Flags::AccumulatorRegisterSizeFlag => {
				match self.check_flag(flags::Flags::NativeModeFlag) {
					Ok(true)  => Ok(self.processor_status & flags::Flags::AccumulatorRegisterSizeFlag == 1_u16),
					Ok(false) => Err("Cannot check accumulator register size flag when not CPU is in emulator mode."),
					Err(_)    => unreachable!(),
				}
			},
			flags::Flags::ProgramBreakInterruptFlag => {
				match self.check_flag(flags::Flags::NativeModeFlag) {
					Ok(true)  => Err("Cannot check program break interrupt flag when in native mode."),
					Ok(false) => Ok(self.processor_status & flags::Flags::IndexRegisterSizeFlag == 1_u16),
					Err(_)    => unreachable!(),
				}
			},
			_ => Ok(self.processor_status & mask == 1_u16),
		}
	}

	fn set_flag(&mut self, flag: flags::Flags, do_clear: bool) {
		if do_clear {
			self.processor_status = self.processor_status ^ flag
		} else {
			self.processor_status = self.processor_status | flag
		}
	}

	fn add_with_carry_direct_page_indexed_indirect_x(&mut self, memory: &mut mem::Memory) {
		let address = self.direct_page + self.index_x;

		unsafe {
			let (result, overflow) = intrinsics::u16_add_with_overflow(memory.get_word_from_ram(address as usize), 1);

			self.set_flag(flags::Flags::NegativeFlag, result & 0x8000 == 0x8000);
			self.set_flag(flags::Flags::OverflowFlag, overflow);
			self.set_flag(flags::Flags::ZeroFlag, result == 0);
			self.set_flag(flags::Flags::CarryFlag, overflow);

			self.accumulator = result as u16;
		}
	}
}

