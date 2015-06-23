use core::ops::{BitAnd, BitXor, BitOr};

pub enum Flags {
	CarryFlag,
	ZeroFlag,
	IRQDisableFlag,
	DecimalFlag,
	IndexRegisterSizeFlag,
	AccumulatorRegisterSizeFlag,
	OverflowFlag,
	NegativeFlag,
	NativeModeFlag,
	ProgramBreakInterruptFlag,
}

impl BitAnd<Flags> for u16 {
	type Output = u16;

	fn bitand(self, rhs: Flags) -> u16 {
		match rhs {
			Flags::CarryFlag		    	=> self & 0x0001,
			Flags::ZeroFlag 		    	=> self & 0x0002,
			Flags::IRQDisableFlag	 	    	=> self & 0x0004,
			Flags::DecimalFlag	 	    	=> self & 0x0008,
			Flags::IndexRegisterSizeFlag	    	=> self & 0x0010,
			Flags::AccumulatorRegisterSizeFlag	=> self & 0x0020,
			Flags::OverflowFlag			=> self & 0x0040,
			Flags::NegativeFlag			=> self & 0x0080,
			Flags::NativeModeFlag			=> self & 0x0100,
			Flags::ProgramBreakInterruptFlag	=> self & 0x0010,
		}
	}
}

impl BitXor<Flags> for u16 {
	type Output = u16;

	fn bitxor(self, rhs: Flags) -> u16 {
		match rhs {
			Flags::CarryFlag		    	=> self ^ 0x0001,
			Flags::ZeroFlag 		    	=> self ^ 0x0002,
			Flags::IRQDisableFlag	 	    	=> self ^ 0x0004,
			Flags::DecimalFlag	 	    	=> self ^ 0x0008,
			Flags::IndexRegisterSizeFlag	    	=> self ^ 0x0010,
			Flags::AccumulatorRegisterSizeFlag	=> self ^ 0x0020,
			Flags::OverflowFlag			=> self ^ 0x0040,
			Flags::NegativeFlag			=> self ^ 0x0080,
			Flags::NativeModeFlag			=> self ^ 0x0100,
			Flags::ProgramBreakInterruptFlag	=> self & 0x0010,
		}
	}
}

impl BitOr<Flags> for u16 {
	type Output = u16;

	fn bitor(self, rhs: Flags) -> u16 {
		match rhs {
			Flags::CarryFlag		    	=> self | 0x0001,
			Flags::ZeroFlag 		    	=> self | 0x0002,
			Flags::IRQDisableFlag	 	    	=> self | 0x0004,
			Flags::DecimalFlag	 	    	=> self | 0x0008,
			Flags::IndexRegisterSizeFlag	    	=> self | 0x0010,
			Flags::AccumulatorRegisterSizeFlag	=> self | 0x0020,
			Flags::OverflowFlag			=> self | 0x0040,
			Flags::NegativeFlag			=> self | 0x0080,
			Flags::NativeModeFlag			=> self | 0x0100,
			Flags::ProgramBreakInterruptFlag	=> self & 0x0010,
		}
	}
}

