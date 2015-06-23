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
                           + (if self.check_flag(AccumulatorRegisterSizeFlag) { 0 } else { 1 }) // is this right? what if the CPU is in emulator mode?
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
	
	fn add_with_carry_stack_relative(&mut self, memory: &mut mem::Memory) -> u8 {
		4
		
		//Sets NVZC
	}
	
	fn add_with_carry_direct_page(&mut self, memory: &mut mem::Memory) -> u8 {
		3
		
		//Sets NVZC
	}
	
	fn add_with_carry_direct_page_indirect_long(&mut self, memory: &mut mem::Memory) -> u8 {
		6
		
		//Sets NVZC
	}
	
	fn add_with_carry_immediate(&mut self, memory: &mut mem::Memory) -> u8 {
		2
		
		//Sets NVZC
	}
	
	fn add_with_carry_absolute(&mut self, memory: &mut mem::Memory) -> u8 {
		4
		
		//Sets NVZC
	}
	
	fn add_with_carry_absolute_long(&mut self, memory: &mut mem::Memory) -> u8 {
		5
		
		//Sets NVZC
	}
	
	fn add_with_carry_direct_page_indexed_indirect_y(&mut self, memory: &mut mem::Memory) -> u8 {
		let cycles: u8 = 5 + (if self.check_flag(AccumulatorRegisterSizeFlag) { 0 } else { 2 });
		
		//Sets NVZC
		
		cycles
	}
	
	fn add_with_carry_direct_page_indirect(&mut self, memory: &mut mem::Memory) -> u8 {
		5
		
		//Sets NVZC
	}
	
	fn add_with_carry_stack_relative_indirect_indexed_y(&mut self, memory: &mut mem::Memory) -> u8 {
		7
		
		//Sets NVZC
	}
	
	fn add_with_carry_direct_page_indexed_x(&mut self, memory: &mut mem::Memory) -> u8 {
		4
		
		//Sets NVZC
	}
	
	fn add_with_carry_direct_page_indirect_long_indexed_y(&mut self, memory: &mut mem::Memory) -> u8 {
		6
		
		//Sets NVZC
	}
	
	fn add_with_carry_absolute_indexed_y(&mut self, memory: &mut mem::Memory) -> u8 {
		4
		
		//Sets NVZC
	}
	
	fn add_with_carry_absolute_indexed_x(&mut self, memory: &mut mem::Memory) -> u8 {
		4
		
		//Sets NVZC
	}
	
	fn add_with_carry_absolute_long_indexed_x(&mut self, memory: &mut mem::Memory) -> u8 {
		5
		
		//Sets NVZC
	}
	
	fn and_accumulator_with_memory_direct_page_indexed_indirect_x(&mut self, memory: &mut mem::Memory) -> u8 {
		6
		
		//Sets NZ
	}
	
	fn and_accumulator_with_memory_stack_relative(&mut self, memory: &mut mem::Memory) -> u8 {
		4
		
		//Sets NZ
	}
	
	fn and_accumulator_with_memory_direct_page(&mut self, memory: &mut mem::Memory) -> u8 {
		3
		
		//Sets NZ
	}
	
	fn and_accumulator_with_memory_direct_page_indirect_long(&mut self, memory: &mut mem::Memory) -> u8 {
		6
		
		//Sets NZ
	}
	
	fn and_accumulator_with_memory_immediate(&mut self, memory: &mut mem::Memory) -> u8 {
		2
		
		//Sets NZ
	}
	
	fn and_accumulator_with_memory_absolute(&mut self, memory: &mut mem::Memory) -> u8 {
		4
		
		//Sets NZ
	}
	
	fn and_accumulator_with_memory_absolute_long(&mut self, memory: &mut mem::Memory) -> u8 {
		5
		
		//Sets NZ
	}
	
	fn and_accumulator_with_memory_direct_page_indirect_indexed_y(&mut self, memory: &mut mem::Memory) -> u8 {
		5
		
		//Sets NZ
	}
	
	fn and_accumulator_with_memory_direct_page_indirect(&mut self, memory: &mut mem::Memory) -> u8 {
		5
		
		//Sets NZ
	}
	
	fn and_accumulator_with_memory_stack_relative_indirect_indexed_y(&mut self, memory: &mut mem::Memory) -> u8 {
		7
		
		//Sets NZ
	}
	
	fn and_accumulator_with_memory_direct_page_indexed_x(&mut self, memory: &mut mem::Memory) -> u8 {
		4
		
		//Sets NZ
	}
	
	fn and_accumulator_with_memory_direct_page_indirect_long_indexed_y(&mut self, memory: &mut mem::Memory) -> u8 {
		6
		
		//Sets NZ
	}
	
	fn and_accumulator_with_memory_absolute_indexed_y(&mut self, memory: &mut mem::Memory) -> u8 {
		4
		
		//Sets NZ
	}
	
	fn and_accumulator_with_memory_absolute_indexed_x(&mut self, memory: &mut mem::Memory) -> u8 {
		4
		
		//Sets NZ
	}
	
	fn and_accumulator_with_memory_absolute_long_indexed_x(&mut self, memory: &mut mem::Memory) -> u8 {
		5
		
		//Sets NZ
	}
	
	fn arithmetic_shift_left_direct_page(&mut self, memory: &mut mem::Memory) -> u8 {
		let cycles = 5 + (if self.check_flag(AccumulatorRegisterSizeFlag) { 0 } else { 2 });
		
		//Sets NZC
		
		cycles
	}
	
	fn arithmetic_shift_left_accumulator(&mut self, memory: &mut mem::Memory) -> u8 {
		2
		
		//Sets NZC
	}
	
	fn arithmetic_shift_left_absolute(&mut self, memory: &mut mem::Memory) -> u8 {
		6
		
		//Sets NZC
	}
	
	fn arithmetic_shift_left_direct_page_indexed_x(&mut self, memory: &mut mem::Memory) -> u8 {
		6
		
		//Sets NZC
	}
	
	fn arithmetic_shift_left_absolute_indexed_x(&mut self, memory: &mut mem::Memory) -> u8 {
		7
		
		//Sets NZC
	}
	
	fn branch_if_carry_clear(&mut self) -> u8 {
		//TODO: Add 1 cycle if branch taken crosses page boundary in emulation mode
		let cycles = 2 + if !self.check_flag(CarryFlag) { 1 } else { 0 };
		
		cycles
	}
	
	fn branch_if_carry_set(&mut self) -> u8 {
		2
	}
	
	fn branch_if_equal(&mut self) -> u8 {
		2
	}
	
	fn test_bits_direct_page(&mut self, memory: &mut mem::Memory) -> u8 {
		3
		
		//Sets NVZ
	}
	
	fn test_bits_absolute(&mut self, memory: &mut mem::Memory) -> u8 {
		4
		
		//Sets NVZ
	}
	
	fn test_bits_direct_page_indexed_x(&mut self, memory: &mut mem::Memory) -> u8 {
		4
		
		//Sets NVZ
	}
	
	fn test_bits_absolute_indexed_x(&mut self, memory: &mut mem::Memory) -> u8 {
		4
		
		//Sets NVZ
	}
	
	fn test_bits_immediate(&mut self, memory: &mut mem::Memory) -> u8 {
		2
		
		//Sets Z
	}
	
	fn branch_if_minus(&mut self) -> u8 {
		2
	}
	
	fn branch_if_not_equal(&mut self) -> u8 {
		2
	}
	
	fn branch_if_plus(&mut self) -> u8 {
		2
	}
	
	fn branch_always(&mut self) -> u8 {
		3
	}
	
	fn break_op(&mut self) -> u8 {
		//TODO: Add 1 cycle if in native mode
		let cycles = 7;
		
		//Sets DI
		
		cycles
	}
	
	fn branch_long_always(&mut self) -> u8 {
		4
	}
	
	fn branch_if_overflow_clear(&mut self) -> u8 {
		2
	}
	
	fn branch_if_overflow_set(&mut self) -> u8 {
		2
	}
	
	fn clear_carry(&mut self) -> u8 {
		self.set_flag(CarryFlag, false);
		
		2
	}
	
	fn clear_decimal_mode_flag(&mut self) -> u8 {
		self.set_flag(DecimalFlag, false);
		
		2
	}
	
	fn clear_interrupt_disable_flag(&mut self) -> u8 {
		self.set_flag(IRQDisableFlag, false);
		
		2
	}
	
	fn clear_overflow_flag(&mut self) -> u8 {
		self.set_flag(OverflowFlag, false);
		
		2
	}
	
	fn compare_accumulator_with_memory_direct_page_indexed_indirect_x(&mut self, memory: &mut mem::Memory) -> u8 {
		6
		
		//Sets: NZC
	}
	
	fn compare_accumulator_with_memory_stack_relative(&mut self, memory: &mut mem::Memory) -> u8 {
		4
		
		//Sets: NZC
	}
	
	fn compare_accumulator_with_memory_direct_page(&mut self, memory: &mut mem::Memory) -> u8 {
		3
		
		//Sets: NZC
	}
	
	fn compare_accumulator_with_memory_direct_page_indirect_long(&mut self, memory: &mut mem::Memory) -> u8 {
		6
		
		//Sets: NZC
	}
	
	fn compare_accumulator_with_immediate(&mut self) -> u8 {
		2
		
		//Sets: NZC
	}
	
	fn compare_accumulator_with_memory_absolute(&mut self, memory: &mut mem::Memory) -> u8 {
		4
		
		//Sets: NZC
	}
	
	fn compare_accumulator_with_memory_absolute_long(&mut self, memory: &mut mem::Memory) -> u8 {
		5
		
		//Sets: NZC
	}
	
	fn compare_accumulator_with_memory_direct_page_indirect_indexed_y(&mut self, memory: &mut mem::Memory) -> u8 {
		5
		
		//Sets: NZC
	}
	
	fn compare_accumulator_with_memory_direct_page_indirect(&mut self, memory: &mut mem::Memory) -> u8 {
		5
		
		//Sets: NZC
	}
	
	fn compare_accumulator_with_memory_stack_relative_indirect_indexed_y(&mut self, memory: &mut mem::Memory) -> u8 {
		7
		
		//Sets: NZC
	}
	
	fn compare_accumulator_with_memory_direct_page_indexed_x(&mut self, memory: &mut mem::Memory) -> u8 {
		4
		
		//Sets: NZC
	}
	
	fn compare_accumulator_with_memory_direct_page_indirect_long_indexed_y(&mut self, memory: &mut mem::Memory) -> u8 {
		6
		
		//Sets: NZC
	}
	
	fn compare_accumulator_with_memory_absolute_indexed_y(&mut self, memory: &mut mem::Memory) -> u8 {
		4
		
		//Sets: NZC
	}
	
	fn compare_accumulator_with_memory_absolute_indexed_x(&mut self, memory: &mut mem::Memory) -> u8 {
		4
		
		//Sets: NZC
	}
	
	fn compare_accumulator_with_memory_absolute_long_indexed_x(&mut self, memory: &mut mem::Memory) -> u8 {
		5
		
		//Sets: NZC
	}
	
	fn coprocessor_enable(&mut self) -> u8 {
		7
		
		//Sets: DI
	}
	
	fn compare_index_register_x_with_memory_immediate(&mut self, memory: &mut mem::Memory) -> u8 {
		let cycles = 2 + (if self.check_flag(IndexRegisterSizeFlag) { 0 } else { 1 });
		
		//Sets: NZC
		
		cycles
	}
	
	fn compare_index_register_x_with_memory_direct_page(&mut self, memory: &mut mem::Memory) -> u8 {
		3
		
		//Sets: NZC
	}
	
	fn compare_index_register_x_with_memory_absolute(&mut self, memory: &mut mem::Memory) -> u8 {
		4
		
		//Sets: NZC
	}
	
	fn compare_index_register_y_with_memory_immediate(&mut self, memory: &mut mem::Memory) -> u8 {
		2
		
		//Sets: NZC
	}
	
	fn compare_index_register_y_with_memory_direct_page(&mut self, memory: &mut mem::Memory) -> u8 {
		3
		
		//Sets: NZC
	}
	
	fn compare_index_register_y_with_memory_absolute(&mut self, memory: &mut mem::Memory) -> u8 {
		4
		
		//Sets: NZC
	}
	
	fn decrement_accumulator(&mut self) -> u8 {
		2
		
		//Sets: NZ
	}
	
	fn decrement_direct_page(&mut self) -> u8 {
		5
		
		//Sets: NZ
	}
	
	fn decrement_absolute(&mut self, memory: &mut mem::Memory) -> u8 {
		6
		
		//Sets: NZ
	}
	
	fn decrement_direct_page_indexed_x(&mut self, memory: &mut mem::Memory) -> u8 {
		6
		
		//Sets: NZ
	}
	
	fn decrement_absolute_indexed_x(&mut self, memory: &mut mem::Memory) -> u8 {
		7
		
		//Sets: NZ
	}
	
	fn decrement_index_register_x(&mut self) -> u8 {
		2
		
		//Sets: NZ
	}
	
	fn decrement_index_register_y(&mut self) -> u8 {
		2
		
		//Sets: NZ
	}
}
