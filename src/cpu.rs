use memory::Memory;
use flags::Flags;
use flags::Flags::*;
use self::InstructionType::*;

macro_rules! decode_op_and_execute {
    ($op:expr, $this:ident) => (
        match $op {
            0x00 => {
                let mode = StackInterrupt;
                $this.brk(&mode);
            },
            0x01 => {
                let mode = DirectPageIndexedIndirectX;
                $this.ora(&mode);
            },
            0x02 => {
                let mode = StackInterrupt;
                $this.cop(&mode);
            },
            0x03 => {
                let mode = StackRelative;
                $this.ora(&mode);
            },
            0x04 => {
                let mode = DirectPage;
                $this.tsb(&mode);
            },
            0x05 => {
                let mode = DirectPage;
                $this.ora(&mode);
            },
            0x06 => {
                let mode = DirectPage;
                $this.asl(&mode);
            },
            0x07 => {
                let mode = DirectPageIndirectLong;
                $this.ora(&mode);
            },
            0x08 => {
                let mode = StackPush;
                $this.php(&mode);
            },
            0x09 => {
                let mode = Immediate;
                $this.ora(&mode);
            },
            0x0A => {
                let mode = Accumulator;
                $this.asl(&mode);
            },
            0x0B => {
                let mode = StackPush;
                $this.phd(&mode);
            },
            0x0C => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.tsb(&mode);
            },
            0x0D => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.ora(&mode);
            },
            0x0E => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.asl(&mode);
            },
            0x0F => {
                let mode = AbsoluteLong;
                $this.ora(&mode);
            },
            0x10 => {
                let mode = ProgramCounterRelative;
                $this.blp(&mode);
            },
            0x11 => {
                let mode = DirectPageIndirectIndexedY;
                $this.ora(&mode);
            },
            0x12 => {
                let mode = DirectPageIndirect;
                $this.ora(&mode);
            },
            0x13 => {
                let mode = StackRelativeIndirectIndexedY;
                $this.ora(&mode);
            },
            0x14 => {
                let mode = DirectPage;
                $this.trb(&mode);
            },
            0x15 => {
                let mode = DirectPageIndexedX;
                $this.ora(&mode);
            },
            0x16 => {
                let mode = DirectPageIndexedX;
                $this.asl(&mode);
            },
            0x17 => {
                let mode = DirectPageIndirectLongIndexedY;
                $this.ora(&mode);
            },
            0x18 => {
                let mode = Implied;
                $this.clc(&mode);
            },
            0x19 => {
                let mode = AbsoluteIndexedY;
                $this.ora(&mode);
            },
            0x1A => {
                let mode = Accumulator;
                $this.inc(&mode);
            },
            0x1B => {
                let mode = Implied;
                $this.tcs(&mode);
            },
            0x1C => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.trb(&mode);
            },
            0x1D => {
                let mode = AbsoluteIndexedX;
                $this.ora(&mode);
            },
            0x1E => {
                let mode = AbsoluteIndexedX;
                $this.asl(&mode);
            },
            0x1F => {
                let mode = AbsoluteLongIndexedX;
                $this.ora(&mode);
            },
            0x20 => {
                let mode = Absolute { instruction_type: ControlTransfer };
                $this.jsr(&mode);
            },
            0x21 => {
                let mode = DirectPageIndexedIndirectX;
                $this.and(&mode);
            },
            0x22 => {
                let mode = AbsoluteLong;
                $this.jsr(&mode);
            },
            0x23 => {
                let mode = StackRelative;
                $this.and(&mode);
            },
            0x24 => {
                let mode = DirectPage;
                $this.bit(&mode);
            },
            0x25 => {
                let mode = DirectPage;
                $this.and(&mode);
            },
            0x26 => {
                let mode = DirectPage;
                $this.rol(&mode);
            },
            0x27 => {
                let mode = DirectPageIndirectLong;
                $this.and(&mode);
            },
            0x28 => {
                let mode = StackPull;
                $this.plp(&mode);
            },
            0x29 => {
                let mode = Immediate;
                $this.and(&mode);
            },
            0x2A => {
                let mode = Accumulator;
                $this.rol(&mode);
            },
            0x2B => {
                let mode = StackPull;
                $this.pld(&mode);
            },
            0x2C => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.bit(&mode);
            },
            0x2D => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.and(&mode);
            },
            0x2E => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.rol(&mode);
            },
            0x2F => {
                let mode = AbsoluteLong;
                $this.and(&mode);
            },
            0x30 => {
                let mode = ProgramCounterRelative;
                $this.bmi(&mode);
            },
            0x31 => {
                let mode = DirectPageIndirectIndexedY;
                $this.and(&mode);
            },
            0x32 => {
                let mode = DirectPageIndirect;
                $this.and(&mode);
            },
            0x33 => {
                let mode = StackRelativeIndirectIndexedY;
                $this.and(&mode);
            },
            0x34 => {
                let mode = DirectPageIndexedX;
                $this.bit(&mode);
            },
            0x35 => {
                let mode = DirectPageIndexedX;
                $this.and(&mode);
            },
            0x36 => {
                let mode = DirectPageIndexedX;
                $this.rol(&mode);
            },
            0x37 => {
                let mode = DirectPageIndirectLongIndexedY;
                $this.and(&mode);
            },
            0x38 => {
                let mode = Implied;
                $this.sec(&mode);
            },
            0x39 => {
                let mode = AbsoluteIndexedY;
                $this.and(&mode);
            },
            0x3A => {
                let mode = Accumulator;
                $this.dec(&mode);
            },
            0x3B => {
                let mode = Implied;
                $this.tsc(&mode);
            },
            0x3C => {
                let mode = AbsoluteIndexedX;
                $this.bit(&mode);
            },
            0x3D => {
                let mode = AbsoluteIndexedX;
                $this.and(&mode);
            },
            0x3E => {
                let mode = AbsoluteIndexedX;
                $this.rol(&mode);
            },
            0x3F => {
                let mode = AbsoluteLongIndexedX;
                $this.and(&mode);
            },
            0x40 => {
                let mode = StackRTI;
                $this.rti(&mode);
            },
            0x41 => {
                let mode = DirectPageIndexedIndirectX;
                $this.eor(&mode);
            },
            0x42 => {
                $this.wdm();
            },
            0x43 => {
                let mode = StackRelative;
                $this.eor(&mode);
            },
            0x44 => {
                let mode = BlockMove;
                $this.mvp(&mode);
            },
            0x45 => {
                let mode = DirectPage;
                $this.eor(&mode);
            },
            0x46 => {
                let mode = DirectPage;
                $this.lsr(&mode);
            },
            0x47 => {
                let mode = DirectPageIndirectLong;
                $this.eor(&mode);
            },
            0x48 => {
                let mode = StackPush;
                $this.pha(&mode);
            },
            0x49 => {
                let mode = Immediate;
                $this.eor(&mode);
            },
            0x4A => {
                let mode = Accumulator;
                $this.lsr(&mode);
            },
            0x4B => {
                let mode = StackPush;
                $this.phk(&mode);
            },
            0x4C => {
                let mode = Absolute { instruction_type: ControlTransfer };
                $this.jmp(&mode);
            },
            0x4D => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.eor(&mode);
            },
            0x4E => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.lsr(&mode);
            },
            0x4F => {
                let mode = AbsoluteLong;
                $this.eor(&mode);
            },
            0x50 => {
                let mode = ProgramCounterRelative;
                $this.bvc(&mode);
            },
            0x51 => {
                let mode = DirectPageIndirectIndexedY;
                $this.eor(&mode);
            },
            0x52 => {
                let mode = DirectPageIndirect;
                $this.eor(&mode);
            },
            0x53 => {
                let mode = StackRelativeIndirectIndexedY;
                $this.eor(&mode);
            },
            0x54 => {
                let mode = BlockMove;
                $this.mvn(&mode);
            },
            0x55 => {
                let mode = DirectPageIndexedX;
                $this.eor(&mode);
            },
            0x56 => {
                let mode = DirectPageIndexedX;
                $this.lsr(&mode);
            },
            0x57 => {
                let mode = DirectPageIndirectLongIndexedY;
                $this.eor(&mode);
            },
            0x58 => {
                let mode = Implied;
                $this.cli(&mode);
            },
            0x59 => {
                let mode = AbsoluteIndexedY;
                $this.eor(&mode);
            },
            0x5A => {
                let mode = StackPush;
                $this.phy(&mode);
            },
            0x5B => {
                let mode = Implied;
                $this.tcd(&mode);
            },
            0x5C => {
                let mode = AbsoluteLong;
                $this.jmp(&mode);
            },
            0x5D => {
                let mode = AbsoluteIndexedX;
                $this.eor(&mode);
            },
            0x5E => {
                let mode = AbsoluteIndexedX;
                $this.lsr(&mode);
            },
            0x5F => {
                let mode = AbsoluteLongIndexedX;
                $this.eor(&mode);
            },
            0x60 => {
                let mode = StackRTS;
                $this.rts(&mode);
            },
            0x61 => {
                let mode = DirectPageIndexedIndirectX;
                $this.adc(&mode);
            },
            0x62 => {
                let mode = StackProgramCounterRelative;
                $this.per(&mode);
            },
            0x63 => {
                let mode = StackRelative;
                $this.adc(&mode);
            },
            0x64 => {
                let mode = DirectPage;
                $this.stz(&mode);
            },
            0x65 => {
                let mode = DirectPage;
                $this.adc(&mode);
            },
            0x66 => {
                let mode = DirectPage;
                $this.ror(&mode);
            },
            0x67 => {
                let mode = DirectPageIndirectLong;
                $this.adc(&mode);
            },
            0x68 => {
                let mode = StackPull;
                $this.pla(&mode);
            },
            0x69 => {
                let mode = Immediate;
                $this.adc(&mode);
            },
            0x6A => {
                let mode = Accumulator;
                $this.ror(&mode);
            },
            0x6B => {
                let mode = StackRTL;
                $this.rtl(&mode);
            },
            0x6C => {
                let mode = AbsoluteIndirect;
                $this.jmp(&mode);
            },
            0x6D => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.adc(&mode);
            },
            0x6E => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.ror(&mode);
            },
            0x6F => {
                let mode = AbsoluteLong;
                $this.adc(&mode);
            },
            0x70 => {
                let mode = ProgramCounterRelative;
                $this.bvs(&mode);
            },
            0x71 => {
                let mode = DirectPageIndirectIndexedY;
                $this.adc(&mode);
            },
            0x72 => {
                let mode = DirectPageIndirect;
                $this.adc(&mode);
            },
            0x73 => {
                let mode = StackRelativeIndirectIndexedY;
                $this.adc(&mode);
            },
            0x74 => {
                let mode = StackRelativeIndirectIndexedY;
                $this.stz(&mode);
            },
            0x75 => {
                let mode = DirectPageIndexedX;
                $this.adc(&mode);
            },
            0x76 => {
                let mode = DirectPageIndexedX;
                $this.ror(&mode);
            },
            0x77 => {
                let mode = DirectPageIndirectLongIndexedY;
                $this.adc(&mode);
            },
            0x78 => {
                let mode = Implied;
                $this.sei(&mode);
            },
            0x79 => {
                let mode = AbsoluteIndexedY;
                $this.adc(&mode);
            },
            0x7A => {
                let mode = StackPull;
                $this.ply(&mode);
            },
            0x7B => {
                let mode = Implied;
                $this.tdc(&mode);
            },
            0x7C => {
                let mode = AbsoluteIndexedIndirect;
                $this.jmp(&mode);
            },
            0x7D => {
                let mode = AbsoluteIndexedX;
                $this.adc(&mode);
            },
            0x7E => {
                let mode = AbsoluteIndexedX;
                $this.ror(&mode);
            },
            0x7F => {
                let mode = AbsoluteLongIndexedX;
                $this.adc(&mode);
            },
            0x80 => {
                let mode = ProgramCounterRelative;
                $this.bra(&mode);
            },
            0x81 => {
                let mode = DirectPageIndexedIndirectX;
                $this.sta(&mode);
            },
            0x82 => {
                let mode = ProgramCounterRelativeLong;
                $this.brl(&mode);
            },
            0x83 => {
                let mode = StackRelative;
                $this.sta(&mode);
            },
            0x84 => {
                let mode = DirectPage;
                $this.sty(&mode);
            },
            0x85 => {
                let mode = DirectPage;
                $this.sta(&mode);
            },
            0x86 => {
                let mode = DirectPage;
                $this.stx(&mode);
            },
            0x87 => {
                let mode = DirectPageIndirectLong;
                $this.sta(&mode);
            },
            0x88 => {
                let mode = Implied;
                $this.dey(&mode);
            },
            0x89 => {
                let mode = Immediate;
                $this.bit(&mode);
            },
            0x8A => {
                let mode = Implied;
                $this.txa(&mode);
            },
            0x8B => {
                let mode = StackPush;
                $this.phb(&mode);
            },
            0x8C => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.sty(&mode);
            },
            0x8D => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.sta(&mode);
            },
            0x8E => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.stx(&mode);
            },
            0x8F => {
                let mode = AbsoluteLong;
                $this.sta(&mode);
            },
            0x90 => {
                let mode = ProgramCounterRelative;
                $this.bcc(&mode);
            },
            0x91 => {
                let mode = DirectPageIndirectIndexedY;
                $this.sta(&mode);
            },
            0x92 => {
                let mode = DirectPageIndirect;
                $this.sta(&mode);
            },
            0x93 => {
                let mode = StackRelativeIndirectIndexedY;
                $this.sta(&mode);
            },
            0x94 => {
                let mode = DirectPageIndexedX;
                $this.sty(&mode);
            },
            0x95 => {
                let mode = DirectPageIndexedX;
                $this.sta(&mode);
            },
            0x96 => {
                let mode = DirectPageIndexedY;
                $this.stx(&mode);
            },
            0x97 => {
                let mode = DirectPageIndirectLongIndexedY;
                $this.sta(&mode);
            },
            0x98 => {
                let mode = Implied;
                $this.tya(&mode);
            },
            0x99 => {
                let mode = AbsoluteIndexedY;
                $this.sta(&mode);
            },
            0x9A => {
                let mode = Implied;
                $this.txs(&mode);
            },
            0x9B => {
                let mode = Implied;
                $this.txy(&mode);
            },
            0x9C => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.stz(&mode);
            },
            0x9D => {
                let mode = AbsoluteIndexedX;
                $this.sta(&mode);
            },
            0x9E => {
                let mode = AbsoluteIndexedX;
                $this.stz(&mode);
            },
            0x9F => {
                let mode = AbsoluteLongIndexedX;
                $this.sta(&mode);
            },
            0xA0 => {
                let mode = Immediate;
                $this.ldy(&mode);
            },
            0xA1 => {
                let mode = DirectPageIndexedIndirectX;
                $this.lda(&mode);
            },
            0xA2 => {
                let mode = Immediate;
                $this.ldx(&mode);
            },
            0xA3 => {
                let mode = StackRelative;
                $this.lda(&mode);
            },
            0xA4 => {
                let mode = DirectPage;
                $this.ldy(&mode);
            },
            0xA5 => {
                let mode = DirectPage;
                $this.lda(&mode);
            },
            0xA6 => {
                let mode = DirectPage;
                $this.ldx(&mode);
            },
            0xA7 => {
                let mode = DirectPageIndirectLong;
                $this.lda(&mode);
            },
            0xA8 => {
                let mode = Implied;
                $this.tay(&mode);
            },
            0xA9 => {
                let mode = Immediate;
                $this.lda(&mode);
            },
            0xAA => {
                let mode = Implied;
                $this.tax(&mode);
            },
            0xAB => {
                let mode = StackPull;
                $this.plb(&mode);
            },
            0xAC => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.ldy(&mode);
            },
            0xAD => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.lda(&mode);
            },
            0xAE => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.ldx(&mode);
            },
            0xAF => {
                let mode = AbsoluteLong;
                $this.lda(&mode);
            },
            0xB0 => {
                let mode = ProgramCounterRelative;
                $this.bcs(&mode);
            },
            0xB1 => {
                let mode = DirectPageIndirectIndexedY;
                $this.lda(&mode);
            },
            0xB2 => {
                let mode = DirectPageIndirect;
                $this.lda(&mode);
            },
            0xB3 => {
                let mode = StackRelativeIndirectIndexedY;
                $this.lda(&mode);
            },
            0xB4 => {
                let mode = DirectPageIndexedX;
                $this.ldy(&mode);
            },
            0xB5 => {
                let mode = DirectPageIndexedX;
                $this.lda(&mode);
            },
            0xB6 => {
                let mode = DirectPageIndexedY;
                $this.ldx(&mode);
            },
            0xB7 => {
                let mode = DirectPageIndirectLongIndexedY;
                $this.lda(&mode);
            },
            0xB8 => {
                let mode = Implied;
                $this.clv(&mode);
            },
            0xB9 => {
                let mode = AbsoluteIndexedY;
                $this.lda(&mode);
            },
            0xBA => {
                let mode = Implied;
                $this.tsx(&mode);
            },
            0xBB => {
                let mode = Implied;
                $this.tyx(&mode);
            },
            0xBC => {
                let mode = AbsoluteIndexedX;
                $this.ldy(&mode);
            },
            0xBD => {
                let mode = AbsoluteIndexedX;
                $this.lda(&mode);
            },
            0xBE => {
                let mode = AbsoluteIndexedY;
                $this.ldx(&mode);
            },
            0xBF => {
                let mode = AbsoluteLongIndexedX;
                $this.lda(&mode);
            },
            0xC0 => {
                let mode = Immediate;
                $this.cpy(&mode);
            },
            0xC1 => {
                let mode = DirectPageIndexedIndirectX;
                $this.cmp(&mode);
            },
            0xC2 => {
                let mode = Immediate;
                $this.rep(&mode);
            },
            0xC3 => {
                let mode = StackRelative;
                $this.cmp(&mode);
            },
            0xC4 => {
                let mode = DirectPage;
                $this.cpy(&mode);
            },
            0xC5 => {
                let mode = DirectPage;
                $this.cmp(&mode);
            },
            0xC6 => {
                let mode = DirectPage;
                $this.dec(&mode);
            },
            0xC7 => {
                let mode = DirectPageIndirectLong;
                $this.cmp(&mode);
            },
            0xC8 => {
                let mode = Implied;
                $this.iny(&mode);
            },
            0xC9 => {
                let mode = Immediate;
                $this.cmp(&mode);
            },
            0xCA => {
                let mode = Implied;
                $this.dex(&mode);
            },
            0xCB => {
                let mode = Implied;
                $this.wai(&mode);
            },
            0xCC => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.cpy(&mode);
            },
            0xCD => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.cmp(&mode);
            },
            0xCE => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.dec(&mode);
            },
            0xCF => {
                let mode = AbsoluteLong;
                $this.cmp(&mode);
            },
            0xD0 => {
                let mode = ProgramCounterRelative;
                $this.bne(&mode);
            },
            0xD1 => {
                let mode = DirectPageIndirectIndexedY;
                $this.cmp(&mode);
            },
            0xD2 => {
                let mode = DirectPageIndirect;
                $this.cmp(&mode);
            },
            0xD3 => {
                let mode = StackRelativeIndirectIndexedY;
                $this.cmp(&mode);
            },
            0xD4 => {
                let mode = StackDirectPageIndirect;
                $this.pei(&mode);
            },
            0xD5 => {
                let mode = DirectPageIndexedX;
                $this.cmp(&mode)
            },
            0xD6 => {
                let mode = DirectPageIndexedX;
                $this.dec(&mode);
            },
            0xD7 => {
                let mode = DirectPageIndirectLongIndexedY;
                $this.cmp(&mode);
            },
            0xD8 => {
                let mode = Implied;
                $this.cld(&mode);
            },
            0xD9 => {
                let mode = AbsoluteIndexedY;
                $this.cmp(&mode);
            },
            0xDA => {
                let mode = StackPush;
                $this.phx(&mode);
            },
            0xDB => {
                let mode = Implied;
                $this.stp(&mode);
            },
            0xDC => {
                let mode = AbsoluteIndirectLong;
                $this.jmp(&mode);
            },
            0xDD => {
                let mode = AbsoluteIndexedX;
                $this.cmp(&mode);
            },
            0xDE => {
                let mode = AbsoluteIndexedX;
                $this.dec(&mode);
            },
            0xDF => {
                let mode = AbsoluteLongIndexedX;
                $this.cmp(&mode);
            },
            0xE0 => {
                let mode = Immediate;
                $this.cpx(&mode);
            },
            0xE1 => {
                let mode = DirectPageIndexedIndirectX;
                $this.sbc(&mode);
            },
            0xE2 => {
                let mode = Immediate;
                $this.cpx(&mode);
            },
            0xE3 => {
                let mode = DirectPage;
                $this.sbc(&mode);
            },
            0xE4 => {
                let mode = DirectPage;
                $this.inx(&mode);
            },
            0xE5 => {
                let mode = DirectPage;
                $this.sbc(&mode);
            },
            0xE6 => {
                let mode = DirectPage;
                $this.inc(&mode);
            },
            0xE7 => {
                let mode = DirectPageIndirectLong;
                $this.sbc(&mode);
            },
            0xE8 => {
                let mode = Implied;
                $this.inx(&mode);
            },
            0xE9 => {
                let mode = Immediate;
                $this.sbc(&mode);
            },
            0xEA => {
                let mode = Implied;
                $this.nop(&mode);
            },
            0xEB => {
                let mode = Implied;
                $this.xba(&mode);
            },
            0xEC => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.cpx(&mode);
            },
            0xED => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.sbc(&mode);
            },
            0xEE => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.inc(&mode);
            },
            0xEF => {
                let mode = AbsoluteLong;
                $this.sbc(&mode);
            },
            0xF0 => {
                let mode = ProgramCounterRelative;
                $this.beq(&mode);
            },
            0xF1 => {
                let mode = DirectPageIndirectIndexedY;
                $this.sbc(&mode);
            },
            0xF2 => {
                let mode = DirectPageIndirect;
                $this.sbc(&mode);
            },
            0xF3 => {
                let mode = StackRelativeIndirectIndexedY;
                $this.sbc(&mode);
            },
            0xF4 => {
                let mode = StackAbsolute;
                $this.pea(&mode);
            },
            0xF5 => {
                let mode = DirectPageIndexedX;
                $this.sbc(&mode);
            },
            0xF6 => {
                let mode = DirectPageIndexedX;
                $this.inc(&mode);
            },
            0xF7 => {
                let mode = DirectPageIndirectLongIndexedY;
                $this.sbc(&mode);
            },
            0xF8 => {
                let mode = Implied;
                $this.sed(&mode);
            },
            0xF9 => {
                let mode = AbsoluteIndexedY;
                $this.sbc(&mode);
            },
            0xFA => {
                let mode = StackPull;
                $this.plx(&mode);
            },
            0xFB => {
                let mode = Implied;
                $this.xce(&mode);
            },
            0xFC => {
                let mode = AbsoluteIndexedIndirect;
                $this.jsr(&mode);
            },
            0xFD => {
                let mode = AbsoluteIndexedX;
                $this.sbc(&mode);
            },
            0xFE => {
                let mode = AbsoluteIndexedX;
                $this.inc(&mode);
            },
            0xFF => {
                let mode = AbsoluteLongIndexedX;
                $this.sbc(&mode);
            },
            _ => panic!("{} is not an opcode", $op),
        }
    );
}

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
            let opcode = self.memory.get_byte(self.program_counter as usize);
            self.program_counter += 1;
            decode_op_and_execute!(opcode, self);
        }
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

    fn brk(&mut self, mode: &Instruction) {
    }

    fn ora(&mut self, mode: &Instruction) {
    }

    fn cop(&mut self, mode: &Instruction) {
    }

    fn tsb(&mut self, mode: &Instruction) {
    }

    fn asl(&mut self, mode: &Instruction) {
    }

    fn php(&mut self, mode: &Instruction) {
    }

    fn phd(&mut self, mode: &Instruction) {
    }

    fn blp(&mut self, mode: &Instruction) {
    }

    fn trb(&mut self, mode: &Instruction) {
    }

    fn clc(&mut self, mode: &Instruction) {
    }

    fn inc(&mut self, mode: &Instruction) {
    }

    fn tcs(&mut self, mode: &Instruction) {
    }

    fn jsr(&mut self, mode: &Instruction) {
    }

    fn and(&mut self, mode: &Instruction) {
    }

    fn bit(&mut self, mode: &Instruction) {
    }

    fn rol(&mut self, mode: &Instruction) {
    }

    fn plp(&mut self, mode: &Instruction) {
    }

    fn pld(&mut self, mode: &Instruction) {
    }

    fn bmi(&mut self, mode: &Instruction) {
    }

    fn sec(&mut self, mode: &Instruction) {
    }

    fn dec(&mut self, mode: &Instruction) {
    }

    fn tsc(&mut self, mode: &Instruction) {
    }

    fn rti(&mut self, mode: &Instruction) {
    }

    fn eor(&mut self, mode: &Instruction) {
    }

    fn wdm(&mut self) {
    }

    fn mvp(&mut self, mode: &Instruction) {
    }

    fn lsr(&mut self, mode: &Instruction) {
    }

    fn pha(&mut self, mode: &Instruction) {
    }

    fn phk(&mut self, mode: &Instruction) {
    }

    fn jmp(&mut self, mode: &Instruction) {
    }

    fn bvc(&mut self, mode: &Instruction) {
    }

    fn mvn(&mut self, mode: &Instruction) {
    }

    fn cli(&mut self, mode: &Instruction) {
    }

    fn phy(&mut self, mode: &Instruction) {
    }

    fn tcd(&mut self, mode: &Instruction) {
    }

    fn rts(&mut self, mode: &Instruction) {
    }

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
        self.set_flag(OverflowFlag, (!orig_neg && negative) || (overflow && negative));
        self.set_flag(ZeroFlag, result == 0);
        self.set_flag(CarryFlag, overflow);
    }

    fn per(&mut self, mode: &Instruction) {
    }

    fn stz(&mut self, mode: &Instruction) {
    }

    fn ror(&mut self, mode: &Instruction) {
    }

    fn pla(&mut self, mode: &Instruction) {
    }

    fn rtl(&mut self, mode: &Instruction) {
    }

    fn bvs(&mut self, mode: &Instruction) {
    }

    fn sei(&mut self, mode: &Instruction) {
    }

    fn ply(&mut self, mode: &Instruction) {
    }

    fn tdc(&mut self, mode: &Instruction) {
    }

    fn bra(&mut self, mode: &Instruction) {
    }

    fn sta(&mut self, mode: &Instruction) {
    }

    fn brl(&mut self, mode: &Instruction) {
    }

    fn sty(&mut self, mode: &Instruction) {
    }

    fn stx(&mut self, mode: &Instruction) {
    }

    fn dey(&mut self, mode: &Instruction) {
    }

    fn txa(&mut self, mode: &Instruction) {
    }

    fn phb(&mut self, mode: &Instruction) {
    }

    fn bcc(&mut self, mode: &Instruction) {
    }

    fn tya(&mut self, mode: &Instruction) {
    }

    fn txs(&mut self, mode: &Instruction) {
    }

    fn txy(&mut self, mode: &Instruction) {
    }

    fn ldy(&mut self, mode: &Instruction) {
    }

    fn lda(&mut self, mode: &Instruction) {
    }

    fn ldx(&mut self, mode: &Instruction) {
    }

    fn tay(&mut self, mode: &Instruction) {
    }

    fn tax(&mut self, mode: &Instruction) {
    }

    fn plb(&mut self, mode: &Instruction) {
    }

    fn bcs(&mut self, mode: &Instruction) {
    }

    fn clv(&mut self, mode: &Instruction) {
    }

    fn tsx(&mut self, mode: &Instruction) {
    }

    fn tyx(&mut self, mode: &Instruction) {
    }

    fn cpy(&mut self, mode: &Instruction) {
    }

    fn cmp(&mut self, mode: &Instruction) {
    }

    fn rep(&mut self, mode: &Instruction) {
    }

    fn iny(&mut self, mode: &Instruction) {
    }

    fn dex(&mut self, mode: &Instruction) {
    }

    fn wai(&mut self, mode: &Instruction) {
    }

    fn bne(&mut self, mode: &Instruction) {
    }

    fn pei(&mut self, mode: &Instruction) {
    }

    fn cld(&mut self, mode: &Instruction) {
    }

    fn phx(&mut self, mode: &Instruction) {
    }

    fn stp(&mut self, mode: &Instruction) {
    }

    fn cpx(&mut self, mode: &Instruction) {
    }

    fn sbc(&mut self, mode: &Instruction) {
    }

    fn inx(&mut self, mode: &Instruction) {
    }

    fn nop(&mut self, mode: &Instruction) {
    }

    fn xba(&mut self, mode: &Instruction) {
    }

    fn beq(&mut self, mode: &Instruction) {
    }

    fn pea(&mut self, mode: &Instruction) {
    }

    fn sed(&mut self, mode: &Instruction) {
    }

    fn plx(&mut self, mode: &Instruction) {
    }

    fn xce(&mut self, mode: &Instruction) {
    }
}

pub trait Instruction {
    fn load(&self, cpu: &mut CPU) -> u16;
    fn store(&self, cpu: &mut CPU, data: u16);
}

pub enum InstructionType {
    LocatingData,
    ControlTransfer,
}

struct Absolute { instruction_type: InstructionType }
impl Instruction for Absolute {
    fn load(&self, cpu: &mut CPU) -> u16 {
        let pc = cpu.program_counter as usize;
        let bank = {
            match self.instruction_type {
                LocatingData    =>    cpu.data_bank as u32,
                ControlTransfer => cpu.program_bank as u32,
            }
        };
        let high = cpu.memory.get_byte(pc) as u32;
        let low = cpu.memory.get_byte(pc + 1) as u32;
        let addr: u32 = bank << 16 | high << 8 | low;

        cpu.program_counter += 2;

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

