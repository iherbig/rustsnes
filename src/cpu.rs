use memory::Memory;
use modes::*;
use modes::InstructionType::*;
use std::fmt;

const IS_BYTE: bool = true;

macro_rules! decode_op_and_execute {
    ($op:expr, $this:ident, $mem:ident) => (
        match $op {
            0x00 => {
                let mode = StackInterrupt;
                $this.brk(&mode, $mem);
            },
            0x01 => {
                let mode = DirectPageIndexedIndirectX;
                $this.ora(&mode, $mem);
            },
            0x02 => {
                let mode = StackInterrupt;
                $this.cop(&mode, $mem);
            },
            0x03 => {
                let mode = StackRelative;
                $this.ora(&mode, $mem);
            },
            0x04 => {
                let mode = DirectPage;
                $this.tsb(&mode, $mem);
            },
            0x05 => {
                let mode = DirectPage;
                $this.ora(&mode, $mem);
            },
            0x06 => {
                let mode = DirectPage;
                $this.asl(&mode, $mem);
            },
            0x07 => {
                let mode = DirectPageIndirectLong;
                $this.ora(&mode, $mem);
            },
            0x08 => {
                let mode = StackPush;
                $this.php(&mode, $mem);
            },
            0x09 => {
                let mode = Immediate;
                $this.ora(&mode, $mem);
            },
            0x0A => {
                let mode = Accumulator;
                $this.asl(&mode, $mem);
            },
            0x0B => {
                let mode = StackPush;
                $this.phd(&mode, $mem);
            },
            0x0C => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.tsb(&mode, $mem);
            },
            0x0D => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.ora(&mode, $mem);
            },
            0x0E => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.asl(&mode, $mem);
            },
            0x0F => {
                let mode = AbsoluteLong;
                $this.ora(&mode, $mem);
            },
            0x10 => {
                let mode = ProgramCounterRelative;
                $this.bpl(&mode, $mem);
            },
            0x11 => {
                let mode = DirectPageIndirectIndexedY;
                $this.ora(&mode, $mem);
            },
            0x12 => {
                let mode = DirectPageIndirect;
                $this.ora(&mode, $mem);
            },
            0x13 => {
                let mode = StackRelativeIndirectIndexedY;
                $this.ora(&mode, $mem);
            },
            0x14 => {
                let mode = DirectPage;
                $this.trb(&mode, $mem);
            },
            0x15 => {
                let mode = DirectPageIndexedX;
                $this.ora(&mode, $mem);
            },
            0x16 => {
                let mode = DirectPageIndexedX;
                $this.asl(&mode, $mem);
            },
            0x17 => {
                let mode = DirectPageIndirectLongIndexedY;
                $this.ora(&mode, $mem);
            },
            0x18 => {
                $this.clc();
            },
            0x19 => {
                let mode = AbsoluteIndexedY;
                $this.ora(&mode, $mem);
            },
            0x1A => {
                let mode = Accumulator;
                $this.inc(&mode, $mem);
            },
            0x1B => {
                $this.tcs();
            },
            0x1C => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.trb(&mode, $mem);
            },
            0x1D => {
                let mode = AbsoluteIndexedX;
                $this.ora(&mode, $mem);
            },
            0x1E => {
                let mode = AbsoluteIndexedX;
                $this.asl(&mode, $mem);
            },
            0x1F => {
                let mode = AbsoluteLongIndexedX;
                $this.ora(&mode, $mem);
            },
            0x20 => {
                let mode = StackPush;
                $this.jsr(&mode, $mem, false);
            },
            0x21 => {
                let mode = DirectPageIndexedIndirectX;
                $this.and(&mode, $mem);
            },
            0x22 => {
                let mode = StackPush;
                $this.jsr(&mode, $mem, true);
            },
            0x23 => {
                let mode = StackRelative;
                $this.and(&mode, $mem);
            },
            0x24 => {
                let mode = DirectPage;
                $this.bit(&mode, $mem);
            },
            0x25 => {
                let mode = DirectPage;
                $this.and(&mode, $mem);
            },
            0x26 => {
                let mode = DirectPage;
                $this.rol(&mode, $mem);
            },
            0x27 => {
                let mode = DirectPageIndirectLong;
                $this.and(&mode, $mem);
            },
            0x28 => {
                let mode = StackPull;
                $this.plp(&mode, $mem);
            },
            0x29 => {
                let mode = Immediate;
                $this.and(&mode, $mem);
            },
            0x2A => {
                let mode = Accumulator;
                $this.rol(&mode, $mem);
            },
            0x2B => {
                let mode = StackPull;
                $this.pld(&mode, $mem);
            },
            0x2C => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.bit(&mode, $mem);
            },
            0x2D => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.and(&mode, $mem);
            },
            0x2E => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.rol(&mode, $mem);
            },
            0x2F => {
                let mode = AbsoluteLong;
                $this.and(&mode, $mem);
            },
            0x30 => {
                let mode = ProgramCounterRelative;
                $this.bmi(&mode, $mem);
            },
            0x31 => {
                let mode = DirectPageIndirectIndexedY;
                $this.and(&mode, $mem);
            },
            0x32 => {
                let mode = DirectPageIndirect;
                $this.and(&mode, $mem);
            },
            0x33 => {
                let mode = StackRelativeIndirectIndexedY;
                $this.and(&mode, $mem);
            },
            0x34 => {
                let mode = DirectPageIndexedX;
                $this.bit(&mode, $mem);
            },
            0x35 => {
                let mode = DirectPageIndexedX;
                $this.and(&mode, $mem);
            },
            0x36 => {
                let mode = DirectPageIndexedX;
                $this.rol(&mode, $mem);
            },
            0x37 => {
                let mode = DirectPageIndirectLongIndexedY;
                $this.and(&mode, $mem);
            },
            0x38 => {
                $this.sec();
            },
            0x39 => {
                let mode = AbsoluteIndexedY;
                $this.and(&mode, $mem);
            },
            0x3A => {
                let mode = Accumulator;
                $this.dec(&mode, $mem);
            },
            0x3B => {
                $this.tsc();
            },
            0x3C => {
                let mode = AbsoluteIndexedX;
                $this.bit(&mode, $mem);
            },
            0x3D => {
                let mode = AbsoluteIndexedX;
                $this.and(&mode, $mem);
            },
            0x3E => {
                let mode = AbsoluteIndexedX;
                $this.rol(&mode, $mem);
            },
            0x3F => {
                let mode = AbsoluteLongIndexedX;
                $this.and(&mode, $mem);
            },
            0x40 => {
                let mode = StackRTI;
                $this.rti(&mode, $mem);
            },
            0x41 => {
                let mode = DirectPageIndexedIndirectX;
                $this.eor(&mode, $mem);
            },
            0x42 => {
                $this.wdm();
            },
            0x43 => {
                let mode = StackRelative;
                $this.eor(&mode, $mem);
            },
            0x44 => {
                let mode = BlockMove;
                $this.mvp(&mode, $mem);
            },
            0x45 => {
                let mode = DirectPage;
                $this.eor(&mode, $mem);
            },
            0x46 => {
                let mode = DirectPage;
                $this.lsr(&mode, $mem);
            },
            0x47 => {
                let mode = DirectPageIndirectLong;
                $this.eor(&mode, $mem);
            },
            0x48 => {
                let mode = StackPush;
                $this.pha(&mode, $mem);
            },
            0x49 => {
                let mode = Immediate;
                $this.eor(&mode, $mem);
            },
            0x4A => {
                let mode = Accumulator;
                $this.lsr(&mode, $mem);
            },
            0x4B => {
                let mode = StackPush;
                $this.phk(&mode, $mem);
            },
            0x4C => {
                let mode = Absolute { instruction_type: ControlTransfer };
                $this.jmp(&mode, $mem);
            },
            0x4D => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.eor(&mode, $mem);
            },
            0x4E => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.lsr(&mode, $mem);
            },
            0x4F => {
                let mode = AbsoluteLong;
                $this.eor(&mode, $mem);
            },
            0x50 => {
                let mode = ProgramCounterRelative;
                $this.bvc(&mode, $mem);
            },
            0x51 => {
                let mode = DirectPageIndirectIndexedY;
                $this.eor(&mode, $mem);
            },
            0x52 => {
                let mode = DirectPageIndirect;
                $this.eor(&mode, $mem);
            },
            0x53 => {
                let mode = StackRelativeIndirectIndexedY;
                $this.eor(&mode, $mem);
            },
            0x54 => {
                let mode = BlockMove;
                $this.mvn(&mode, $mem);
            },
            0x55 => {
                let mode = DirectPageIndexedX;
                $this.eor(&mode, $mem);
            },
            0x56 => {
                let mode = DirectPageIndexedX;
                $this.lsr(&mode, $mem);
            },
            0x57 => {
                let mode = DirectPageIndirectLongIndexedY;
                $this.eor(&mode, $mem);
            },
            0x58 => {
                $this.cli();
            },
            0x59 => {
                let mode = AbsoluteIndexedY;
                $this.eor(&mode, $mem);
            },
            0x5A => {
                let mode = StackPush;
                $this.phy(&mode, $mem);
            },
            0x5B => {
                $this.tcd();
            },
            0x5C => {
                let mode = AbsoluteLong;
                $this.jmp(&mode, $mem);
            },
            0x5D => {
                let mode = AbsoluteIndexedX;
                $this.eor(&mode, $mem);
            },
            0x5E => {
                let mode = AbsoluteIndexedX;
                $this.lsr(&mode, $mem);
            },
            0x5F => {
                let mode = AbsoluteLongIndexedX;
                $this.eor(&mode, $mem);
            },
            0x60 => {
                let mode = StackRTS;
                $this.rts(&mode, $mem);
            },
            0x61 => {
                let mode = DirectPageIndexedIndirectX;
                $this.adc(&mode, $mem);
            },
            0x62 => {
                let mode = StackProgramCounterRelative;
                $this.per(&mode, $mem);
            },
            0x63 => {
                let mode = StackRelative;
                $this.adc(&mode, $mem);
            },
            0x64 => {
                let mode = DirectPage;
                $this.stz(&mode, $mem);
            },
            0x65 => {
                let mode = DirectPage;
                $this.adc(&mode, $mem);
            },
            0x66 => {
                let mode = DirectPage;
                $this.ror(&mode, $mem);
            },
            0x67 => {
                let mode = DirectPageIndirectLong;
                $this.adc(&mode, $mem);
            },
            0x68 => {
                let mode = StackPull;
                $this.pla(&mode, $mem);
            },
            0x69 => {
                let mode = Immediate;
                $this.adc(&mode, $mem);
            },
            0x6A => {
                let mode = Accumulator;
                $this.ror(&mode, $mem);
            },
            0x6B => {
                let mode = StackRTL;
                $this.rtl(&mode, $mem);
            },
            0x6C => {
                let mode = AbsoluteIndirect;
                $this.jmp(&mode, $mem);
            },
            0x6D => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.adc(&mode, $mem);
            },
            0x6E => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.ror(&mode, $mem);
            },
            0x6F => {
                let mode = AbsoluteLong;
                $this.adc(&mode, $mem);
            },
            0x70 => {
                let mode = ProgramCounterRelative;
                $this.bvs(&mode, $mem);
            },
            0x71 => {
                let mode = DirectPageIndirectIndexedY;
                $this.adc(&mode, $mem);
            },
            0x72 => {
                let mode = DirectPageIndirect;
                $this.adc(&mode, $mem);
            },
            0x73 => {
                let mode = StackRelativeIndirectIndexedY;
                $this.adc(&mode, $mem);
            },
            0x74 => {
                let mode = StackRelativeIndirectIndexedY;
                $this.stz(&mode, $mem);
            },
            0x75 => {
                let mode = DirectPageIndexedX;
                $this.adc(&mode, $mem);
            },
            0x76 => {
                let mode = DirectPageIndexedX;
                $this.ror(&mode, $mem);
            },
            0x77 => {
                let mode = DirectPageIndirectLongIndexedY;
                $this.adc(&mode, $mem);
            },
            0x78 => {
                $this.sei();
            },
            0x79 => {
                let mode = AbsoluteIndexedY;
                $this.adc(&mode, $mem);
            },
            0x7A => {
                let mode = StackPull;
                $this.ply(&mode, $mem);
            },
            0x7B => {
                $this.tdc();
            },
            0x7C => {
                let mode = AbsoluteIndexedIndirect;
                $this.jmp(&mode, $mem);
            },
            0x7D => {
                let mode = AbsoluteIndexedX;
                $this.adc(&mode, $mem);
            },
            0x7E => {
                let mode = AbsoluteIndexedX;
                $this.ror(&mode, $mem);
            },
            0x7F => {
                let mode = AbsoluteLongIndexedX;
                $this.adc(&mode, $mem);
            },
            0x80 => {
                let mode = ProgramCounterRelative;
                $this.bra(&mode, $mem);
            },
            0x81 => {
                let mode = DirectPageIndexedIndirectX;
                $this.sta(&mode, $mem);
            },
            0x82 => {
                let mode = ProgramCounterRelativeLong;
                $this.brl(&mode, $mem);
            },
            0x83 => {
                let mode = StackRelative;
                $this.sta(&mode, $mem);
            },
            0x84 => {
                let mode = DirectPage;
                $this.sty(&mode, $mem);
            },
            0x85 => {
                let mode = DirectPage;
                $this.sta(&mode, $mem);
            },
            0x86 => {
                let mode = DirectPage;
                $this.stx(&mode, $mem);
            },
            0x87 => {
                let mode = DirectPageIndirectLong;
                $this.sta(&mode, $mem);
            },
            0x88 => {
                $this.dey();
            },
            0x89 => {
                let mode = Immediate;
                $this.bit(&mode, $mem);
            },
            0x8A => {
                $this.txa();
            },
            0x8B => {
                let mode = StackPush;
                $this.phb(&mode, $mem);
            },
            0x8C => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.sty(&mode, $mem);
            },
            0x8D => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.sta(&mode, $mem);
            },
            0x8E => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.stx(&mode, $mem);
            },
            0x8F => {
                let mode = AbsoluteLong;
                $this.sta(&mode, $mem);
            },
            0x90 => {
                let mode = ProgramCounterRelative;
                $this.bcc(&mode, $mem);
            },
            0x91 => {
                let mode = DirectPageIndirectIndexedY;
                $this.sta(&mode, $mem);
            },
            0x92 => {
                let mode = DirectPageIndirect;
                $this.sta(&mode, $mem);
            },
            0x93 => {
                let mode = StackRelativeIndirectIndexedY;
                $this.sta(&mode, $mem);
            },
            0x94 => {
                let mode = DirectPageIndexedX;
                $this.sty(&mode, $mem);
            },
            0x95 => {
                let mode = DirectPageIndexedX;
                $this.sta(&mode, $mem);
            },
            0x96 => {
                let mode = DirectPageIndexedY;
                $this.stx(&mode, $mem);
            },
            0x97 => {
                let mode = DirectPageIndirectLongIndexedY;
                $this.sta(&mode, $mem);
            },
            0x98 => {
                $this.tya();
            },
            0x99 => {
                let mode = AbsoluteIndexedY;
                $this.sta(&mode, $mem);
            },
            0x9A => {
                $this.txs();
            },
            0x9B => {
                $this.txy();
            },
            0x9C => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.stz(&mode, $mem);
            },
            0x9D => {
                let mode = AbsoluteIndexedX;
                $this.sta(&mode, $mem);
            },
            0x9E => {
                let mode = AbsoluteIndexedX;
                $this.stz(&mode, $mem);
            },
            0x9F => {
                let mode = AbsoluteLongIndexedX;
                $this.sta(&mode, $mem);
            },
            0xA0 => {
                let mode = Immediate;
                $this.ldy(&mode, $mem);
            },
            0xA1 => {
                let mode = DirectPageIndexedIndirectX;
                $this.lda(&mode, $mem);
            },
            0xA2 => {
                let mode = Immediate;
                $this.ldx(&mode, $mem);
            },
            0xA3 => {
                let mode = StackRelative;
                $this.lda(&mode, $mem);
            },
            0xA4 => {
                let mode = DirectPage;
                $this.ldy(&mode, $mem);
            },
            0xA5 => {
                let mode = DirectPage;
                $this.lda(&mode, $mem);
            },
            0xA6 => {
                let mode = DirectPage;
                $this.ldx(&mode, $mem);
            },
            0xA7 => {
                let mode = DirectPageIndirectLong;
                $this.lda(&mode, $mem);
            },
            0xA8 => {
                $this.tay();
            },
            0xA9 => {
                let mode = Immediate;
                $this.lda(&mode, $mem);
            },
            0xAA => {
                $this.tax();
            },
            0xAB => {
                let mode = StackPull;
                $this.plb(&mode, $mem);
            },
            0xAC => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.ldy(&mode, $mem);
            },
            0xAD => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.lda(&mode, $mem);
            },
            0xAE => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.ldx(&mode, $mem);
            },
            0xAF => {
                let mode = AbsoluteLong;
                $this.lda(&mode, $mem);
            },
            0xB0 => {
                let mode = ProgramCounterRelative;
                $this.bcs(&mode, $mem);
            },
            0xB1 => {
                let mode = DirectPageIndirectIndexedY;
                $this.lda(&mode, $mem);
            },
            0xB2 => {
                let mode = DirectPageIndirect;
                $this.lda(&mode, $mem);
            },
            0xB3 => {
                let mode = StackRelativeIndirectIndexedY;
                $this.lda(&mode, $mem);
            },
            0xB4 => {
                let mode = DirectPageIndexedX;
                $this.ldy(&mode, $mem);
            },
            0xB5 => {
                let mode = DirectPageIndexedX;
                $this.lda(&mode, $mem);
            },
            0xB6 => {
                let mode = DirectPageIndexedY;
                $this.ldx(&mode, $mem);
            },
            0xB7 => {
                let mode = DirectPageIndirectLongIndexedY;
                $this.lda(&mode, $mem);
            },
            0xB8 => {
                $this.clv();
            },
            0xB9 => {
                let mode = AbsoluteIndexedY;
                $this.lda(&mode, $mem);
            },
            0xBA => {
                $this.tsx();
            },
            0xBB => {
                $this.tyx();
            },
            0xBC => {
                let mode = AbsoluteIndexedX;
                $this.ldy(&mode, $mem);
            },
            0xBD => {
                let mode = AbsoluteIndexedX;
                $this.lda(&mode, $mem);
            },
            0xBE => {
                let mode = AbsoluteIndexedY;
                $this.ldx(&mode, $mem);
            },
            0xBF => {
                let mode = AbsoluteLongIndexedX;
                $this.lda(&mode, $mem);
            },
            0xC0 => {
                let mode = Immediate;
                $this.cpy(&mode, $mem);
            },
            0xC1 => {
                let mode = DirectPageIndexedIndirectX;
                $this.cmp(&mode, $mem);
            },
            0xC2 => {
                let mode = Immediate;
                $this.rep(&mode, $mem);
            },
            0xC3 => {
                let mode = StackRelative;
                $this.cmp(&mode, $mem);
            },
            0xC4 => {
                let mode = DirectPage;
                $this.cpy(&mode, $mem);
            },
            0xC5 => {
                let mode = DirectPage;
                $this.cmp(&mode, $mem);
            },
            0xC6 => {
                let mode = DirectPage;
                $this.dec(&mode, $mem);
            },
            0xC7 => {
                let mode = DirectPageIndirectLong;
                $this.cmp(&mode, $mem);
            },
            0xC8 => {
                $this.iny();
            },
            0xC9 => {
                let mode = Immediate;
                $this.cmp(&mode, $mem);
            },
            0xCA => {
                $this.dex();
            },
            0xCB => {
                $this.wai();
            },
            0xCC => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.cpy(&mode, $mem);
            },
            0xCD => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.cmp(&mode, $mem);
            },
            0xCE => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.dec(&mode, $mem);
            },
            0xCF => {
                let mode = AbsoluteLong;
                $this.cmp(&mode, $mem);
            },
            0xD0 => {
                let mode = ProgramCounterRelative;
                $this.bne(&mode, $mem);
            },
            0xD1 => {
                let mode = DirectPageIndirectIndexedY;
                $this.cmp(&mode, $mem);
            },
            0xD2 => {
                let mode = DirectPageIndirect;
                $this.cmp(&mode, $mem);
            },
            0xD3 => {
                let mode = StackRelativeIndirectIndexedY;
                $this.cmp(&mode, $mem);
            },
            0xD4 => {
                let mode = StackDirectPageIndirect;
                $this.pei(&mode, $mem);
            },
            0xD5 => {
                let mode = DirectPageIndexedX;
                $this.cmp(&mode, $mem)
            },
            0xD6 => {
                let mode = DirectPageIndexedX;
                $this.dec(&mode, $mem);
            },
            0xD7 => {
                let mode = DirectPageIndirectLongIndexedY;
                $this.cmp(&mode, $mem);
            },
            0xD8 => {
                $this.cld();
            },
            0xD9 => {
                let mode = AbsoluteIndexedY;
                $this.cmp(&mode, $mem);
            },
            0xDA => {
                let mode = StackPush;
                $this.phx(&mode, $mem);
            },
            0xDB => {
                $this.stp();
            },
            0xDC => {
                let mode = AbsoluteIndirectLong;
                $this.jmp(&mode, $mem);
            },
            0xDD => {
                let mode = AbsoluteIndexedX;
                $this.cmp(&mode, $mem);
            },
            0xDE => {
                let mode = AbsoluteIndexedX;
                $this.dec(&mode, $mem);
            },
            0xDF => {
                let mode = AbsoluteLongIndexedX;
                $this.cmp(&mode, $mem);
            },
            0xE0 => {
                let mode = Immediate;
                $this.cpx(&mode, $mem);
            },
            0xE1 => {
                let mode = DirectPageIndexedIndirectX;
                $this.sbc(&mode, $mem);
            },
            0xE2 => {
                let mode = Immediate;
                $this.sep(&mode, $mem);
            },
            0xE3 => {
                let mode = DirectPage;
                $this.sbc(&mode, $mem);
            },
            0xE4 => {
                let mode = DirectPage;
                $this.cpx(&mode, $mem);
            },
            0xE5 => {
                let mode = DirectPage;
                $this.sbc(&mode, $mem);
            },
            0xE6 => {
                let mode = DirectPage;
                $this.inc(&mode, $mem);
            },
            0xE7 => {
                let mode = DirectPageIndirectLong;
                $this.sbc(&mode, $mem);
            },
            0xE8 => {
                $this.inx();
            },
            0xE9 => {
                let mode = Immediate;
                $this.sbc(&mode, $mem);
            },
            0xEA => {
                $this.nop();
            },
            0xEB => {
                $this.xba();
            },
            0xEC => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.cpx(&mode, $mem);
            },
            0xED => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.sbc(&mode, $mem);
            },
            0xEE => {
                let mode = Absolute { instruction_type: LocatingData };
                $this.inc(&mode, $mem);
            },
            0xEF => {
                let mode = AbsoluteLong;
                $this.sbc(&mode, $mem);
            },
            0xF0 => {
                let mode = ProgramCounterRelative;
                $this.beq(&mode, $mem);
            },
            0xF1 => {
                let mode = DirectPageIndirectIndexedY;
                $this.sbc(&mode, $mem);
            },
            0xF2 => {
                let mode = DirectPageIndirect;
                $this.sbc(&mode, $mem);
            },
            0xF3 => {
                let mode = StackRelativeIndirectIndexedY;
                $this.sbc(&mode, $mem);
            },
            0xF4 => {
                let mode = StackAbsolute;
                $this.pea(&mode, $mem);
            },
            0xF5 => {
                let mode = DirectPageIndexedX;
                $this.sbc(&mode, $mem);
            },
            0xF6 => {
                let mode = DirectPageIndexedX;
                $this.inc(&mode, $mem);
            },
            0xF7 => {
                let mode = DirectPageIndirectLongIndexedY;
                $this.sbc(&mode, $mem);
            },
            0xF8 => {
                $this.sed();
            },
            0xF9 => {
                let mode = AbsoluteIndexedY;
                $this.sbc(&mode, $mem);
            },
            0xFA => {
                let mode = StackPull;
                $this.plx(&mode, $mem);
            },
            0xFB => {
                $this.xce();
            },
            0xFC => {
                let mode = AbsoluteIndexedIndirect;
                $this.jsr(&mode, $mem, false);
            },
            0xFD => {
                let mode = AbsoluteIndexedX;
                $this.sbc(&mode, $mem);
            },
            0xFE => {
                let mode = AbsoluteIndexedX;
                $this.inc(&mode, $mem);
            },
            0xFF => {
                let mode = AbsoluteLongIndexedX;
                $this.sbc(&mode, $mem);
            },
            _ => panic!("{} is not an opcode", $op),
        }
    );
}

