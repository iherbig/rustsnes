use memory::{Memory, LOROM_EMU_MODE_VECTORS, HIROM_EMU_MODE_VECTORS};
use modes::*;
use modes::InstructionType::*;

macro_rules! decode_op_and_execute {
    ($op:expr, $this:ident) => (
        match $op {
            0x00 => {
                let mode = Box::new(StackInterrupt);
                $this.brk(mode);
            },
            0x01 => {
                let mode = Box::new(DirectPageIndexedIndirectX);
                $this.ora(mode);
            },
            0x02 => {
                let mode = Box::new(StackInterrupt);
                $this.cop(mode);
            },
            0x03 => {
                let mode = Box::new(StackRelative);
                $this.ora(mode);
            },
            0x04 => {
                let mode = Box::new(DirectPage);
                $this.tsb(mode);
            },
            0x05 => {
                let mode = Box::new(DirectPage);
                $this.ora(mode);
            },
            0x06 => {
                let mode = Box::new(DirectPage);
                $this.asl(mode);
            },
            0x07 => {
                let mode = Box::new(DirectPageIndirectLong);
                $this.ora(mode);
            },
            0x08 => {
                let mode = Box::new(StackPush);
                $this.php(mode);
            },
            0x09 => {
                let mode = Box::new(Immediate);
                $this.ora(mode);
            },
            0x0A => {
                let mode = Box::new(Accumulator);
                $this.asl(mode);
            },
            0x0B => {
                let mode = Box::new(StackPush);
                $this.phd(mode);
            },
            0x0C => {
                let mode = Box::new(Absolute { instruction_type: LocatingData });
                $this.tsb(mode);
            },
            0x0D => {
                let mode = Box::new(Absolute { instruction_type: LocatingData });
                $this.ora(mode);
            },
            0x0E => {
                let mode = Box::new(Absolute { instruction_type: LocatingData });
                $this.asl(mode);
            },
            0x0F => {
                let mode = Box::new(AbsoluteLong);
                $this.ora(mode);
            },
            0x10 => {
                let mode = Box::new(ProgramCounterRelative);
                $this.blp(mode);
            },
            0x11 => {
                let mode = Box::new(DirectPageIndirectIndexedY);
                $this.ora(mode);
            },
            0x12 => {
                let mode = Box::new(DirectPageIndirect);
                $this.ora(mode);
            },
            0x13 => {
                let mode = Box::new(StackRelativeIndirectIndexedY);
                $this.ora(mode);
            },
            0x14 => {
                let mode = Box::new(DirectPage);
                $this.trb(mode);
            },
            0x15 => {
                let mode = Box::new(DirectPageIndexedX);
                $this.ora(mode);
            },
            0x16 => {
                let mode = Box::new(DirectPageIndexedX);
                $this.asl(mode);
            },
            0x17 => {
                let mode = Box::new(DirectPageIndirectLongIndexedY);
                $this.ora(mode);
            },
            0x18 => {
                $this.clc();
            },
            0x19 => {
                let mode = Box::new(AbsoluteIndexedY);
                $this.ora(mode);
            },
            0x1A => {
                let mode = Box::new(Accumulator);
                $this.inc(mode);
            },
            0x1B => {
                $this.tcs();
            },
            0x1C => {
                let mode = Box::new(Absolute { instruction_type: LocatingData });
                $this.trb(mode);
            },
            0x1D => {
                let mode = Box::new(AbsoluteIndexedX);
                $this.ora(mode);
            },
            0x1E => {
                let mode = Box::new(AbsoluteIndexedX);
                $this.asl(mode);
            },
            0x1F => {
                let mode = Box::new(AbsoluteLongIndexedX);
                $this.ora(mode);
            },
            0x20 => {
                let mode = Box::new(Absolute { instruction_type: ControlTransfer });
                $this.jsr(mode);
            },
            0x21 => {
                let mode = Box::new(DirectPageIndexedIndirectX);
                $this.and(mode);
            },
            0x22 => {
                let mode = Box::new(AbsoluteLong);
                $this.jsr(mode);
            },
            0x23 => {
                let mode = Box::new(StackRelative);
                $this.and(mode);
            },
            0x24 => {
                let mode = Box::new(DirectPage);
                $this.bit(mode);
            },
            0x25 => {
                let mode = Box::new(DirectPage);
                $this.and(mode);
            },
            0x26 => {
                let mode = Box::new(DirectPage);
                $this.rol(mode);
            },
            0x27 => {
                let mode = Box::new(DirectPageIndirectLong);
                $this.and(mode);
            },
            0x28 => {
                let mode = Box::new(StackPull);
                $this.plp(mode);
            },
            0x29 => {
                let mode = Box::new(Immediate);
                $this.and(mode);
            },
            0x2A => {
                let mode = Box::new(Accumulator);
                $this.rol(mode);
            },
            0x2B => {
                let mode = Box::new(StackPull);
                $this.pld(mode);
            },
            0x2C => {
                let mode = Box::new(Absolute { instruction_type: LocatingData });
                $this.bit(mode);
            },
            0x2D => {
                let mode = Box::new(Absolute { instruction_type: LocatingData });
                $this.and(mode);
            },
            0x2E => {
                let mode = Box::new(Absolute { instruction_type: LocatingData });
                $this.rol(mode);
            },
            0x2F => {
                let mode = Box::new(AbsoluteLong);
                $this.and(mode);
            },
            0x30 => {
                let mode = Box::new(ProgramCounterRelative);
                $this.bmi(mode);
            },
            0x31 => {
                let mode = Box::new(DirectPageIndirectIndexedY);
                $this.and(mode);
            },
            0x32 => {
                let mode = Box::new(DirectPageIndirect);
                $this.and(mode);
            },
            0x33 => {
                let mode = Box::new(StackRelativeIndirectIndexedY);
                $this.and(mode);
            },
            0x34 => {
                let mode = Box::new(DirectPageIndexedX);
                $this.bit(mode);
            },
            0x35 => {
                let mode = Box::new(DirectPageIndexedX);
                $this.and(mode);
            },
            0x36 => {
                let mode = Box::new(DirectPageIndexedX);
                $this.rol(mode);
            },
            0x37 => {
                let mode = Box::new(DirectPageIndirectLongIndexedY);
                $this.and(mode);
            },
            0x38 => {
                $this.sec();
            },
            0x39 => {
                let mode = Box::new(AbsoluteIndexedY);
                $this.and(mode);
            },
            0x3A => {
                let mode = Box::new(Accumulator);
                $this.dec(mode);
            },
            0x3B => {
                $this.tsc();
            },
            0x3C => {
                let mode = Box::new(AbsoluteIndexedX);
                $this.bit(mode);
            },
            0x3D => {
                let mode = Box::new(AbsoluteIndexedX);
                $this.and(mode);
            },
            0x3E => {
                let mode = Box::new(AbsoluteIndexedX);
                $this.rol(mode);
            },
            0x3F => {
                let mode = Box::new(AbsoluteLongIndexedX);
                $this.and(mode);
            },
            0x40 => {
                let mode = Box::new(StackRTI);
                $this.rti(mode);
            },
            0x41 => {
                let mode = Box::new(DirectPageIndexedIndirectX);
                $this.eor(mode);
            },
            0x42 => {
                $this.wdm();
            },
            0x43 => {
                let mode = Box::new(StackRelative);
                $this.eor(mode);
            },
            0x44 => {
                let mode = Box::new(BlockMove);
                $this.mvp(mode);
            },
            0x45 => {
                let mode = Box::new(DirectPage);
                $this.eor(mode);
            },
            0x46 => {
                let mode = Box::new(DirectPage);
                $this.lsr(mode);
            },
            0x47 => {
                let mode = Box::new(DirectPageIndirectLong);
                $this.eor(mode);
            },
            0x48 => {
                let mode = Box::new(StackPush);
                $this.pha(mode);
            },
            0x49 => {
                let mode = Box::new(Immediate);
                $this.eor(mode);
            },
            0x4A => {
                let mode = Box::new(Accumulator);
                $this.lsr(mode);
            },
            0x4B => {
                let mode = Box::new(StackPush);
                $this.phk(mode);
            },
            0x4C => {
                let mode = Box::new(Absolute { instruction_type: ControlTransfer });
                $this.jmp(mode);
            },
            0x4D => {
                let mode = Box::new(Absolute { instruction_type: LocatingData });
                $this.eor(mode);
            },
            0x4E => {
                let mode = Box::new(Absolute { instruction_type: LocatingData });
                $this.lsr(mode);
            },
            0x4F => {
                let mode = Box::new(AbsoluteLong);
                $this.eor(mode);
            },
            0x50 => {
                let mode = Box::new(ProgramCounterRelative);
                $this.bvc(mode);
            },
            0x51 => {
                let mode = Box::new(DirectPageIndirectIndexedY);
                $this.eor(mode);
            },
            0x52 => {
                let mode = Box::new(DirectPageIndirect);
                $this.eor(mode);
            },
            0x53 => {
                let mode = Box::new(StackRelativeIndirectIndexedY);
                $this.eor(mode);
            },
            0x54 => {
                let mode = Box::new(BlockMove);
                $this.mvn(mode);
            },
            0x55 => {
                let mode = Box::new(DirectPageIndexedX);
                $this.eor(mode);
            },
            0x56 => {
                let mode = Box::new(DirectPageIndexedX);
                $this.lsr(mode);
            },
            0x57 => {
                let mode = Box::new(DirectPageIndirectLongIndexedY);
                $this.eor(mode);
            },
            0x58 => {
                $this.cli();
            },
            0x59 => {
                let mode = Box::new(AbsoluteIndexedY);
                $this.eor(mode);
            },
            0x5A => {
                let mode = Box::new(StackPush);
                $this.phy(mode);
            },
            0x5B => {
                $this.tcd();
            },
            0x5C => {
                let mode = Box::new(AbsoluteLong);
                $this.jmp(mode);
            },
            0x5D => {
                let mode = Box::new(AbsoluteIndexedX);
                $this.eor(mode);
            },
            0x5E => {
                let mode = Box::new(AbsoluteIndexedX);
                $this.lsr(mode);
            },
            0x5F => {
                let mode = Box::new(AbsoluteLongIndexedX);
                $this.eor(mode);
            },
            0x60 => {
                let mode = Box::new(StackRTS);
                $this.rts(mode);
            },
            0x61 => {
                let mode = Box::new(DirectPageIndexedIndirectX);
                $this.adc(mode);
            },
            0x62 => {
                let mode = Box::new(StackProgramCounterRelative);
                $this.per(mode);
            },
            0x63 => {
                let mode = Box::new(StackRelative);
                $this.adc(mode);
            },
            0x64 => {
                let mode = Box::new(DirectPage);
                $this.stz(mode);
            },
            0x65 => {
                let mode = Box::new(DirectPage);
                $this.adc(mode);
            },
            0x66 => {
                let mode = Box::new(DirectPage);
                $this.ror(mode);
            },
            0x67 => {
                let mode = Box::new(DirectPageIndirectLong);
                $this.adc(mode);
            },
            0x68 => {
                let mode = Box::new(StackPull);
                $this.pla(mode);
            },
            0x69 => {
                let mode = Box::new(Immediate);
                $this.adc(mode);
            },
            0x6A => {
                let mode = Box::new(Accumulator);
                $this.ror(mode);
            },
            0x6B => {
                let mode = Box::new(StackRTL);
                $this.rtl(mode);
            },
            0x6C => {
                let mode = Box::new(AbsoluteIndirect);
                $this.jmp(mode);
            },
            0x6D => {
                let mode = Box::new(Absolute { instruction_type: LocatingData });
                $this.adc(mode);
            },
            0x6E => {
                let mode = Box::new(Absolute { instruction_type: LocatingData });
                $this.ror(mode);
            },
            0x6F => {
                let mode = Box::new(AbsoluteLong);
                $this.adc(mode);
            },
            0x70 => {
                let mode = Box::new(ProgramCounterRelative);
                $this.bvs(mode);
            },
            0x71 => {
                let mode = Box::new(DirectPageIndirectIndexedY);
                $this.adc(mode);
            },
            0x72 => {
                let mode = Box::new(DirectPageIndirect);
                $this.adc(mode);
            },
            0x73 => {
                let mode = Box::new(StackRelativeIndirectIndexedY);
                $this.adc(mode);
            },
            0x74 => {
                let mode = Box::new(StackRelativeIndirectIndexedY);
                $this.stz(mode);
            },
            0x75 => {
                let mode = Box::new(DirectPageIndexedX);
                $this.adc(mode);
            },
            0x76 => {
                let mode = Box::new(DirectPageIndexedX);
                $this.ror(mode);
            },
            0x77 => {
                let mode = Box::new(DirectPageIndirectLongIndexedY);
                $this.adc(mode);
            },
            0x78 => {
                $this.sei();
            },
            0x79 => {
                let mode = Box::new(AbsoluteIndexedY);
                $this.adc(mode);
            },
            0x7A => {
                let mode = Box::new(StackPull);
                $this.ply(mode);
            },
            0x7B => {
                $this.tdc();
            },
            0x7C => {
                let mode = Box::new(AbsoluteIndexedIndirect);
                $this.jmp(mode);
            },
            0x7D => {
                let mode = Box::new(AbsoluteIndexedX);
                $this.adc(mode);
            },
            0x7E => {
                let mode = Box::new(AbsoluteIndexedX);
                $this.ror(mode);
            },
            0x7F => {
                let mode = Box::new(AbsoluteLongIndexedX);
                $this.adc(mode);
            },
            0x80 => {
                let mode = Box::new(ProgramCounterRelative);
                $this.bra(mode);
            },
            0x81 => {
                let mode = Box::new(DirectPageIndexedIndirectX);
                $this.sta(mode);
            },
            0x82 => {
                let mode = Box::new(ProgramCounterRelativeLong);
                $this.brl(mode);
            },
            0x83 => {
                let mode = Box::new(StackRelative);
                $this.sta(mode);
            },
            0x84 => {
                let mode = Box::new(DirectPage);
                $this.sty(mode);
            },
            0x85 => {
                let mode = Box::new(DirectPage);
                $this.sta(mode);
            },
            0x86 => {
                let mode = Box::new(DirectPage);
                $this.stx(mode);
            },
            0x87 => {
                let mode = Box::new(DirectPageIndirectLong);
                $this.sta(mode);
            },
            0x88 => {
                $this.dey();
            },
            0x89 => {
                let mode = Box::new(Immediate);
                $this.bit(mode);
            },
            0x8A => {
                $this.txa();
            },
            0x8B => {
                let mode = Box::new(StackPush);
                $this.phb(mode);
            },
            0x8C => {
                let mode = Box::new(Absolute { instruction_type: LocatingData });
                $this.sty(mode);
            },
            0x8D => {
                let mode = Box::new(Absolute { instruction_type: LocatingData });
                $this.sta(mode);
            },
            0x8E => {
                let mode = Box::new(Absolute { instruction_type: LocatingData });
                $this.stx(mode);
            },
            0x8F => {
                let mode = Box::new(AbsoluteLong);
                $this.sta(mode);
            },
            0x90 => {
                let mode = Box::new(ProgramCounterRelative);
                $this.bcc(mode);
            },
            0x91 => {
                let mode = Box::new(DirectPageIndirectIndexedY);
                $this.sta(mode);
            },
            0x92 => {
                let mode = Box::new(DirectPageIndirect);
                $this.sta(mode);
            },
            0x93 => {
                let mode = Box::new(StackRelativeIndirectIndexedY);
                $this.sta(mode);
            },
            0x94 => {
                let mode = Box::new(DirectPageIndexedX);
                $this.sty(mode);
            },
            0x95 => {
                let mode = Box::new(DirectPageIndexedX);
                $this.sta(mode);
            },
            0x96 => {
                let mode = Box::new(DirectPageIndexedY);
                $this.stx(mode);
            },
            0x97 => {
                let mode = Box::new(DirectPageIndirectLongIndexedY);
                $this.sta(mode);
            },
            0x98 => {
                $this.tya();
            },
            0x99 => {
                let mode = Box::new(AbsoluteIndexedY);
                $this.sta(mode);
            },
            0x9A => {
                $this.txs();
            },
            0x9B => {
                $this.txy();
            },
            0x9C => {
                let mode = Box::new(Absolute { instruction_type: LocatingData });
                $this.stz(mode);
            },
            0x9D => {
                let mode = Box::new(AbsoluteIndexedX);
                $this.sta(mode);
            },
            0x9E => {
                let mode = Box::new(AbsoluteIndexedX);
                $this.stz(mode);
            },
            0x9F => {
                let mode = Box::new(AbsoluteLongIndexedX);
                $this.sta(mode);
            },
            0xA0 => {
                let mode = Box::new(Immediate);
                $this.ldy(mode);
            },
            0xA1 => {
                let mode = Box::new(DirectPageIndexedIndirectX);
                $this.lda(mode);
            },
            0xA2 => {
                let mode = Box::new(Immediate);
                $this.ldx(mode);
            },
            0xA3 => {
                let mode = Box::new(StackRelative);
                $this.lda(mode);
            },
            0xA4 => {
                let mode = Box::new(DirectPage);
                $this.ldy(mode);
            },
            0xA5 => {
                let mode = Box::new(DirectPage);
                $this.lda(mode);
            },
            0xA6 => {
                let mode = Box::new(DirectPage);
                $this.ldx(mode);
            },
            0xA7 => {
                let mode = Box::new(DirectPageIndirectLong);
                $this.lda(mode);
            },
            0xA8 => {
                $this.tay();
            },
            0xA9 => {
                let mode = Box::new(Immediate);
                $this.lda(mode);
            },
            0xAA => {
                $this.tax();
            },
            0xAB => {
                let mode = Box::new(StackPull);
                $this.plb(mode);
            },
            0xAC => {
                let mode = Box::new(Absolute { instruction_type: LocatingData });
                $this.ldy(mode);
            },
            0xAD => {
                let mode = Box::new(Absolute { instruction_type: LocatingData });
                $this.lda(mode);
            },
            0xAE => {
                let mode = Box::new(Absolute { instruction_type: LocatingData });
                $this.ldx(mode);
            },
            0xAF => {
                let mode = Box::new(AbsoluteLong);
                $this.lda(mode);
            },
            0xB0 => {
                let mode = Box::new(ProgramCounterRelative);
                $this.bcs(mode);
            },
            0xB1 => {
                let mode = Box::new(DirectPageIndirectIndexedY);
                $this.lda(mode);
            },
            0xB2 => {
                let mode = Box::new(DirectPageIndirect);
                $this.lda(mode);
            },
            0xB3 => {
                let mode = Box::new(StackRelativeIndirectIndexedY);
                $this.lda(mode);
            },
            0xB4 => {
                let mode = Box::new(DirectPageIndexedX);
                $this.ldy(mode);
            },
            0xB5 => {
                let mode = Box::new(DirectPageIndexedX);
                $this.lda(mode);
            },
            0xB6 => {
                let mode = Box::new(DirectPageIndexedY);
                $this.ldx(mode);
            },
            0xB7 => {
                let mode = Box::new(DirectPageIndirectLongIndexedY);
                $this.lda(mode);
            },
            0xB8 => {
                $this.clv();
            },
            0xB9 => {
                let mode = Box::new(AbsoluteIndexedY);
                $this.lda(mode);
            },
            0xBA => {
                $this.tsx();
            },
            0xBB => {
                $this.tyx();
            },
            0xBC => {
                let mode = Box::new(AbsoluteIndexedX);
                $this.ldy(mode);
            },
            0xBD => {
                let mode = Box::new(AbsoluteIndexedX);
                $this.lda(mode);
            },
            0xBE => {
                let mode = Box::new(AbsoluteIndexedY);
                $this.ldx(mode);
            },
            0xBF => {
                let mode = Box::new(AbsoluteLongIndexedX);
                $this.lda(mode);
            },
            0xC0 => {
                let mode = Box::new(Immediate);
                $this.cpy(mode);
            },
            0xC1 => {
                let mode = Box::new(DirectPageIndexedIndirectX);
                $this.cmp(mode);
            },
            0xC2 => {
                let mode = Box::new(Immediate);
                $this.rep(mode);
            },
            0xC3 => {
                let mode = Box::new(StackRelative);
                $this.cmp(mode);
            },
            0xC4 => {
                let mode = Box::new(DirectPage);
                $this.cpy(mode);
            },
            0xC5 => {
                let mode = Box::new(DirectPage);
                $this.cmp(mode);
            },
            0xC6 => {
                let mode = Box::new(DirectPage);
                $this.dec(mode);
            },
            0xC7 => {
                let mode = Box::new(DirectPageIndirectLong);
                $this.cmp(mode);
            },
            0xC8 => {
                $this.iny();
            },
            0xC9 => {
                let mode = Box::new(Immediate);
                $this.cmp(mode);
            },
            0xCA => {
                $this.dex();
            },
            0xCB => {
                $this.wai();
            },
            0xCC => {
                let mode = Box::new(Absolute { instruction_type: LocatingData });
                $this.cpy(mode);
            },
            0xCD => {
                let mode = Box::new(Absolute { instruction_type: LocatingData });
                $this.cmp(mode);
            },
            0xCE => {
                let mode = Box::new(Absolute { instruction_type: LocatingData });
                $this.dec(mode);
            },
            0xCF => {
                let mode = Box::new(AbsoluteLong);
                $this.cmp(mode);
            },
            0xD0 => {
                let mode = Box::new(ProgramCounterRelative);
                $this.bne(mode);
            },
            0xD1 => {
                let mode = Box::new(DirectPageIndirectIndexedY);
                $this.cmp(mode);
            },
            0xD2 => {
                let mode = Box::new(DirectPageIndirect);
                $this.cmp(mode);
            },
            0xD3 => {
                let mode = Box::new(StackRelativeIndirectIndexedY);
                $this.cmp(mode);
            },
            0xD4 => {
                let mode = Box::new(StackDirectPageIndirect);
                $this.pei(mode);
            },
            0xD5 => {
                let mode = Box::new(DirectPageIndexedX);
                $this.cmp(mode)
            },
            0xD6 => {
                let mode = Box::new(DirectPageIndexedX);
                $this.dec(mode);
            },
            0xD7 => {
                let mode = Box::new(DirectPageIndirectLongIndexedY);
                $this.cmp(mode);
            },
            0xD8 => {
                $this.cld();
            },
            0xD9 => {
                let mode = Box::new(AbsoluteIndexedY);
                $this.cmp(mode);
            },
            0xDA => {
                let mode = Box::new(StackPush);
                $this.phx(mode);
            },
            0xDB => {
                $this.stp();
            },
            0xDC => {
                let mode = Box::new(AbsoluteIndirectLong);
                $this.jmp(mode);
            },
            0xDD => {
                let mode = Box::new(AbsoluteIndexedX);
                $this.cmp(mode);
            },
            0xDE => {
                let mode = Box::new(AbsoluteIndexedX);
                $this.dec(mode);
            },
            0xDF => {
                let mode = Box::new(AbsoluteLongIndexedX);
                $this.cmp(mode);
            },
            0xE0 => {
                let mode = Box::new(Immediate);
                $this.cpx(mode);
            },
            0xE1 => {
                let mode = Box::new(DirectPageIndexedIndirectX);
                $this.sbc(mode);
            },
            0xE2 => {
                let mode = Box::new(Immediate);
                $this.sep(mode);
            },
            0xE3 => {
                let mode = Box::new(DirectPage);
                $this.sbc(mode);
            },
            0xE4 => {
                let mode = Box::new(DirectPage);
                $this.inx(mode);
            },
            0xE5 => {
                let mode = Box::new(DirectPage);
                $this.sbc(mode);
            },
            0xE6 => {
                let mode = Box::new(DirectPage);
                $this.inc(mode);
            },
            0xE7 => {
                let mode = Box::new(DirectPageIndirectLong);
                $this.sbc(mode);
            },
            0xE8 => {
                let mode = Box::new(Dummy);
                $this.inx(mode);
            },
            0xE9 => {
                let mode = Box::new(Immediate);
                $this.sbc(mode);
            },
            0xEA => {
                $this.nop();
            },
            0xEB => {
                $this.xba();
            },
            0xEC => {
                let mode = Box::new(Absolute { instruction_type: LocatingData });
                $this.cpx(mode);
            },
            0xED => {
                let mode = Box::new(Absolute { instruction_type: LocatingData });
                $this.sbc(mode);
            },
            0xEE => {
                let mode = Box::new(Absolute { instruction_type: LocatingData });
                $this.inc(mode);
            },
            0xEF => {
                let mode = Box::new(AbsoluteLong);
                $this.sbc(mode);
            },
            0xF0 => {
                let mode = Box::new(ProgramCounterRelative);
                $this.beq(mode);
            },
            0xF1 => {
                let mode = Box::new(DirectPageIndirectIndexedY);
                $this.sbc(mode);
            },
            0xF2 => {
                let mode = Box::new(DirectPageIndirect);
                $this.sbc(mode);
            },
            0xF3 => {
                let mode = Box::new(StackRelativeIndirectIndexedY);
                $this.sbc(mode);
            },
            0xF4 => {
                let mode = Box::new(StackAbsolute);
                $this.pea(mode);
            },
            0xF5 => {
                let mode = Box::new(DirectPageIndexedX);
                $this.sbc(mode);
            },
            0xF6 => {
                let mode = Box::new(DirectPageIndexedX);
                $this.inc(mode);
            },
            0xF7 => {
                let mode = Box::new(DirectPageIndirectLongIndexedY);
                $this.sbc(mode);
            },
            0xF8 => {
                $this.sed();
            },
            0xF9 => {
                let mode = Box::new(AbsoluteIndexedY);
                $this.sbc(mode);
            },
            0xFA => {
                let mode = Box::new(StackPull);
                $this.plx(mode);
            },
            0xFB => {
                $this.xce();
            },
            0xFC => {
                let mode = Box::new(AbsoluteIndexedIndirect);
                $this.jsr(mode);
            },
            0xFD => {
                let mode = Box::new(AbsoluteIndexedX);
                $this.sbc(mode);
            },
            0xFE => {
                let mode = Box::new(AbsoluteIndexedX);
                $this.inc(mode);
            },
            0xFF => {
                let mode = Box::new(AbsoluteLongIndexedX);
                $this.sbc(mode);
            },
            _ => panic!("{} is not an opcode", $op),
        }
    );
}

