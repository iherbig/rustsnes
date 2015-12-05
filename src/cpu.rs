use std::intrinsics;

use memory::Memory;
use flags;
use flags::Flags::*;

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
    pub memory:    Memory,
}

impl CPU {
	pub fn new(memory: Memory) -> CPU {
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
            memory:      memory,
		}
	}
	
	pub fn execute(&self, addr: usize) {
		// opcode <- mem[self.PC]
		// cycles <- instruction_table[opcode]() //this will update PC
		// return cycles
		
		//anything else?
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
	//TODO: Read "Programming the 65816"
}

trait Instruction {
    fn load(&self, cpu: &mut CPU, mode: AddressingMode) -> u8;
    fn store(&self, cpu: &mut CPU, mode: AddressingMode, data: u8);
}

pub enum OpCode {
    BRK, //00 Stack/Interrupt 2** 7(9)
    ORA, //01 DP Indexed Indirect, X 2 6(1,2)
    COP, //02 Stack/Interrupt 2** 7(9)
    //ORA, //03 Stack Relative 2 4(1)
    TSB, //04 Direct Page 2 5(2,5)
    //ORA, //05 Direct Page 2 3(1,2)
    ASL, //06 Direct Page 2 5(2,5)
    //ORA, //07 DP Indirect Long 2 6(1,2)
    PHP, //08 Stack (Push) 1 3
    //ORA, //09 Immediate 2* 2(1)
    //ASL, //0A Accumulator 1 2
    PHD, //0B Stack (Push) 1 4
    //TSB, //0C Absolute 3 6(5)
    //ORA, //0D Absolute 3 4(1)
    //ASL, //0E Absolute 3 6(5)
    //ORA, //0F Absolute Long 4 5(1)
    BLP, //10 Program Counter Relative 2 2(7,8)
    //ORA, //11 DP Indirect Indexed, Y 2 5(1,2,3)
    //ORA, //12 DP Indirect 2 5(1,2)
    //ORA, //13 SR Indirect Indexed, Y 2 7(1)
    TRB, //14 Direct Page 2 5(2,5)
    //ORA, //15 DP Indexed, X 2 4(1,2)
    //ASL, //16 DP Indexed, X 2 6(2,5)
    //ORA, //17 DP Indirect Long Indexed, Y 2 6(1,2)
    CLC, //18 Implied 1 2
    //ORA, //19 Absolute Indexed, Y 3 4(1,3)
    INC, //1A Accumulator 1 2
    TCS, //1B Implied 1 2
    //TRB, //1C Absolute 3 6(5)
    //ORA, //1D Absolute Indexed, X 3 4(1,3)
    //ASL, //1E Absolute Indexed, X 3 7(5,6)
    //ORA, //1F Absolute Long Indexed, X 4 5(1)
    JSR, //20 Absolute 3 6
    AND, //21 DP Indexed Indirect, X 2 6(1,2)
    //JSR, //22 Absolute Long 4 8
    //AND, //23 Stack Relative 2 4(1)
    BIT, //24 Direct Page 2 3(1,2)
    //AND, //25 Direct Page 2 3(1,2)
    ROL, //26 Direct Page 2 5(2,5)
    //AND, //27 DP Indirect Long 2 6(1,2)
    PLP, //28 Stack (Pull) 1 4
    //AND, //29 Immediate 2* 2(1)
    //ROL, //2A Accumulator 1 2
    PLD, //2B Stack (Pull) 1 5
    //BIT, //2C Absolute 3 4(1)
    //AND, //2D Absolute 3 4(1)
    //ROL, //2E Absolute 3 6(5)
    //AND, //2F Absolute Long 4 5(1)
    BMI, //30 Program Counter Relative 2 2(7,8)
    //AND, //31 DP Indirect Indexed, Y 2 5(1,2,3)
    //AND, //32 DP Indirect 2 5(1,2)
    //AND, //33 SR Indirect Indexed, Y 2 7(1)
    //BIT, //34 DP Indexed, X 2 4(1,2)
    //AND, //35 DP Indexed, X 2 4(1,2)
    //ROL, //36 DP Indexed, X 2 6(2,5)
    //AND, //37 DP Indirect Long Indexed, Y 2 6(1,2)
    SEC, //38 Implied 1 2
    //AND, //39 Absolute Indexed, Y 3 4(1,3)
    DEC, //3A Accumulator 1 2
    TSC, //3B Implied 1 2
    //BIT, //3C Absolute Indexed, X 3 4(1,3)
    //AND, //3D Absolute Indexed, X 3 4(1,3)
    //ROL, //3E Absolute Indexed, X 3 7(5,6)
    //AND, //3F Absolute Long Indexed, X 4 5(1)
    RTI, //40 Stack/RTI 1 6(9)
    EOR, //41 DP Indexed Indirect, X 2 6(1,2)
    WDM, //42 2^16 (16)
    //EOR, //43 Stack Relative 2 4(1)
    MVP, //44 Block Move 3(13)
    //EOR, //45 Direct Page 2 3(1,2)
    LSR, //46 Direct Page 2 5(2,5)
    //EOR, //47 DP Indirect Long 2 6(1,2)
    PHA, //48 Stack (Push) 1 3(1)
    //EOR, //49 Immediate 2* 2(1)
    //LSR, //4A Accumulator 1 2
    PHK, //4B Stack (Push) 1 3
    JMP, //4C Absolute 3 3
    //EOR, //4D Absolute 3 4(1)
    //LSR, //4E Absolute 3 6(5)
    //EOR, //4F Absolute Long 4 5(1)
    BVC, //50 Program Counter Relative 2 2(7,8)
    //EOR, //51 DP Indirect Indexed, Y 2 5(1,2,3,)
    //EOR, //52 DP Indirect 2 5(1,2)
    //EOR, //53 SR Indirect Indexed, Y 2 7(1)
    MVN, //54 Block Move 3(13)
    //EOR, //55 DP Indexed, X 2 4(1,2)
    //LSR, //56 DP Indexed, X 2 6(2,5)
    //EOR, //57 DP Indirect Long Indexed, Y 2 6(1,2)
    CLI, //58 Implied 1 2
    //EOR, //59 Absolute Indexed, Y 3 4(1,3)
    PHY, //5A Stack (Push) 1 3(10)
    TCD, //5B Implied 1 2
    //JMP, //5C Absolute Long 4 4
    //EOR, //5D Absolute Indexed, X 3 4(1,3)
    //LSR, //5E Absolute Indexed, X 3 7(5,6)
    //EOR, //5F Absolute Long Indexed, X 4 5(1)
    RTS, //60 Stack (RTS) 1 6
    ADC, //61 DP Indexed Indirect, X 2 6(1,2,4)
    PER, //62 Stack (PC Relative Long) 3 6
    //ADC, //63 Stack Relative 2 4(1,4)
    STZ, //64 Direct Page 2 3(1,2)
    //ADC, //65 Direct Page 2 3(1,2,4)
    ROR, //66 Direct Page 2 5(2,5)
    //ADC, //67 DP Indirect Long 2 6(1,2,4)
    PLA, //68 Stack (Pull) 1 4(1)
    //ADC, //69 Immediate 2* 2(1,4)
    //ROR, //6A Accumulator 1 2
    RTL, //6B Stack (RTL) 1 6
    //JMP, //6C Absolute Indirect 3 5(11,12)
    //ADC, //6D Absolute 3 4(1,4)
    //ROR, //6E Absolute 3 6(5)
    //ADC, //6F Absolute Long 4 5(1,4)
    BVS, //70 Program Counter Relative 2 2(7,8)
    //ADC, //71 DP Indirect Indexed, Y 2 5(1,2,3,4)
    //ADC, //72 DP Indirect 2 5(1,2,4)
    //ADC, //73 SR Indirect Indexed, Y 2 7(1,4)
    //STZ, //74 Direct Page Indexed, X 2 4(1,2)
    //ADC, //75 DP Indexed, X 2 4(1,2,4)
    //ROR, //76 DP Indexed, X 2 6(2,5)
    //ADC, //77 DP Indirect Long Indexed, Y 2 6(1,2,4)
    SEI, //78 Implied 1 2
    //ADC, //79 Absolute Indexed, Y 3 4(1,3,4)
    PLY, //7A Stack/Pull 1 4(10)
    TDC, //7B Implied 1 2
    //JMP, //7C Absolute Indexed Indirect 3 6
    //ADC, //7D Absolute Indexed, X 3 4(1,3,4)
    //ROR, //7E Absolute Indexed, X 3 7(5,6)
    //ADC, //7F Absolute Long Indexed, X 4 5(1,4)
    BRA, //80 Program Counter Relative 2 3(8)
    STA, //81 DP Indexed Indirect, X 2 6(1,2)
    BRL, //82 Program Counter Relative Long 3 4
    //STA, //83 Stack Relative 2 4(1)
    STY, //84 Direct Page 2 3(2,10)
    //STA, //85 Direct Page 2 3(1,2)
    STX, //86 Direct Page 2 3(2,10)
    //STA, //87 DP Indirect Long 2 6(1,2)
    DEY, //88 Implied 1 2
    //BIT, //89 Immediate 2* 2(1)
    TXA, //8A Implied 1 2
    PHB, //8B Stack (Push) 1 3
    //STY, //8C Absolute 3 4(10)
    //STA, //8D Absolute 3 4(1)
    //STX, //8E Absolute 3 4(10)
    //STA, //8F Absolute Long 4 5(1)
    BCC, //90 Program Counter Relative 2 2(7,8)
    //STA, //91 DP Indirect Indexed, Y 2 6(1,2)
    //STA, //92 DP Indirect 2 5(1,2)
    //STA, //93 SR Indirect Indexed, Y 2 7(1)
    //STY, //94 Direct Page Indexed, X 2 4(2,10)
    //STA, //95 DP Indexed, X 2 4(1,2)
    //STX, //96 Direct Page Indexed, Y 2 4(2,10)
    //STA, //97 DP Indirect Long Indexed, Y 2 6(1,2)
    TYA, //98 Implied 1 2
    //STA, //99 Absolute Indexed, Y 3 5(1)
    TXS, //9A Implied 1 2
    TXY, //9B Implied 1 2
    //STZ, //9C Absolute 3 4(1)
    //STA, //9D Absolute Indexed, X 3 5(1)
    //STZ, //9E Absolute Indexed, X 3 5(1)
    //STA, //9F Absolute Long Indexed, X 4 5(1)
    LDY, //A0 Immediate 2+ 2(10)
    LDA, //A1 DP Indexed Indirect, X 2 6(1,2)
    LDX, //A2 Immediate 2+ 2(10)
    //LDA, //A3 Stack Relative 2 4(1)
    //LDY, //A4 Direct Page 2 3(2,10)
    //LDA, //A5 Direct Page 2 3(1,2)
    //LDX, //A6 Direct Page 2 3(2,10)
    //LDA, //A7 DP Indirect Long 2 6(1,2)
    TAY, //A8 Implied 1 2
    //LDA, //A9 Immediate 2* 2(1)
    TAX, //AA Implied 1 2
    PLB, //AB Stack (Pull) 1 4
    //LDY, //AC Absolute 3 4(10)
    //LDA, //AD Absolute 3 4(1)
    //LDX, //AE Absolute 3 4(10)
    //LDA, //AF Absolute Long 4 5(1)
    BCS, //B0 Program Counter Relative 2 2(7,8)
    //LDA, //B1 DP Indirect Indexed, Y 2 5(1,2,3)
    //LDA, //B2 DP Indirect 2 5(1,2)
    //LDA, //B3 SR Indirect Indexed, Y 2 7(1)
    //LDY, //B4 DP Indexed, X 2 4(2,10)
    //LDA, //B5 DP Indexed, X 2 4(1,2)
    //LDX, //B6 DP Indexed, Y 2 4(2,10)
    //LDA, //B7 DP Indirect Long Indexed, Y 2 6(1,2)
    CLV, //B8 Implied 1 2
    //LDA, //B9 Absolute Indexed, Y 3 4(1,3)
    TSX, //BA Implied 1 2
    TYX, //BB Implied 1 2
    //LDY, //BC Absolute Indexed, X 3 4(3,10)
    //LDA, //BD Absolute Indexed, X 3 4(1,3)
    //LDX, //BE Absolute Indexed, Y 3 4(3,10)
    //LDA, //BF Absolute Long Indexed, X 4 5(1)
    CPY, //C0 Immediate 2+ 2(10)
    CMP, //C1 DP Indexed Indirect, X 2 6(1,2)
    REP, //C2 Immediate 2 3
    //CMP, //C3 Stack Relative 2 4(1)
    //CPY, //C4 Direct Page 2 3(2,10)
    //CMP, //C5 Direct Page 2 3(1,2)
    //DEC, //C6 Direct Page 2 5(2,5)
    //CMP, //C7 DP Indirect Long 2 6(1,2)
    INY, //C8 Implied 1 2
    //CMP, //C9 Immediate 2* 2(1)
    DEX, //CA Implied 1 2
    WAI, //CB Implied 1 3(15)
    //CPY, //CC Absolute 3 4(10)
    //CMP, //CD Absolute 3 4(1)
    //DEC, //CE Absolute 3 6(5)
    //CMP, //CF Absolute Long 4 5(1)
    BNE, //D0 Program Counter Relative 2 2(7,8)
    //CMP, //D1 DP Indirect Indexed, Y 2 5(1,2,3)
    //CMP, //D2 DP Indirect 2 5(1,2)
    //CMP, //D3 SR Indirect Indexed, Y 2 7(1)
    PEI, //D4 Stack (Direct Page Indirect) 2 6(2)
    //CMP, //D5 DP Indexed, X 2 4(1,2)
    //DEC, //D6 DP Indexed, X 2 6(2,5)
    //CMP, //D7 DP Indirect Long Indexed, Y 2 6(1,2)
    CLD, //D8 Implied 1 2
    //CMP, //D9 Absolute Indexed, Y 3 4(1,3)
    PHX, //DA Stack (Push) 1 3(10)
    STP, //DB Implied 1 3(14)
    //JMP, //DC Absolute Indirect Long 3 6
    //CMP, //DD Absolute Indexed, X 3 41,3
    //DEC, //DE Absolute Indexed, X 3 7(5,6)
    //CMP, //DF Absolute Long Indexed, X 4 5(1)
    CPX, //E0 Immediate 2+ 2(10)
    SBC, //E1 DP Indexed Indirect, X 2 6(1,2,4)
    //CPX, //E2 Immediate 2 3
    //SBC, //E3 Stack Relative 2 4(1,4)
    INX, //E4 Direct Page 2 3(2,10)
    //SBC, //E5 Direct Page 2 3(1,2,4)
    //INC, //E6 Direct Page 2 5(2,5)
    //SBC, //E7 DP Indirect Long 2 6(1,2,4)
    //INX, //E8 Implied 1 2
    //SBC, //E9 Immediate 2* 2(1,4)
    NOP, //EA Implied 1 2
    XBA, //EB Implied 1 3
    //CPX, //EC Absolute 3 4(10)
    //SBC, //ED Absolute 3 4(1,4)
    //INC, //EE Absolute 3 6(5)
    //SBC, //EF Absolute Long 4 5(1,4)
    BEQ, //F0 Program Counter Relative 2 2(7,8)
    //SBC, //F1 DP Indirect Indexed, Y 2 5(1,2,3,4)
    //SBC, //F2 DP Indirect 2 5(1,2,4)
    //SBC, //F3 SR Indirect Indexed, Y 2 7(1,4)
    PEA, //F4 Stack (absolute) 3 5
    //SBC, //F5 DP Indexed, X 2 4(1,2,4)
    //INC, //F6 DP Indexed, X 2 6(2,5)
    //SBC, //F7 DP Indirect Long Indexed, Y 2 6(1,2,4)
    SED, //F8 Implied 1 2
    //SBC, //F9 Absolute Indexed, Y 3 4(1,3,4)
    PLX, //FA Stack /Pull 1 4(10)
    XCE, //FB Implied 1 2
    //JSR, //FC Absolute Indexed Indirect 3 8
    //SBC, //FD Absolute Indexed, X 3 4(1,3,4)
    //INC, //FE Absolute Indexed, X 3 7(5,6)
    //SBC, //FF Absolute Long Indexed, X 4 5(1,4)
}

impl Instruction for OpCode {
    fn load(&self, cpu: &mut CPU, mode: AddressingMode) -> u8 {
        1_u8
    }

    fn store(&self, cpu: &mut CPU, mode: AddressingMode, data: u8) {
    }
}

enum AddressingMode {
    Absolute,
    AbsoluteIndexedX,
    AbsoluteIndexedY,
    AbsoluteIndexedIndirect,
    AbsoluteIndirect,
    AbsoluteIndirectLong,
    AbsoluteLong,
    AbsoluteLongIndexedX,
    Accumulator,
    BlockMove,
    DirectPage,
    DirectPageIndexedX,
    DirectPageIndexedY,
    DirectPageIndexedIndirectX,
    DirectPageIndirect,
    DirectPageIndirectLong,
    DirectPageIndirectIndexedY,
    DirectPageIndirectLongIndexedY,
    Immediate,
    Implied,
    ProgramCounterRelative,
    ProgramCounterRelativeLong,
    StackAbsolute,
    StackDirectPageIndirect,
    StackInterrupt,
    StackProgramCounterRelative,
    StackPull,
    StackPush,
    StackRTI,
    StackRTL,
    StackRTS,
    StackRelative,
    StackRelativeIndirectIndexedY,
}