pub struct CPU {
    accumulator:                        u16,
    index_x:                            u16,
    index_y:                            u16,
    pub stack_pointer:                usize,
    pub data_bank:                    usize,
    pub direct_page:                  usize,
    pub program_bank:                 usize,
    pub processor_status:   ProcessorStatus,
    pub program_counter:              usize,
    emulation_mode:                    bool,
}

impl CPU {
	pub fn new(memory: &Memory) -> CPU {
        use memory::RomType::*;

        let pc = memory.rom.reset_vector;
        let pb = match memory.rom.rom_type {
            LoROM | FastLoROM => 0x80,
            HiROM | FastHiROM => 0xC0,
            _ => unreachable!(),
        };

        CPU {
            accumulator:                         0,
            index_x:                             0,
            index_y:                             0,
            stack_pointer:                       0,
            data_bank:                           0,
            direct_page:                         0,
            program_bank:                       pb,
            processor_status:   Default::default(),
            program_counter:                    pc,
            emulation_mode:                   true,
        }
	}

    pub fn run(&mut self, memory: &mut Memory) {
        loop {
            println!("{:?}\n{:?}", self, memory);
            self.run_instruction(memory);
        }
    }

    fn run_instruction(&mut self, memory: &mut Memory) {
        let opcode = memory.get_byte((self.program_bank << 16) | self.program_counter);
        println!("opcode {:x}", opcode);

        self.program_counter += 1;
        decode_op_and_execute!(opcode, self, memory);
    }