#[derive(Debug)]
pub struct CPU {
    emu_acc:                u8,
    accumulator:           u16,
    emu_index_x:            u8,
    index_x:               u16,
    index_y:               u16,
    pub stack_pointer:         usize,
    pub data_bank:          u8,
    pub direct_page:       u16,
    pub program_bank:       usize,
    pub processor_status:   ProcessorStatus,
    pub program_counter: usize,
    emulation_mode:       bool,
    pub memory:         Memory,
}

impl CPU {
	pub fn new(memory: Memory) -> CPU {
        use memory::Vectors::*;
        use memory::RomType::*;

        let pc = match memory.rom.rom_type {
            LoROM | FastLoROM => {
                let vector_loc = LOROM_EMU_MODE_VECTORS[RESET as usize];
                memory.get_byte(vector_loc) as usize |
                    ((memory.get_byte(vector_loc + 1) as usize) << 8)
            },
            HiROM | FastHiROM => {
                let vector_loc = HIROM_EMU_MODE_VECTORS[RESET as usize];
                memory.get_byte(vector_loc) as usize | ((memory.get_byte(vector_loc + 1) as usize) << 8)
            },
            _ => panic!("ExLo/HiROM formats not supported")
        };

        CPU {
            emu_acc:           0,
            accumulator:       0,
            emu_index_x:       0,
            index_x:           0,
            index_y:           0,
            stack_pointer:     0,
            data_bank:         0,
            direct_page:       0,
            program_bank:      0,
            processor_status:  ProcessorStatus::default(),
            program_counter:   pc,
            emulation_mode: true,
            memory:       memory,
        }
	}

