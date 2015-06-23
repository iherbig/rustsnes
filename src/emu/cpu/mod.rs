use std::intrinsics;

use emu::memory as mem;

mod flags;
use self::flags::Flags::*;

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
		// opcode <- mem[self.PC]
		// cycles <- function_table[opcode]() //this will update PC
		// return cycles
		
		//anything else?

		1_u8
	}

	fn check_flag(&self, mask: flags::Flags) -> bool {
		match mask {
			IndexRegisterSizeFlag => {
				match self.check_flag(NativeModeFlag) {
					true  => self.processor_status & IndexRegisterSizeFlag == 1_u16,
					_     => false,
				}
			},
			AccumulatorRegisterSizeFlag => {
				match self.check_flag(NativeModeFlag) {
					true  => self.processor_status & AccumulatorRegisterSizeFlag == 1_u16,
					_     => false,
				}
			},
			ProgramBreakInterruptFlag => {
				match self.check_flag(NativeModeFlag) {
					true  => false,
					_     => self.processor_status & IndexRegisterSizeFlag == 1_u16,
				}
			},
			_ => self.processor_status & mask == 1_u16,
		}
	}

	fn set_flag(&mut self, flag: flags::Flags, do_clear: bool) {
		if do_clear {
			self.processor_status = self.processor_status ^ flag
		} else {
			self.processor_status = self.processor_status | flag
		}
	}

	// start instructions
    // TODO: Look up addressing modes for the 65816: wiki.superfamicom.org has a lot of details
    // missing

    // something's missing here
    // this instruction is a two-byte instruction
    // the opcode uses up one of the bytes, where's the other?
	fn add_with_carry_direct_page_indexed_indirect_x(&mut self, memory: &mut mem::Memory) -> u8 {
		let address    = self.direct_page + self.index_x;
        let cycles: u8 = 6
                           + (if self.check_flag(AccumulatorRegisterSizeFlag) { 1 } else { 0 }) // is this right? what if the CPU is in emulator mode?
                           + (if ((self.direct_page & 0x000F) as u8) == 0 { 0 } else { 1 });

		unsafe {
			let (result, overflow) = intrinsics::u16_add_with_overflow(memory.get_word_from_ram(address as usize), 1);

			self.set_flag(NegativeFlag, result & 0x8000 == 0x8000);
			self.set_flag(OverflowFlag, overflow);
			self.set_flag(ZeroFlag, result == 0);
			self.set_flag(CarryFlag, overflow);

			self.accumulator = result as u16;
		}

        self.program_counter += 2;

        cycles
	}
}

