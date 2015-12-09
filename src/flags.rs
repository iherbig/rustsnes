use std::ops::{BitAnd, BitOr, Not};

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

impl Not for Flags {
    type Output = u8;

    fn not(self) -> u8 {
        match self {
            Flags::CarryFlag		    	    => 0b11111110,
			Flags::ZeroFlag 		    	    => 0b11111101,
			Flags::IRQDisableFlag	 	    	=> 0b11111011,
			Flags::DecimalFlag	 	    	    => 0b11110111,
			Flags::IndexRegisterSizeFlag	    => 0b11101111,
			Flags::AccumulatorRegisterSizeFlag	=> 0b11011111,
			Flags::OverflowFlag			        => 0b10111111,
			Flags::NegativeFlag			        => 0b01111111,
            Flags::ProgramBreakInterruptFlag    => 0b11101111,
            _                                   => unimplemented!(),
        }
    }
}

impl BitAnd<Flags> for u8 {
    type Output = u8;
    
    fn bitand(self, rhs: Flags) -> u8 {
        match rhs {
            Flags::CarryFlag                    => self & 0x01,
            Flags::ZeroFlag                     => self & 0x02,
            Flags::IRQDisableFlag               => self & 0x04,
            Flags::DecimalFlag                  => self & 0x08,
            Flags::IndexRegisterSizeFlag        => self & 0x10,
            Flags::AccumulatorRegisterSizeFlag  => self & 0x20,
            Flags::OverflowFlag                 => self & 0x40,
            Flags::NegativeFlag                 => self & 0x80,
            Flags::ProgramBreakInterruptFlag    => self & 0x10,
            _                                   => unimplemented!(),
        }
    }
}

impl BitOr<Flags> for u8 {
    type Output = u8;

    fn bitor(self, rhs: Flags) -> u8 {
        match rhs {
            Flags::CarryFlag                    => self | 0x01,
            Flags::ZeroFlag                     => self | 0x02,
            Flags::IRQDisableFlag               => self | 0x04,
            Flags::DecimalFlag                  => self | 0x08,
            Flags::IndexRegisterSizeFlag        => self | 0x10,
            Flags::AccumulatorRegisterSizeFlag  => self | 0x20,
            Flags::OverflowFlag                 => self | 0x40,
            Flags::NegativeFlag                 => self | 0x80,
            Flags::ProgramBreakInterruptFlag    => self | 0x10,
            _                                   => unimplemented!(),
        }
    }
}
