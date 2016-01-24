use memory::Memory;
use flags::Flags;
use flags::Flags::*;
use modes::*;
use modes::InstructionType::*;

const EMULATION_INTERRUPT_VECTOR: u16 = 0xFFFE;
const NATIVE_INTERRUPT_VECTOR: u16 = 0xFFE6;
const EMULATION_COPROCESSOR_VECTOR: u16 = 0xFFF4;
const NATIVE_COPROCESSOR_VECTOR: u16 = 0xFFE4;

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
                let mode = Box::new(Implied);
                $this.clc(mode);
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
                let mode = Box::new(Implied);
                $this.tcs(mode);
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
                let mode = Box::new(Implied);
                $this.sec(mode);
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
                let mode = Box::new(Implied);
                $this.tsc(mode);
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
                let mode = Box::new(Implied);
                $this.cli(mode);
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
                let mode = Box::new(Implied);
                $this.tcd(mode);
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
                let mode = Box::new(Implied);
                $this.sei(mode);
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
                let mode = Box::new(Implied);
                $this.tdc(mode);
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
                let mode = Box::new(Implied);
                $this.dey(mode);
            },
            0x89 => {
                let mode = Box::new(Immediate);
                $this.bit(mode);
            },
            0x8A => {
                let mode = Box::new(Implied);
                $this.txa(mode);
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
                let mode = Box::new(Implied);
                $this.tya(mode);
            },
            0x99 => {
                let mode = Box::new(AbsoluteIndexedY);
                $this.sta(mode);
            },
            0x9A => {
                let mode = Box::new(Implied);
                $this.txs(mode);
            },
            0x9B => {
                let mode = Box::new(Implied);
                $this.txy(mode);
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
                let mode = Box::new(Implied);
                $this.tay(mode);
            },
            0xA9 => {
                let mode = Box::new(Immediate);
                $this.lda(mode);
            },
            0xAA => {
                let mode = Box::new(Implied);
                $this.tax(mode);
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
                let mode = Box::new(Implied);
                $this.clv(mode);
            },
            0xB9 => {
                let mode = Box::new(AbsoluteIndexedY);
                $this.lda(mode);
            },
            0xBA => {
                let mode = Box::new(Implied);
                $this.tsx(mode);
            },
            0xBB => {
                let mode = Box::new(Implied);
                $this.tyx(mode);
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
                let mode = Box::new(Implied);
                $this.iny(mode);
            },
            0xC9 => {
                let mode = Box::new(Immediate);
                $this.cmp(mode);
            },
            0xCA => {
                let mode = Box::new(Implied);
                $this.dex(mode);
            },
            0xCB => {
                let mode = Box::new(Implied);
                $this.wai(mode);
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
                let mode = Box::new(Implied);
                $this.cld(mode);
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
                let mode = Box::new(Implied);
                $this.stp(mode);
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
                $this.cpx(mode);
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
                let mode = Box::new(Implied);
                $this.inx(mode);
            },
            0xE9 => {
                let mode = Box::new(Immediate);
                $this.sbc(mode);
            },
            0xEA => {
                let mode = Box::new(Implied);
                $this.nop(mode);
            },
            0xEB => {
                let mode = Box::new(Implied);
                $this.xba(mode);
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
                let mode = Box::new(Implied);
                $this.sed(mode);
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
                let mode = Box::new(Implied);
                $this.xce(mode);
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

pub struct CPU {
    accumulator:          u16,
    index_x:              u16,
    index_y:              u16,
    stack_pointer:        u16,
    pub data_bank:         u8,
    pub direct_page:      u16,
    pub program_bank:      u8,
    pub processor_status:  u8,
    pub program_counter:  u16,
    emulation_mode:      bool,
    pub memory:        Memory,
}

impl CPU {
	pub fn new(memory: Memory) -> CPU {
        CPU {
            accumulator:       0,
            index_x:           0,
            index_y:           0,
            stack_pointer:     0,
            data_bank:         0,
            direct_page:       0,
            program_bank:      0,
            processor_status:  0,
            program_counter:   0,
            emulation_mode: true,
            memory:       memory,
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
            NativeModeFlag              => !self.emulation_mode,
		}
	}
	
	pub fn set_flag(&mut self, flag: Flags, val: bool) {
		if val {
			self.processor_status = self.processor_status | flag
		} else {
			self.processor_status = self.processor_status & !flag
		}
	}

    fn check_if_negative_u16(data: u16) -> bool {
        (data & 0x8000) >> 15 == 1
    }
    
    fn check_if_negative_u8(data: u8) -> bool {
        (data & 0x80) >> 7 == 1
    }

    fn brk<T: Instruction>(&mut self, mode: Box<T>) {
        self.program_counter += 1;

        let pc = self.program_counter;
        let ps = self.processor_status as u16;
        let pb = self.program_bank as u16;

        if self.emulation_mode {
            mode.store(self, pc);
            self.set_flag(ProgramBreakInterruptFlag, true);
            mode.store(self, ps);
            self.set_flag(IRQDisableFlag, true);
            self.program_counter = EMULATION_INTERRUPT_VECTOR;
        } else {
            mode.store(self, pb);
            mode.store(self, pc);
            mode.store(self, ps);
            self.set_flag(IRQDisableFlag, true);
            self.program_bank = 0;
            self.program_counter = NATIVE_INTERRUPT_VECTOR;
        }

        self.set_flag(DecimalFlag, false);
    }

    fn ora<T: Instruction>(&mut self, mode: Box<T>) {
        let data = mode.load(self);
        self.accumulator |= data;

        let negative = if self.emulation_mode {
            CPU::check_if_negative_u8(self.accumulator as u8)
        } else {
            CPU::check_if_negative_u16(self.accumulator)
        };

        self.set_flag(NegativeFlag, negative);
        let is_zero = self.accumulator == 0;
        self.set_flag(ZeroFlag, is_zero);
    }

    fn cop<T: Instruction>(&mut self, mode: Box<T>) {
        self.program_counter += 1;

        let pc = self.program_counter;
        let pb = self.program_bank as u16;
        let ps = self.processor_status as u16;

        self.set_flag(IRQDisableFlag, true);

        if self.emulation_mode {
            mode.store(self, pc);
            self.program_counter = EMULATION_COPROCESSOR_VECTOR;
        } else {
            mode.store(self, pb);
            mode.store(self, pc);
            mode.store(self, ps);
            self.program_bank = 0;
            self.program_counter = NATIVE_COPROCESSOR_VECTOR;
        }

        self.set_flag(DecimalFlag, true);
    }

    fn tsb<T: Instruction>(&mut self, mode: Box<T>) {
        let data = mode.load(self);
        let result = data | self.accumulator;
        let is_zero = (data & self.accumulator) == 0;

        mode.store(self, result);

        self.set_flag(ZeroFlag, is_zero);
    }

    fn asl<T: Instruction>(&mut self, mode: Box<T>) {
        let data = mode.load(self);

        let is_carry = if self.emulation_mode {
            CPU::check_if_negative_u8(data as u8)
        } else {
            CPU::check_if_negative_u16(data)
        };

        self.set_flag(CarryFlag, is_carry);

        let result = data << 1;
        mode.store(self, data);
    }

    fn php<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn phd<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn blp<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn trb<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn clc<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn inc<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn tcs<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn jsr<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn and<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn bit<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn rol<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn plp<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn pld<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn bmi<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn sec<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn dec<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn tsc<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn rti<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn eor<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn wdm(&mut self) {
    }

    fn mvp<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn lsr<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn pha<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn phk<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn jmp<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn bvc<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn mvn<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn cli<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn phy<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn tcd<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn rts<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn adc<T: Instruction>(&mut self, mode: Box<T>) {
        let data = mode.load(self);

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

        let negative = CPU::check_if_negative_u16(self.accumulator);

        self.set_flag(NegativeFlag, negative);
        self.set_flag(OverflowFlag, (!orig_neg && negative) || (overflow && negative));
        self.set_flag(ZeroFlag, result == 0);
        self.set_flag(CarryFlag, overflow);
    }

    fn per<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn stz<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn ror<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn pla<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn rtl<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn bvs<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn sei<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn ply<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn tdc<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn bra<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn sta<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn brl<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn sty<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn stx<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn dey<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn txa<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn phb<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn bcc<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn tya<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn txs<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn txy<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn ldy<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn lda<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn ldx<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn tay<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn tax<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn plb<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn bcs<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn clv<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn tsx<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn tyx<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn cpy<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn cmp<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn rep<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn iny<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn dex<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn wai<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn bne<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn pei<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn cld<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn phx<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn stp<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn cpx<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn sbc<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn inx<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn nop<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn xba<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn beq<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn pea<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn sed<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn plx<T: Instruction>(&mut self, mode: Box<T>) {
    }

    fn xce<T: Instruction>(&mut self, mode: Box<T>) {
    }
}