    fn brk<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("brk unimplemented")
    }

    fn ora<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("ora unimplemented")
    }

    fn cop<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("cop unimplemented")
    }

    fn tsb<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("tsb unimplemented")
    }

    fn asl<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("asl unimplemented")
    }

    fn php<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("php unimplemented")
    }

    fn phd<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("phd unimplemented")
    }

    fn bpl<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        use self::StatusFlags::Negative;
        
        if !self.processor_status.get_flag(Negative) {
            let data = mode.load(self, memory, IS_BYTE) as i8;

            if data < 0 {
                self.program_counter -= !(data as usize) + 1;
            } else {
                self.program_counter += data as usize;
            }
        }

        self.program_counter += 1;
    }

    fn trb<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("trb unimplemented")
    }

    fn clc(&mut self) {
        use self::StatusFlags::Carry;

        self.processor_status.set_flag(Carry, false);
    }

    fn inc<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("inc unimplemented")
    }

    fn tcs(&mut self) {
        panic!("tcs unimplemented")
    }

    // The mode cannot be used in this function. Despite that the opcode may specify
    // Absolute, Absolute Long, or Absolute Indexed Indirect X the actual value being
    // assigned to PC and PB (in the case of long) are the immediate values specified
    // by the operand rather than the operand being treated as an effective address.
    fn jsr<T: Instruction>(&mut self, mode: &T, memory: &mut Memory, is_long: bool) {

        // PC starts just past opcode at this point, but before we push it onto the stack
        // it must be pointing at the last byte of the operand, which is either two or three
        // past the opcode depending on the addressing mode
        let pc = if is_long { self.program_counter + 2 } else { self.program_counter + 1 };

        if is_long {
            let pb = self.program_bank;
            mode.store(self, memory, IS_BYTE, pb);
        }

        mode.store(self, memory, !IS_BYTE, pc);

        let (bank, addr) = {
            let tmp_addr = (self.program_bank << 16) + self.program_counter;
            let low = memory.get_byte(tmp_addr) as usize;
            let high = memory.get_byte(tmp_addr + 1) as usize;
            let bank = if is_long { memory.get_byte(tmp_addr + 2) as usize } else { self.program_bank };

            (bank, (high << 8) | low)
        };

        self.program_counter = addr;
        
        if is_long {
            self.program_bank = bank;
        }
    }

    fn and<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("and unimplemented")
    }

    fn bit<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("bit unimplemented")
    }

    fn rol<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("rol unimplemented")
    }

    fn plp<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("plp unimplemented")
    }

    fn pld<T: Instruction>(&mut self, mode: &T, memory: &Memory) {
        use self::StatusFlags::{Zero, Negative};

        let data = mode.load(self, memory, !IS_BYTE);
        self.direct_page = data;

        self.processor_status.set_flag(Negative, data.rotate_left(1) & 1 == 1);
        self.processor_status.set_flag(Zero, self.direct_page == 0);
    }

    fn bmi<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("bmi unimplemented")
    }

    fn sec(&mut self) {
        panic!("sec unimplemented")
    }

    fn dec<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("dec unimplemented")
    }

    fn tsc(&mut self) {
        panic!("tsc unimplemented")
    }

    fn rti<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("rti unimplemented")
    }

    fn eor<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("eor unimplemented")
    }

    fn wdm(&mut self) {
        panic!("wdm unimplemented")
    }

    fn mvp<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("mvp unimplemented")
    }

    fn lsr<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("lsr unimplemented")
    }

    fn pha<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        use self::StatusFlags::AccumulatorRegisterSize;

        let data = self.accumulator as usize;

        let emu = self.processor_status.status[AccumulatorRegisterSize as usize];
        mode.store(self, memory, emu, data);
    }

    fn phk<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        let data = self.program_bank;

        mode.store(self, memory, IS_BYTE, data);
    }

    fn jmp<T: Instruction>(&mut self, mode: &T, memory: &Memory) {
        let jump_addr = mode.load(self, memory, !IS_BYTE) as usize;
        println!("jump addr {:x}", jump_addr);
        self.program_counter = jump_addr & 0x00FFFF;
        self.program_bank = (jump_addr & 0xFF0000) >> 16;
    }

    fn bvc<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("bvc unimplemented")
    }

    fn mvn<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("mvn unimplemented")
    }

    fn cli(&mut self) {
        panic!("cli unimplemented")
    }

    fn phy<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("phy unimplemented")
    }

    fn tcd(&mut self) {
        use self::StatusFlags::{Negative, Zero};

        let data = self.accumulator as usize;
        self.direct_page = data;

        self.processor_status.set_flag(Negative, data.rotate_left(1) & 1 == 1);
        self.processor_status.set_flag(Zero, self.accumulator == 0);
    }

    fn rts<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        let addr = mode.load(self, memory, !IS_BYTE);

        self.program_counter = addr + 1;
    }

    fn adc<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("adc unimplemented")
    }

    fn per<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("per unimplemented")
    }

    fn stz<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        use self::StatusFlags::AccumulatorRegisterSize;

        let emu = self.processor_status.status[AccumulatorRegisterSize as usize];

        mode.store(self, memory, emu, 0);
    }

    fn ror<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("ror unimplemented")
    }

    fn pla<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("pla unimplemented")
    }

    fn rtl<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("rtl unimplemented")
    }

    fn bvs<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("bvs unimplemented")
    }

    fn sei(&mut self) {
        use self::StatusFlags::IRQDisable;

        self.processor_status.set_flag(IRQDisable, true);
    }

    fn ply<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("ply unimplemented")
    }

    fn tdc(&mut self) {
        use self::StatusFlags::{Negative, Zero};
        let data = self.direct_page;
        self.accumulator = self.direct_page as u16;

        self.processor_status.set_flag(Negative, data.rotate_left(1) & 1 == 1);
        self.processor_status.set_flag(Zero, self.accumulator == 0);
    }

    fn bra<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("bra unimplemented")
    }

    fn sta<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        use self::StatusFlags::AccumulatorRegisterSize;

        let data = self.accumulator as usize;

        let emu = self.processor_status.get_flag(AccumulatorRegisterSize);
        mode.store(self, memory, emu, data);
    }

    fn brl<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("brl unimplemented")
    }

    fn sty<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("sty unimplemented")
    }

    fn stx<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        use self::StatusFlags::IndexRegisterSize;

        let data = self.index_x as usize;

        let emu = self.processor_status.get_flag(IndexRegisterSize);
        mode.store(self, memory, emu, data);
    }

    fn dey(&mut self) {
        panic!("dey unimplemented")
    }

    fn txa(&mut self) {
        panic!("txa unimplemented")
    }

    fn phb<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("phb unimplemented")
    }

    fn bcc<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("bcc unimplemented")
    }

    fn tya(&mut self) {
        panic!("tya unimplemented")
    }

    fn txs(&mut self) {
        use self::StatusFlags::IndexRegisterSize;

        if self.emulation_mode || self.processor_status.status[IndexRegisterSize as usize] {
            self.stack_pointer = (0xFF & self.index_x) as usize;
        } else {
            self.stack_pointer = self.index_x as usize;
        }
    }

    fn txy(&mut self) {
        panic!("txy unimplemented")
    }

    fn ldy<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("ldy unimplemented")
    }

    fn lda<T: Instruction>(&mut self, mode: &T, memory: &Memory) {
        use self::StatusFlags::{AccumulatorRegisterSize, Negative, Zero};

        let emu = self.processor_status.status[AccumulatorRegisterSize as usize];
        let data = mode.load(self, memory, emu);
        self.accumulator = data as u16;
        
        self.processor_status.set_flag(Negative, data.rotate_left(1) & 1 == 1);
        self.processor_status.set_flag(Zero, self.accumulator == 0);
    }

    fn ldx<T: Instruction>(&mut self, mode: &T, memory: &Memory) {
        use self::StatusFlags::{IndexRegisterSize, Zero, Negative};

        let emu = self.processor_status.status[IndexRegisterSize as usize];
        let data = mode.load(self, memory, emu);
        self.index_x = data as u16;

        self.processor_status.set_flag(Negative, data.rotate_left(1) & 1 == 1);
        self.processor_status.set_flag(Zero, self.index_x == 0);
    }

    fn tay(&mut self) {
        panic!("tay unimplemented")
    }

    fn tax(&mut self) {
        use self::StatusFlags::{Negative, Zero, IndexRegisterSize};

        self.index_x = self.accumulator;

        if self.processor_status.get_flag(IndexRegisterSize) {
            self.index_x &= 0x00FF;
        }

        let data = self.index_x as u8;

        self.processor_status.set_flag(Negative, data.rotate_left(1) & 1 == 1);
        self.processor_status.set_flag(Zero, self.index_x == 0);
    }

    fn plb<T: Instruction>(&mut self, mode: &T, memory: &Memory) {
        use self::StatusFlags::{Negative, Zero};

        let data = mode.load(self, memory, IS_BYTE);
        self.data_bank = data;

        self.processor_status.set_flag(Negative, data.rotate_left(1) & 1 == 1);
        self.processor_status.set_flag(Zero, self.data_bank == 0);
    }

    fn bcs<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("bcs unimplemented")
    }

    fn clv(&mut self) {
        panic!("clv unimplemented")
    }

    fn tsx(&mut self) {
        panic!("tsx unimplemented")
    }

    fn tyx(&mut self) {
        panic!("tyx unimplemented")
    }

    fn cpy<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("cpy unimplemented")
    }

    fn cmp<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("cmp unimplemented")
    }

    fn rep<T: Instruction>(&mut self, mode: &T, memory: &Memory) {
        let val = mode.load(self, memory, IS_BYTE);

        self.processor_status.set_by_byte(self.emulation_mode, val as u8, false);
    }

    fn iny(&mut self) {
        panic!("iny unimplemented")
    }

    fn dex(&mut self) {
        use self::StatusFlags::{Negative, Zero};
        
        self.index_x -= 1;
        let data = self.index_x;

        self.processor_status.set_flag(Negative, data.rotate_left(1) & 1 == 1);
        self.processor_status.set_flag(Zero, self.index_x == 0);
    }

    fn wai(&mut self) {
        panic!("wai unimplemented")
    }

    fn bne<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        use self::StatusFlags::Zero;

        if !self.processor_status.get_flag(Zero) {
            let data = mode.load(self, memory, IS_BYTE) as i8;

            if data < 0 {
                self.program_counter -= !(data as usize) + 1;
            } else {
                self.program_counter += data as usize;
            }
        }

        self.program_counter += 1;
    }

    fn pei<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("pei unimplemented")
    }

    fn cld(&mut self) {
        panic!("cld unimplemented")
    }

    fn phx<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        use self::StatusFlags::IndexRegisterSize;

        let data = self.index_x as usize;

        let emu = self.processor_status.status[IndexRegisterSize as usize];
        mode.store(self, memory, emu, data);
    }

    fn stp(&mut self) {
        panic!("stp unimplemented")
    }

    fn cpx<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("cpx unimplemented")
    }

    fn sbc<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("sbc unimplemented")
    }

    fn inx(&mut self) {
        panic!("inx unimplemented")
    }

    fn nop(&mut self) {
        panic!("nop unimplemented")
    }

    fn xba(&mut self) {
        panic!("xba unimplemented")
    }

    fn beq<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("beq unimplemented")
    }

    fn pea<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("pea unimplemented")
    }

    fn sed(&mut self) {
        panic!("sed unimplemented")
    }

    fn plx<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("plx unimplemented")
    }

    fn xce(&mut self) {
        use self::StatusFlags::Carry;

        let emu_bit = self.emulation_mode;
        self.emulation_mode = self.processor_status.status[Carry as usize];
        self.processor_status.status[Carry as usize] = emu_bit;
    }

    fn sep<T: Instruction>(&mut self, mode: &T, memory: &Memory) {
        let val = mode.load(self, memory, IS_BYTE);
        
        self.processor_status.set_by_byte(self.emulation_mode, val as u8, true);
    }
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CPU {{
      accumulator: {:x},
      index_x: {:x},
      index_y: {:x},
      stack_pointer: {:x},
      data_bank: {:x},
      direct_page: {:x},
      program_bank: {:x},
      processor_status: {:?},
      program_counter: {:x},
      emulation_mode: {}
    }}",
                  self.accumulator, self.index_x, self.index_y, self.stack_pointer,
                  self.data_bank, self.direct_page, self.program_bank, self.processor_status,
                  self.program_counter, self.emulation_mode)
    }
}

#[derive(Default, Debug)]
pub struct ProcessorStatus {
    status: [bool; 8],
}

impl ProcessorStatus {
    pub fn set_flag(&mut self, flag: StatusFlags, val: bool) {
        self.status[flag as usize] = val;
    }

    pub fn set_by_byte(&mut self, emulation_mode: bool, byte: u8, val: bool) {
        for i in 0..8 {
            if byte & (1 << i) != 0 {
                if !emulation_mode {
                    self.status[i] = val;
                } else if i != 4 && i != 5 {
                    self.status[i] = val;
                }
            }
        }
    }

    pub fn get_flag(&self, flag: StatusFlags) -> bool {
        self.status[flag as usize]
    }
}

pub enum StatusFlags {
    Carry = 0,
    Zero = 1,
    IRQDisable = 2,
    Decimal = 3,
    IndexRegisterSize = 4,
    AccumulatorRegisterSize = 5,
    Overflow = 6,
    Negative = 7,
}

