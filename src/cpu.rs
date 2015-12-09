use memory::Memory;
use flags::Flags;
use flags::Flags::*;

pub struct CPU {
    accumulator:      u16,
    index_x:          u16,
    index_y:          u16,
    stack_pointer:    u16,
    data_bank:         u8,
    direct_page:      u16,
    program_bank:      u8,
    processor_status:  u8,
    program_counter:  u16,
    emulation_mode:    u8,
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
            emulation_mode:   1,
            memory:      memory,
        }
	}

    pub fn set_origin(&mut self, addr: u16) {
        self.program_counter = addr;
    }

    pub fn set_acc(&mut self, value: u16) {
        self.accumulator = value;
    }

    pub fn check_acc(&self) -> u16 {
        self.accumulator
    }

    pub fn get_flags(&self) -> u8 {
        self.processor_status
    }
	
    pub fn execute(&mut self, num_instructions: usize) {
        for i in 0..num_instructions {
           //let opcode = self.memory.get_byte(self.program_counter as usize);
           let mode = Absolute;
           self.adc(&mode);
        }

        // decode opcode to get appropriate instruction function
        // execute instruction
		
		//anything else?
    }

	pub fn check_flag(&self, mask: Flags) -> bool {
		match mask {
            CarryFlag                   => self.processor_status & 0b00000001 == 1,
            ZeroFlag                    => (self.processor_status & 0b00000010) >> 1 == 1,
            IRQDisableFlag              => (self.processor_status & 0b00000100) >> 2 == 1,
            DecimalFlag                 => (self.processor_status & 0b00001000) >> 3 == 1,
            IndexRegisterSizeFlag       => (self.processor_status & 0b00010000) >> 4 == 1,
            AccumulatorRegisterSizeFlag => (self.processor_status & 0b00100000) >> 5 == 1,
            ProgramBreakInterruptFlag   => (self.processor_status & 0b00010000) >> 4 == 1,
            OverflowFlag                => (self.processor_status & 0b01000000) >> 6 == 1,
            NegativeFlag                => (self.processor_status & 0b10000000) >> 7 == 1,
            NativeModeFlag              => self.emulation_mode == 0,
		}
	}
	
	pub fn set_flag(&mut self, flag: Flags, val: bool) {
		if val {
			self.processor_status = self.processor_status | flag
		} else {
			self.processor_status = self.processor_status & !flag
		}
	}

    //BRK, //00 Stack/Interrupt 2** 7(9)
    fn brk(&mut self, mode: &Instruction) {
    }

    //ORA, //01 DP Indexed Indirect, X 2 6(1,2)
    //ORA, //03 Stack Relative 2 4(1)
    //ORA, //05 Direct Page 2 3(1,2)
    //ORA, //07 DP Indirect Long 2 6(1,2)
    //ORA, //09 Immediate 2* 2(1)
    //ORA, //0D Absolute 3 4(1)
    //ORA, //0F Absolute Long 4 5(1)
    //ORA, //11 DP Indirect Indexed, Y 2 5(1,2,3)
    //ORA, //12 DP Indirect 2 5(1,2)
    //ORA, //13 SR Indirect Indexed, Y 2 7(1)
    //ORA, //15 DP Indexed, X 2 4(1,2)
    //ORA, //17 DP Indirect Long Indexed, Y 2 6(1,2)
    //ORA, //19 Absolute Indexed, Y 3 4(1,3)
    //ORA, //1D Absolute Indexed, X 3 4(1,3)
    //ORA, //1F Absolute Long Indexed, X 4 5(1)
    fn ora(&mut self, mode: &Instruction) {
    }

    //COP, //02 Stack/Interrupt 2** 7(9)
    fn cop(&mut self, mode: &Instruction) {
    }

    //TSB, //04 Direct Page 2 5(2,5)
    //TSB, //0C Absolute 3 6(5)
    fn tsb(&mut self, mode: &Instruction) {
    }

    //ASL, //06 Direct Page 2 5(2,5)
    //ASL, //0A Accumulator 1 2
    //ASL, //0E Absolute 3 6(5)
    //ASL, //16 DP Indexed, X 2 6(2,5)
    //ASL, //1E Absolute Indexed, X 3 7(5,6)
    fn asl(&mut self, mode: &Instruction) {
    }

    //PHP, //08 Stack (Push) 1 3
    fn php(&mut self, mode: &Instruction) {
    }

    //PHD, //0B Stack (Push) 1 4
    fn phd(&mut self, mode: &Instruction) {
    }

    //BLP, //10 Program Counter Relative 2 2(7,8)
    fn blp(&mut self, mode: &Instruction) {
    }

    //TRB, //14 Direct Page 2 5(2,5)
    //TRB, //1C Absolute 3 6(5)
    fn trb(&mut self, mode: &Instruction) {
    }

    //CLC, //18 Implied 1 2
    fn clc(&mut self, mode: &Instruction) {
    }

    //INC, //1A Accumulator 1 2
    //INC, //E6 Direct Page 2 5(2,5)
    //INC, //EE Absolute 3 6(5)
    //INC, //F6 DP Indexed, X 2 6(2,5)
    //INC, //FE Absolute Indexed, X 3 7(5,6)
    fn inc(&mut self, mode: &Instruction) {
    }

    //TCS, //1B Implied 1 2
    fn tcs(&mut self, mode: &Instruction) {
    }

    //JSR, //20 Absolute 3 6
    //JSR, //22 Absolute Long 4 8
    //JSR, //FC Absolute Indexed Indirect 3 8
    fn jsr(&mut self, mode: &Instruction) {
    }

    //AND, //21 DP Indexed Indirect, X 2 6(1,2)
    //AND, //23 Stack Relative 2 4(1)
    //AND, //25 Direct Page 2 3(1,2)
    //AND, //27 DP Indirect Long 2 6(1,2)
    //AND, //29 Immediate 2* 2(1)
    //AND, //2D Absolute 3 4(1)
    //AND, //2F Absolute Long 4 5(1)
    //AND, //31 DP Indirect Indexed, Y 2 5(1,2,3)
    //AND, //32 DP Indirect 2 5(1,2)
    //AND, //33 SR Indirect Indexed, Y 2 7(1)
    //AND, //35 DP Indexed, X 2 4(1,2)
    //AND, //37 DP Indirect Long Indexed, Y 2 6(1,2)
    //AND, //39 Absolute Indexed, Y 3 4(1,3)
    //AND, //3D Absolute Indexed, X 3 4(1,3)
    //AND, //3F Absolute Long Indexed, X 4 5(1)
    fn and(&mut self, mode: &Instruction) {
    }

    //BIT, //24 Direct Page 2 3(1,2)
    //BIT, //2C Absolute 3 4(1)
    //BIT, //34 DP Indexed, X 2 4(1,2)
    //BIT, //3C Absolute Indexed, X 3 4(1,3)
    //BIT, //89 Immediate 2* 2(1)
    fn bit(&mut self, mode: &Instruction) {
    }

    //ROL, //26 Direct Page 2 5(2,5)
    //ROL, //2A Accumulator 1 2
    //ROL, //2E Absolute 3 6(5)
    //ROL, //36 DP Indexed, X 2 6(2,5)
    //ROL, //3E Absolute Indexed, X 3 7(5,6)
    fn rol(&mut self, mode: &Instruction) {
    }

    //PLP, //28 Stack (Pull) 1 4
    fn plp(&mut self, mode: &Instruction) {
    }

    //PLD, //2B Stack (Pull) 1 5
    fn pld(&mut self, mode: &Instruction) {
    }

    //BMI, //30 Program Counter Relative 2 2(7,8)
    fn bmi(&mut self, mode: &Instruction) {
    }

    //SEC, //38 Implied 1 2
    fn sec(&mut self, mode: &Instruction) {
    }

    //DEC, //3A Accumulator 1 2
    //DEC, //C6 Direct Page 2 5(2,5)
    //DEC, //CE Absolute 3 6(5)
    //DEC, //D6 DP Indexed, X 2 6(2,5)
    //DEC, //DE Absolute Indexed, X 3 7(5,6)
    fn dec(&mut self, mode: &Instruction) {
    }

    //TSC, //3B Implied 1 2
    fn tsc(&mut self, mode: &Instruction) {
    }

    //RTI, //40 Stack/RTI 1 6(9)
    fn rti(&mut self, mode: &Instruction) {
    }

    //EOR, //41 DP Indexed Indirect, X 2 6(1,2)
    //EOR, //43 Stack Relative 2 4(1)
    //EOR, //45 Direct Page 2 3(1,2)
    //EOR, //47 DP Indirect Long 2 6(1,2)
    //EOR, //49 Immediate 2* 2(1)
    //EOR, //4D Absolute 3 4(1)
    //EOR, //4F Absolute Long 4 5(1)
    //EOR, //51 DP Indirect Indexed, Y 2 5(1,2,3,)
    //EOR, //52 DP Indirect 2 5(1,2)
    //EOR, //53 SR Indirect Indexed, Y 2 7(1)
    //EOR, //55 DP Indexed, X 2 4(1,2)
    //EOR, //57 DP Indirect Long Indexed, Y 2 6(1,2)
    //EOR, //59 Absolute Indexed, Y 3 4(1,3)
    //EOR, //5D Absolute Indexed, X 3 4(1,3)
    //EOR, //5F Absolute Long Indexed, X 4 5(1)
    fn eor(&mut self, mode: &Instruction) {
    }

    //WDM, //42 2^16 (16)
    fn wdm(&mut self, mode: &Instruction) {
    }

    //MVP, //44 Block Move 3(13)
    fn mvp(&mut self, mode: &Instruction) {
    }

    //LSR, //46 Direct Page 2 5(2,5)
    //LSR, //4A Accumulator 1 2
    //LSR, //4E Absolute 3 6(5)
    //LSR, //56 DP Indexed, X 2 6(2,5)
    //LSR, //5E Absolute Indexed, X 3 7(5,6)
    fn lsr(&mut self, mode: &Instruction) {
    }

    //PHA, //48 Stack (Push) 1 3(1)
    fn pha(&mut self, mode: &Instruction) {
    }

    //PHK, //4B Stack (Push) 1 3
    fn phk(&mut self, mode: &Instruction) {
    }

    //JMP, //4C Absolute 3 3
    //JMP, //5C Absolute Long 4 4
    //JMP, //6C Absolute Indirect 3 5(11,12)
    //JMP, //7C Absolute Indexed Indirect 3 6
    //JMP, //DC Absolute Indirect Long 3 6
    fn jmp(&mut self, mode: &Instruction) {
    }

    //BVC, //50 Program Counter Relative 2 2(7,8)
    fn bvc(&mut self, mode: &Instruction) {
    }

    //MVN, //54 Block Move 3(13)
    fn mvn(&mut self, mode: &Instruction) {
    }

    //CLI, //58 Implied 1 2
    fn cli(&mut self, mode: &Instruction) {
    }

    //PHY, //5A Stack (Push) 1 3(10)
    fn phy(&mut self, mode: &Instruction) {
    }

    //TCD, //5B Implied 1 2
    fn tcd(&mut self, mode: &Instruction) {
    }

    //RTS, //60 Stack (RTS) 1 6
    fn rts(&mut self, mode: &Instruction) {
    }

    //ADC, //61 DP Indexed Indirect, X 2 6(1,2,4)
    //ADC, //63 Stack Relative 2 4(1,4)
    //ADC, //65 Direct Page 2 3(1,2,4)
    //ADC, //67 DP Indirect Long 2 6(1,2,4)
    //ADC, //69 Immediate 2* 2(1,4)
    //ADC, //6D Absolute 3 4(1,4)
    //ADC, //6F Absolute Long 4 5(1,4)
    //ADC, //71 DP Indirect Indexed, Y 2 5(1,2,3,4)
    //ADC, //72 DP Indirect 2 5(1,2,4)
    //ADC, //73 SR Indirect Indexed, Y 2 7(1,4)
    //ADC, //75 DP Indexed, X 2 4(1,2,4)
    //ADC, //77 DP Indirect Long Indexed, Y 2 6(1,2,4)
    //ADC, //79 Absolute Indexed, Y 3 4(1,3,4)
    //ADC, //7D Absolute Indexed, X 3 4(1,3,4)
    //ADC, //7F Absolute Long Indexed, X 4 5(1,4)
    fn adc(&mut self, mode: &Instruction) {
        use self::Instruction;

        let data: u16 = mode.load(self);
        let mut result: u32;
        let mut overflow = false;
        let acc = self.accumulator;
        let orig_neg = (acc & 0x8000) >> 15 == 1;

        if let Some(res) = data.checked_add(acc) {
            result = res as u32;
        } else {
            result = data as u32 + acc as u32;
            overflow = true;
        }

        if self.check_flag(CarryFlag) {
            result += 1;
            self.set_flag(CarryFlag, true);
        }

        self.accumulator = result as u16;

        let negative = (self.accumulator & 0x8000) >> 15 == 1;

        self.set_flag(NegativeFlag, negative);
        self.set_flag(OverflowFlag, overflow);
        self.set_flag(ZeroFlag, result == 0);
        self.set_flag(CarryFlag, (!orig_neg && negative) || (overflow && negative));
    }

    //PER, //62 Stack (PC Relative Long) 3 6
    fn per(&mut self, mode: &Instruction) {
    }

    //STZ, //64 Direct Page 2 3(1,2)
    //STZ, //74 Direct Page Indexed, X 2 4(1,2)
    //STZ, //9C Absolute 3 4(1)
    //STZ, //9E Absolute Indexed, X 3 5(1)
    fn stz(&mut self, mode: &Instruction) {
    }

    //ROR, //66 Direct Page 2 5(2,5)
    //ROR, //6A Accumulator 1 2
    //ROR, //6E Absolute 3 6(5)
    //ROR, //76 DP Indexed, X 2 6(2,5)
    //ROR, //7E Absolute Indexed, X 3 7(5,6)
    fn ror(&mut self, mode: &Instruction) {
    }

    //PLA, //68 Stack (Pull) 1 4(1)
    fn pla(&mut self, mode: &Instruction) {
    }

    //RTL, //6B Stack (RTL) 1 6
    fn rtl(&mut self, mode: &Instruction) {
    }

    //BVS, //70 Program Counter Relative 2 2(7,8)
    fn bvs(&mut self, mode: &Instruction) {
    }

    //SEI, //78 Implied 1 2
    fn sei(&mut self, mode: &Instruction) {
    }

    //PLY, //7A Stack/Pull 1 4(10)
    fn ply(&mut self, mode: &Instruction) {
    }

    //TDC, //7B Implied 1 2
    fn tdc(&mut self, mode: &Instruction) {
    }

    //BRA, //80 Program Counter Relative 2 3(8)
    fn bra(&mut self, mode: &Instruction) {
    }

    //STA, //81 DP Indexed Indirect, X 2 6(1,2)
    //STA, //83 Stack Relative 2 4(1)
    //STA, //85 Direct Page 2 3(1,2)
    //STA, //87 DP Indirect Long 2 6(1,2)
    //STA, //8D Absolute 3 4(1)
    //STA, //8F Absolute Long 4 5(1)
    //STA, //91 DP Indirect Indexed, Y 2 6(1,2)
    //STA, //92 DP Indirect 2 5(1,2)
    //STA, //93 SR Indirect Indexed, Y 2 7(1)
    //STA, //95 DP Indexed, X 2 4(1,2)
    //STA, //97 DP Indirect Long Indexed, Y 2 6(1,2)
    //STA, //99 Absolute Indexed, Y 3 5(1)
    //STA, //9D Absolute Indexed, X 3 5(1)
    //STA, //9F Absolute Long Indexed, X 4 5(1)
    fn sta(&mut self, mode: &Instruction) {
    }

    //BRL, //82 Program Counter Relative Long 3 4
    fn brl(&mut self, mode: &Instruction) {
    }

    //STY, //84 Direct Page 2 3(2,10)
    //STY, //8C Absolute 3 4(10)
    //STY, //94 Direct Page Indexed, X 2 4(2,10)
    fn sty(&mut self, mode: &Instruction) {
    }

    //STX, //86 Direct Page 2 3(2,10)
    //STX, //8E Absolute 3 4(10)
    //STX, //96 Direct Page Indexed, Y 2 4(2,10)
    fn stx(&mut self, mode: &Instruction) {
    }

    //DEY, //88 Implied 1 2
    fn dey(&mut self, mode: &Instruction) {
    }

    //TXA, //8A Implied 1 2
    fn txa(&mut self, mode: &Instruction) {
    }

    //PHB, //8B Stack (Push) 1 3
    fn phb(&mut self, mode: &Instruction) {
    }

    //BCC, //90 Program Counter Relative 2 2(7,8)
    fn bcc(&mut self, mode: &Instruction) {
    }

    //TYA, //98 Implied 1 2
    fn tya(&mut self, mode: &Instruction) {
    }

    //TXS, //9A Implied 1 2
    fn txs(&mut self, mode: &Instruction) {
    }

    //TXY, //9B Implied 1 2
    fn txy(&mut self, mode: &Instruction) {
    }

    //LDY, //A0 Immediate 2+ 2(10)
    //LDY, //A4 Direct Page 2 3(2,10)
    //LDY, //AC Absolute 3 4(10)
    //LDY, //B4 DP Indexed, X 2 4(2,10)
    //LDY, //BC Absolute Indexed, X 3 4(3,10)
    fn ldy(&mut self, mode: &Instruction) {
    }

    //LDA, //A1 DP Indexed Indirect, X 2 6(1,2)
    //LDA, //A3 Stack Relative 2 4(1)
    //LDA, //A5 Direct Page 2 3(1,2)
    //LDA, //A7 DP Indirect Long 2 6(1,2)
    //LDA, //A9 Immediate 2* 2(1)
    //LDA, //AD Absolute 3 4(1)
    //LDA, //AF Absolute Long 4 5(1)
    //LDA, //B1 DP Indirect Indexed, Y 2 5(1,2,3)
    //LDA, //B2 DP Indirect 2 5(1,2)
    //LDA, //B3 SR Indirect Indexed, Y 2 7(1)
    //LDA, //B5 DP Indexed, X 2 4(1,2)
    //LDA, //B7 DP Indirect Long Indexed, Y 2 6(1,2)
    //LDA, //B9 Absolute Indexed, Y 3 4(1,3)
    //LDA, //BD Absolute Indexed, X 3 4(1,3)
    //LDA, //BF Absolute Long Indexed, X 4 5(1)
    fn lda(&mut self, mode: &Instruction) {
    }

    //LDX, //A2 Immediate 2+ 2(10)
    //LDX, //A6 Direct Page 2 3(2,10)
    //LDX, //AE Absolute 3 4(10)
    //LDX, //B6 DP Indexed, Y 2 4(2,10)
    //LDX, //BE Absolute Indexed, Y 3 4(3,10)
    fn ldx(&mut self, mode: &Instruction) {
    }

    //TAY, //A8 Implied 1 2
    fn tay(&mut self, mode: &Instruction) {
    }

    //TAX, //AA Implied 1 2
    fn tax(&mut self, mode: &Instruction) {
    }

    //PLB, //AB Stack (Pull) 1 4
    fn plb(&mut self, mode: &Instruction) {
    }

    //BCS, //B0 Program Counter Relative 2 2(7,8)
    fn bcs(&mut self, mode: &Instruction) {
    }

    //CLV, //B8 Implied 1 2
    fn clv(&mut self, mode: &Instruction) {
    }

    //TSX, //BA Implied 1 2
    fn tsx(&mut self, mode: &Instruction) {
    }

    //TYX, //BB Implied 1 2
    fn tyx(&mut self, mode: &Instruction) {
    }

    //CPY, //C0 Immediate 2+ 2(10)
    //CPY, //C4 Direct Page 2 3(2,10)
    //CPY, //CC Absolute 3 4(10)
    fn cpy(&mut self, mode: &Instruction) {
    }

    //CMP, //C1 DP Indexed Indirect, X 2 6(1,2)
    //CMP, //C3 Stack Relative 2 4(1)
    //CMP, //C5 Direct Page 2 3(1,2)
    //CMP, //C7 DP Indirect Long 2 6(1,2)
    //CMP, //C9 Immediate 2* 2(1)
    //CMP, //CD Absolute 3 4(1)
    //CMP, //CF Absolute Long 4 5(1)
    //CMP, //D1 DP Indirect Indexed, Y 2 5(1,2,3)
    //CMP, //D2 DP Indirect 2 5(1,2)
    //CMP, //D3 SR Indirect Indexed, Y 2 7(1)
    //CMP, //D5 DP Indexed, X 2 4(1,2)
    //CMP, //D7 DP Indirect Long Indexed, Y 2 6(1,2)
    //CMP, //D9 Absolute Indexed, Y 3 4(1,3)
    //CMP, //DD Absolute Indexed, X 3 41,3
    //CMP, //DF Absolute Long Indexed, X 4 5(1)
    fn cmp(&mut self, mode: &Instruction) {
    }

    //REP, //C2 Immediate 2 3
    fn rep(&mut self, mode: &Instruction) {
    }

    //INY, //C8 Implied 1 2
    fn iny(&mut self, mode: &Instruction) {
    }

    //DEX, //CA Implied 1 2
    fn dex(&mut self, mode: &Instruction) {
    }

    //WAI, //CB Implied 1 3(15)
    fn wai(&mut self, mode: &Instruction) {
    }

    //BNE, //D0 Program Counter Relative 2 2(7,8)
    fn bne(&mut self, mode: &Instruction) {
    }

    //PEI, //D4 Stack (Direct Page Indirect) 2 6(2)
    fn pei(&mut self, mode: &Instruction) {
    }

    //CLD, //D8 Implied 1 2
    fn cld(&mut self, mode: &Instruction) {
    }

    //PHX, //DA Stack (Push) 1 3(10)
    fn phx(&mut self, mode: &Instruction) {
    }

    //STP, //DB Implied 1 3(14)
    fn stp(&mut self, mode: &Instruction) {
    }

    //CPX, //E0 Immediate 2+ 2(10)
    //CPX, //E2 Immediate 2 3
    //CPX, //EC Absolute 3 4(10)
    fn cpx(&mut self, mode: &Instruction) {
    }

    //SBC, //E1 DP Indexed Indirect, X 2 6(1,2,4)
    //SBC, //E3 Stack Relative 2 4(1,4)
    //SBC, //E5 Direct Page 2 3(1,2,4)
    //SBC, //E7 DP Indirect Long 2 6(1,2,4)
    //SBC, //E9 Immediate 2* 2(1,4)
    //SBC, //ED Absolute 3 4(1,4)
    //SBC, //EF Absolute Long 4 5(1,4)
    //SBC, //F1 DP Indirect Indexed, Y 2 5(1,2,3,4)
    //SBC, //F2 DP Indirect 2 5(1,2,4)
    //SBC, //F3 SR Indirect Indexed, Y 2 7(1,4)
    //SBC, //F5 DP Indexed, X 2 4(1,2,4)
    //SBC, //F7 DP Indirect Long Indexed, Y 2 6(1,2,4)
    //SBC, //F9 Absolute Indexed, Y 3 4(1,3,4)
    //SBC, //FD Absolute Indexed, X 3 4(1,3,4)
    //SBC, //FF Absolute Long Indexed, X 4 5(1,4)
    fn sbc(&mut self, mode: &Instruction) {
    }

    //INX, //E4 Direct Page 2 3(2,10)
    //INX, //E8 Implied 1 2
    fn inx(&mut self, mode: &Instruction) {
    }

    //NOP, //EA Implied 1 2
    fn nop(&mut self, mode: &Instruction) {
    }

    //XBA, //EB Implied 1 3
    fn xba(&mut self, mode: &Instruction) {
    }

    //BEQ, //F0 Program Counter Relative 2 2(7,8)
    fn beq(&mut self, mode: &Instruction) {
    }

    //PEA, //F4 Stack (absolute) 3 5
    fn pea(&mut self, mode: &Instruction) {
    }

    //SED, //F8 Implied 1 2
    fn sed(&mut self, mode: &Instruction) {
    }

    //PLX, //FA Stack /Pull 1 4(10)
    fn plx(&mut self, mode: &Instruction) {
    }

    //XCE, //FB Implied 1 2
    fn xce(&mut self, mode: &Instruction) {
    }
}