    pub fn run(&mut self) {
        loop {
            println!("{:?}", self);
            self.run_instruction();
        }
    }

    fn run_instruction(&mut self) {
        let opcode = self.memory.get_byte((self.program_bank << 16) | self.program_counter);
        println!("opcode {:x}", opcode);
        decode_op_and_execute!(opcode, self);
    }

    fn brk<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("brk unimplemented")
    }

    fn ora<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("ora unimplemented")
    }

    fn cop<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("cop unimplemented")
    }

    fn tsb<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("tsb unimplemented")
    }

    fn asl<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("asl unimplemented")
    }

    fn php<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("php unimplemented")
    }

    fn phd<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("phd unimplemented")
    }

    fn blp<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("blp unimplemented")
    }

    fn trb<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("trb unimplemented")
    }

    fn clc(&mut self) {
        use self::StatusFlags::Carry;

        self.processor_status.set_flag(Carry, false);
        self.program_counter += 1;
    }

    fn inc<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("inc unimplemented")
    }

    fn tcs(&mut self) {
        panic!("tcs unimplemented")
    }

    fn jsr<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("jsr unimplemented")
    }

    fn and<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("and unimplemented")
    }

    fn bit<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("bit unimplemented")
    }

    fn rol<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("rol unimplemented")
    }

    fn plp<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("plp unimplemented")
    }

    fn pld<T: Instruction>(&mut self, mode: Box<T>) {
        use self::StatusFlags::Zero;

        let data = mode.load(self);

        self.direct_page = data as u16;

        self.processor_status.set_by_byte(self.emulation_mode, ((data & 0x8000) >> 8) as u8, true);
        if data == 0 {
            self.processor_status.set_by_byte(self.emulation_mode, 1 << (Zero as usize), true);
        }

        self.program_counter += 1;
    }

    fn bmi<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("bmi unimplemented")
    }

    fn sec(&mut self) {
        panic!("sec unimplemented")
    }

    fn dec<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("dec unimplemented")
    }

    fn tsc(&mut self) {
        panic!("tsc unimplemented")
    }

    fn rti<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("rti unimplemented")
    }

    fn eor<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("eor unimplemented")
    }

    fn wdm(&mut self) {
        panic!("wdm unimplemented")
    }

    fn mvp<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("mvp unimplemented")
    }

    fn lsr<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("lsr unimplemented")
    }

    fn pha<T: Instruction>(&mut self, mode: Box<T>) {
        use self::StatusFlags::AccumulatorRegisterSize;

        let mut data = self.emu_acc as usize;
        self.processor_status.inst_8_bit = true;

        if !self.processor_status.status[AccumulatorRegisterSize as usize] {
            data = self.accumulator as usize;
            self.processor_status.inst_8_bit = false;
        }

        mode.store(self, data);

        self.processor_status.inst_8_bit = false;

        self.program_counter += 1;
    }

    fn phk<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("phk unimplemented")
    }

    fn jmp<T: Instruction>(&mut self, mode: Box<T>) {
        let jump_addr = mode.load(self) as usize;
        self.program_counter = jump_addr & 0x00FFFF;
        self.program_bank = (jump_addr & 0xFF0000) >> 16;
    }

    fn bvc<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("bvc unimplemented")
    }

    fn mvn<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("mvn unimplemented")
    }

    fn cli(&mut self) {
        panic!("cli unimplemented")
    }

    fn phy<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("phy unimplemented")
    }

    fn tcd(&mut self) {
        panic!("tcd unimplemented")
    }

    fn rts<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("rts unimplemented")
    }

    fn adc<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("adc unimplemented")
    }

    fn per<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("per unimplemented")
    }

    fn stz<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("stz unimplemented")
    }

    fn ror<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("ror unimplemented")
    }

    fn pla<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("pla unimplemented")
    }

    fn rtl<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("rtl unimplemented")
    }

    fn bvs<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("bvs unimplemented")
    }

    fn sei(&mut self) {
        use self::StatusFlags::IRQDisable;

        self.processor_status.set_flag(IRQDisable, true);
        self.program_counter += 1;
    }

    fn ply<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("ply unimplemented")
    }

    fn tdc(&mut self) {
        use self::StatusFlags::{Negative, Zero};
        self.accumulator = self.direct_page;

        self.processor_status.set_flag(Negative, self.accumulator & 0x8000 != 0);
        self.processor_status.set_flag(Zero, self.accumulator == 0);

        self.program_counter += 1;
    }

    fn bra<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("bra unimplemented")
    }

    fn sta<T: Instruction>(&mut self, mode: Box<T>) {
        use self::StatusFlags::AccumulatorRegisterSize;

        let mut data = self.emu_acc as usize;
        self.processor_status.inst_8_bit = true;

        if !self.processor_status.status[AccumulatorRegisterSize as usize] {
            data = self.accumulator as usize;
            self.processor_status.inst_8_bit = false;
        }

        mode.store(self, data);
        self.processor_status.inst_8_bit = false;

        self.program_counter += 1;
    }

    fn brl<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("brl unimplemented")
    }

    fn sty<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("sty unimplemented")
    }

    fn stx<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("stx unimplemented")
    }

    fn dey(&mut self) {
        panic!("dey unimplemented")
    }

    fn txa(&mut self) {
        panic!("txa unimplemented")
    }

    fn phb<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("phb unimplemented")
    }

    fn bcc<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("bcc unimplemented")
    }

    fn tya(&mut self) {
        panic!("tya unimplemented")
    }

    fn txs(&mut self) {
        use self::StatusFlags::IndexRegisterSize;

        if self.emulation_mode {
            self.stack_pointer = self.emu_index_x as usize;
        } else {
            if self.processor_status.status[IndexRegisterSize as usize] {
                self.stack_pointer = 0x00FF & (self.emu_index_x as usize);
            } else {
                self.stack_pointer = self.index_x as usize;
            }
        }

        self.program_counter += 1;
    }

    fn txy(&mut self) {
        panic!("txy unimplemented")
    }

    fn ldy<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("ldy unimplemented")
    }

    fn lda<T: Instruction>(&mut self, mode: Box<T>) {
        use self::StatusFlags::{AccumulatorRegisterSize, Zero};

        if self.processor_status.status[AccumulatorRegisterSize as usize] {
            let data = mode.load(self);

            self.emu_acc = data as u8;
            self.accumulator = data as u16;

            self.processor_status.set_by_byte(self.emulation_mode, (data as u8) & 0x80, true);
            if data == 0 {
                self.processor_status.set_by_byte(self.emulation_mode, 1 << (Zero as usize), true);
            }
        } else {
            let low = mode.load(self) as usize;
            let high = mode.load(self) as usize;

            let data = (high << 8) | low;

            self.accumulator = data as u16;
            self.emu_acc = (data & 0xFF) as u8;

            self.processor_status.set_by_byte(self.emulation_mode, ((data & 0x8000) >> 8) as u8, true);
            if data == 0 {
                self.processor_status.set_by_byte(self.emulation_mode, 1 << (Zero as usize), true);
            }
        }
        
        self.program_counter += 1;
    }

    fn ldx<T: Instruction>(&mut self, mode: Box<T>) {
        use self::StatusFlags::{IndexRegisterSize, Zero};

        if self.processor_status.status[IndexRegisterSize as usize] {
            let data = mode.load(self);

            self.emu_index_x = data as u8;
            self.index_x = data as u16;

            self.processor_status.set_by_byte(self.emulation_mode, (data as u8) & 0x80, true);
            if data == 0 {
                self.processor_status.set_by_byte(self.emulation_mode, 1 << (Zero as usize), true);
            }
        } else {
            let low = mode.load(self) as usize;
            let high = mode.load(self) as usize;

            let data = (high << 8) | low;

            self.index_x = data as u16;
            self.emu_index_x = (data & 0xFF) as u8;

            self.processor_status.set_by_byte(self.emulation_mode, ((data & 0x8000) >> 8) as u8, true);
            if data == 0 {
                self.processor_status.set_by_byte(self.emulation_mode, 1 << (Zero as usize), true);
            }
        }
        
        self.program_counter += 1;
    }

    fn tay(&mut self) {
        panic!("tay unimplemented")
    }

    fn tax(&mut self) {
        panic!("tax unimplemented")
    }

    fn plb<T: Instruction>(&mut self, mode: Box<T>) {
        use self::StatusFlags::{Negative, Zero};

        self.processor_status.inst_8_bit = true;

        let data = mode.load(self) as u8;
        self.data_bank = data;

        self.processor_status.inst_8_bit = false;

        self.processor_status.set_flag(Negative, data & 0x80 != 0);
        self.processor_status.set_flag(Zero, data == 0);

        self.program_counter += 1;
    }

    fn bcs<T: Instruction>(&mut self, mode: Box<T>) {
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

    fn cpy<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("cpy unimplemented")
    }

    fn cmp<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("cmp unimplemented")
    }

    fn rep<T: Instruction>(&mut self, mode: Box<T>) {
        let val = mode.load(self);

        self.processor_status.set_by_byte(self.emulation_mode, val as u8, false);
        self.program_counter += 1;
    }

    fn iny(&mut self) {
        panic!("iny unimplemented")
    }

    fn dex(&mut self) {
        panic!("dex unimplemented")
    }

    fn wai(&mut self) {
        panic!("wai unimplemented")
    }

    fn bne<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("bne unimplemented")
    }

    fn pei<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("pei unimplemented")
    }

    fn cld(&mut self) {
        panic!("cld unimplemented")
    }

    fn phx<T: Instruction>(&mut self, mode: Box<T>) {
        use self::StatusFlags::IndexRegisterSize;

        let mut xreg = self.emu_index_x as usize;
        self.processor_status.inst_8_bit = true;

        if !self.processor_status.status[IndexRegisterSize as usize] {
            xreg = self.index_x as usize;
            self.processor_status.inst_8_bit = false;
        }

        mode.store(self, xreg);

        self.processor_status.inst_8_bit = false;

        self.program_counter += 1;
    }

    fn stp(&mut self) {
        panic!("stp unimplemented")
    }

    fn cpx<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("cpx unimplemented")
    }

    fn sbc<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("sbc unimplemented")
    }

    fn inx<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("inx unimplemented")
    }

    fn nop(&mut self) {
        panic!("nop unimplemented")
    }

    fn xba(&mut self) {
        panic!("xba unimplemented")
    }

    fn beq<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("beq unimplemented")
    }

    fn pea<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("pea unimplemented")
    }

    fn sed(&mut self) {
        panic!("sed unimplemented")
    }

    fn plx<T: Instruction>(&mut self, mode: Box<T>) {
        panic!("plx unimplemented")
    }

    fn xce(&mut self) {
        use self::StatusFlags::Carry;
        let emu_bit = self.emulation_mode;

        self.emulation_mode = self.processor_status.status[Carry as usize];
        self.processor_status.status[Carry as usize] = emu_bit;
        self.program_counter += 1;
    }

    fn sep<T: Instruction>(&mut self, mode: Box<T>) {
        let val = mode.load(self);
        
        self.processor_status.set_by_byte(self.emulation_mode, val as u8, true);
        self.program_counter += 1;
    }
}

#[derive(Default, Debug)]
pub struct ProcessorStatus {
    pub status: [bool; 8],
    pub inst_8_bit: bool,
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