pub trait Instruction {
    fn load(&self, cpu: &mut CPU) -> u16;
    fn store(&self, cpu: &mut CPU, data: u16);
}

struct Absolute;
impl Instruction for Absolute {
    fn load(&self, cpu: &mut CPU) -> u16 {
        let pc = cpu.program_counter as usize;
        let high = cpu.memory.get_byte(pc + 1) as u32;
        let low = cpu.memory.get_byte(pc + 2) as u32;
        let addr: u32 = (cpu.data_bank as u32) << 16 | high << 8 | low;

        cpu.program_counter += 3;

        cpu.memory.get_word(addr as usize)
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        let pc = cpu.program_counter as usize;
        let high = cpu.memory.get_byte(pc + 1) as u32;
        let low = cpu.memory.get_byte(pc + 2) as u32;
        let addr: u32 = (cpu.data_bank as u32) << 16 | high << 8 | low;

        cpu.program_counter += 3;

        cpu.memory.set_word(addr as usize, data);
    }
}

struct AbsoluteIndexedX;
impl Instruction for AbsoluteIndexedX {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct AbsoluteIndexedY;
impl Instruction for AbsoluteIndexedY {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct AbsoluteIndexedIndirect;
impl Instruction for AbsoluteIndexedIndirect {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct AbsoluteIndirect;
impl Instruction for AbsoluteIndirect {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct AbsoluteIndirectLong;
impl Instruction for AbsoluteIndirectLong {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct AbsoluteLong;
impl Instruction for AbsoluteLong {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct AbsoluteLongIndexedX;
impl Instruction for AbsoluteLongIndexedX {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct Accumulator;
impl Instruction for Accumulator {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct BlockMove;
impl Instruction for BlockMove {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct DirectPage;
impl Instruction for DirectPage {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct DirectPageIndexedX;
impl Instruction for DirectPageIndexedX {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct DirectPageIndexedY;
impl Instruction for DirectPageIndexedY {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct DirectPageIndexedIndirectX;
impl Instruction for DirectPageIndexedIndirectX {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct DirectPageIndirect;
impl Instruction for DirectPageIndirect {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct DirectPageIndirectLong;
impl Instruction for DirectPageIndirectLong {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct DirectPageIndirectIndexedY;
impl Instruction for DirectPageIndirectIndexedY {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct DirectPageIndirectLongIndexedY;
impl Instruction for DirectPageIndirectLongIndexedY {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct Immediate;
impl Instruction for Immediate {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct Implied;
impl Instruction for Implied {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct ProgramCounterRelative;
impl Instruction for ProgramCounterRelative {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct ProgramCounterRelativeLong;
impl Instruction for ProgramCounterRelativeLong {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct StackAbsolute;
impl Instruction for StackAbsolute {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct StackDirectPageIndirect;
impl Instruction for StackDirectPageIndirect {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct StackInterrupt;
impl Instruction for StackInterrupt {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct StackProgramCounterRelative;
impl Instruction for StackProgramCounterRelative {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct StackPull;
impl Instruction for StackPull {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct StackPush;
impl Instruction for StackPush {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct StackRTI;
impl Instruction for StackRTI {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct StackRTL;
impl Instruction for StackRTL {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct StackRTS;
impl Instruction for StackRTS {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct StackRelative;
impl Instruction for StackRelative {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

struct StackRelativeIndirectIndexedY;
impl Instruction for StackRelativeIndirectIndexedY {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

