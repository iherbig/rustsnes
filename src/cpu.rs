use memory::Memory;
use modes::*;
use modes::InstructionType::*;
use std::fmt;

const IS_BYTE: bool = true;

macro_rules! decode_op_and_execute {
    ($op:expr, $this:ident, $mem:ident) => (
        match $op {
            0x00 => {
                print!("opcode {:x} brk ", $op);
                let mode = StackPush;
                $this.brk(&mode, $mem);
            },
            0x01 => {
                print!("opcode {:x} ora ", $op);
                let mode = DirectPageIndexedIndirectX;
                $this.ora(&mode, $mem);
            },
            0x02 => {
                unreachable!("COP is not used by the SNES");
            },
            0x03 => {
                print!("opcode {:x} ora ", $op);
                let mode = StackRelative;
                $this.ora(&mode, $mem);
            },
            0x04 => {
                print!("opcode {:x} tsb ", $op);
                let mode = DirectPage;
                $this.tsb(&mode, $mem);
            },
            0x05 => {
                print!("opcode {:x} ora ", $op);
                let mode = DirectPage;
                $this.ora(&mode, $mem);
            },
            0x06 => {
                print!("opcode {:x} asl ", $op);
                let mode = DirectPage;
                $this.asl(&mode, $mem);
            },
            0x07 => {
                print!("opcode {:x} ora ", $op);
                let mode = DirectPageIndirectLong;
                $this.ora(&mode, $mem);
            },
            0x08 => {
                print!("opcode {:x} php ", $op);
                let mode = StackPush;
                $this.php(&mode, $mem);
            },
            0x09 => {
                print!("opcode {:x} ora ", $op);
                let mode = Immediate;
                $this.ora(&mode, $mem);
            },
            0x0A => {
                print!("opcode {:x} asl ", $op);
                let mode = Accumulator;
                $this.asl(&mode, $mem);
            },
            0x0B => {
                print!("opcode {:x} phd ", $op);
                let mode = StackPush;
                $this.phd(&mode, $mem);
            },
            0x0C => {
                print!("opcode {:x} tsb ", $op);
                let mode = Absolute { instruction_type: LocatingData };
                $this.tsb(&mode, $mem);
            },
            0x0D => {
                print!("opcode {:x} ora ", $op);
                let mode = Absolute { instruction_type: LocatingData };
                $this.ora(&mode, $mem);
            },
            0x0E => {
                print!("opcode {:x} asl ", $op);
                let mode = Absolute { instruction_type: LocatingData };
                $this.asl(&mode, $mem);
            },
            0x0F => {
                print!("opcode {:x} ora ", $op);
                let mode = AbsoluteLong { instruction_type : LocatingData };
                $this.ora(&mode, $mem);
            },
            0x10 => {
                print!("opcode {:x} bpl ", $op);
                let mode = ProgramCounterRelative;
                $this.bpl(&mode, $mem);
            },
            0x11 => {
                print!("opcode {:x} ora ", $op);
                let mode = DirectPageIndirectIndexedY;
                $this.ora(&mode, $mem);
            },
            0x12 => {
                print!("opcode {:x} ora ", $op);
                let mode = DirectPageIndirect;
                $this.ora(&mode, $mem);
            },
            0x13 => {
                print!("opcode {:x} ora ", $op);
                let mode = StackRelativeIndirectIndexedY;
                $this.ora(&mode, $mem);
            },
            0x14 => {
                print!("opcode {:x} trb ", $op);
                let mode = DirectPage;
                $this.trb(&mode, $mem);
            },
            0x15 => {
                print!("opcode {:x} ora ", $op);
                let mode = DirectPageIndexedX;
                $this.ora(&mode, $mem);
            },
            0x16 => {
                print!("opcode {:x} asl ", $op);
                let mode = DirectPageIndexedX;
                $this.asl(&mode, $mem);
            },
            0x17 => {
                print!("opcode {:x} ora ", $op);
                let mode = DirectPageIndirectLongIndexedY;
                $this.ora(&mode, $mem);
            },
            0x18 => {
                print!("opcode {:x} clc ", $op);
                $this.clc();
            },
            0x19 => {
                print!("opcode {:x} ora ", $op);
                let mode = AbsoluteIndexedY;
                $this.ora(&mode, $mem);
            },
            0x1A => {
                print!("opcode {:x} inc ", $op);
                let mode = Accumulator;
                $this.inc(&mode, $mem);
            },
            0x1B => {
                print!("opcode {:x} tcs ", $op);
                $this.tcs();
            },
            0x1C => {
                print!("opcode {:x} trb ", $op);
                let mode = Absolute { instruction_type: LocatingData };
                $this.trb(&mode, $mem);
            },
            0x1D => {
                print!("opcode {:x} ora ", $op);
                let mode = AbsoluteIndexedX;
                $this.ora(&mode, $mem);
            },
            0x1E => {
                print!("opcode {:x} asl ", $op);
                let mode = AbsoluteIndexedX;
                $this.asl(&mode, $mem);
            },
            0x1F => {
                print!("opcode {:x} ora ", $op);
                let mode = AbsoluteLongIndexedX;
                $this.ora(&mode, $mem);
            },
            0x20 => {
                print!("opcode {:x} jsr ", $op);
                let mode = Absolute { instruction_type: ControlTransfer };
                $this.jsr(&mode, $mem, false);
            },
            0x21 => {
                print!("opcode {:x} and ", $op);
                let mode = DirectPageIndexedIndirectX;
                $this.and(&mode, $mem);
            },
            0x22 => {
                print!("opcode {:x} jsr ", $op);
                let mode = AbsoluteLong { instruction_type: ControlTransfer };
                $this.jsr(&mode, $mem, true);
            },
            0x23 => {
                print!("opcode {:x} and ", $op);
                let mode = StackRelative;
                $this.and(&mode, $mem);
            },
            0x24 => {
                print!("opcode {:x} bit ", $op);
                let mode = DirectPage;
                $this.bit(&mode, $mem);
            },
            0x25 => {
                print!("opcode {:x} and ", $op);
                let mode = DirectPage;
                $this.and(&mode, $mem);
            },
            0x26 => {
                print!("opcode {:x} rol ", $op);
                let mode = DirectPage;
                $this.rol(&mode, $mem);
            },
            0x27 => {
                print!("opcode {:x} and ", $op);
                let mode = DirectPageIndirectLong;
                $this.and(&mode, $mem);
            },
            0x28 => {
                print!("opcode {:x} plp ", $op);
                let mode = StackPull;
                $this.plp(&mode, $mem);
            },
            0x29 => {
                print!("opcode {:x} and ", $op);
                let mode = Immediate;
                $this.and(&mode, $mem);
            },
            0x2A => {
                print!("opcode {:x} rol ", $op);
                let mode = Accumulator;
                $this.rol(&mode, $mem);
            },
            0x2B => {
                print!("opcode {:x} pld ", $op);
                let mode = StackPull;
                $this.pld(&mode, $mem);
            },
            0x2C => {
                print!("opcode {:x} bit ", $op);
                let mode = Absolute { instruction_type: LocatingData };
                $this.bit(&mode, $mem);
            },
            0x2D => {
                print!("opcode {:x} and ", $op);
                let mode = Absolute { instruction_type: LocatingData };
                $this.and(&mode, $mem);
            },
            0x2E => {
                print!("opcode {:x} rol ", $op);
                let mode = Absolute { instruction_type: LocatingData };
                $this.rol(&mode, $mem);
            },
            0x2F => {
                print!("opcode {:x} and ", $op);
                let mode = AbsoluteLong { instruction_type: LocatingData };
                $this.and(&mode, $mem);
            },
            0x30 => {
                print!("opcode {:x} bmi ", $op);
                let mode = ProgramCounterRelative;
                $this.bmi(&mode, $mem);
            },
            0x31 => {
                print!("opcode {:x} and ", $op);
                let mode = DirectPageIndirectIndexedY;
                $this.and(&mode, $mem);
            },
            0x32 => {
                print!("opcode {:x} and ", $op);
                let mode = DirectPageIndirect;
                $this.and(&mode, $mem);
            },
            0x33 => {
                print!("opcode {:x} and ", $op);
                let mode = StackRelativeIndirectIndexedY;
                $this.and(&mode, $mem);
            },
            0x34 => {
                print!("opcode {:x} bit ", $op);
                let mode = DirectPageIndexedX;
                $this.bit(&mode, $mem);
            },
            0x35 => {
                print!("opcode {:x} and ", $op);
                let mode = DirectPageIndexedX;
                $this.and(&mode, $mem);
            },
            0x36 => {
                print!("opcode {:x} rol ", $op);
                let mode = DirectPageIndexedX;
                $this.rol(&mode, $mem);
            },
            0x37 => {
                print!("opcode {:x} and ", $op);
                let mode = DirectPageIndirectLongIndexedY;
                $this.and(&mode, $mem);
            },
            0x38 => {
                print!("opcode {:x} sec ", $op);
                $this.sec();
            },
            0x39 => {
                print!("opcode {:x} and ", $op);
                let mode = AbsoluteIndexedY;
                $this.and(&mode, $mem);
            },
            0x3A => {
                print!("opcode {:x} dec ", $op);
                let mode = Accumulator;
                $this.dec(&mode, $mem);
            },
            0x3B => {
                print!("opcode {:x} tsc ", $op);
                $this.tsc();
            },
            0x3C => {
                print!("opcode {:x} bit ", $op);
                let mode = AbsoluteIndexedX;
                $this.bit(&mode, $mem);
            },
            0x3D => {
                print!("opcode {:x} and ", $op);
                let mode = AbsoluteIndexedX;
                $this.and(&mode, $mem);
            },
            0x3E => {
                print!("opcode {:x} rol ", $op);
                let mode = AbsoluteIndexedX;
                $this.rol(&mode, $mem);
            },
            0x3F => {
                print!("opcode {:x} and ", $op);
                let mode = AbsoluteLongIndexedX;
                $this.and(&mode, $mem);
            },
            0x40 => {
                print!("opcode {:x} rti ", $op);
                let mode = StackRTI;
                $this.rti(&mode, $mem);
            },
            0x41 => {
                print!("opcode {:x} eor ", $op);
                let mode = DirectPageIndexedIndirectX;
                $this.eor(&mode, $mem);
            },
            0x42 => {
                print!("opcode {:x} wdm ", $op);
                $this.wdm();
            },
            0x43 => {
                print!("opcode {:x} eor ", $op);
                let mode = StackRelative;
                $this.eor(&mode, $mem);
            },
            0x44 => {
                print!("opcode {:x} mvp ", $op);
                let mode = BlockMove;
                $this.mvp(&mode, $mem);
            },
            0x45 => {
                print!("opcode {:x} eor ", $op);
                let mode = DirectPage;
                $this.eor(&mode, $mem);
            },
            0x46 => {
                print!("opcode {:x} lsr ", $op);
                let mode = DirectPage;
                $this.lsr(&mode, $mem);
            },
            0x47 => {
                print!("opcode {:x} eor ", $op);
                let mode = DirectPageIndirectLong;
                $this.eor(&mode, $mem);
            },
            0x48 => {
                print!("opcode {:x} pha ", $op);
                let mode = StackPush;
                $this.pha(&mode, $mem);
            },
            0x49 => {
                print!("opcode {:x} eor ", $op);
                let mode = Immediate;
                $this.eor(&mode, $mem);
            },
            0x4A => {
                print!("opcode {:x} lsr ", $op);
                let mode = Accumulator;
                $this.lsr(&mode, $mem);
            },
            0x4B => {
                print!("opcode {:x} phk ", $op);
                let mode = StackPush;
                $this.phk(&mode, $mem);
            },
            0x4C => {
                print!("opcode {:x} jmp ", $op);
                let mode = Absolute { instruction_type: ControlTransfer };
                $this.jmp(&mode, $mem);
            },
            0x4D => {
                print!("opcode {:x} eor ", $op);
                let mode = Absolute { instruction_type: LocatingData };
                $this.eor(&mode, $mem);
            },
            0x4E => {
                print!("opcode {:x} lsr ", $op);
                let mode = Absolute { instruction_type: LocatingData };
                $this.lsr(&mode, $mem);
            },
            0x4F => {
                print!("opcode {:x} eor ", $op);
                let mode = AbsoluteLong { instruction_type: LocatingData };
                $this.eor(&mode, $mem);
            },
            0x50 => {
                print!("opcode {:x} bvc ", $op);
                let mode = ProgramCounterRelative;
                $this.bvc(&mode, $mem);
            },
            0x51 => {
                print!("opcode {:x} eor ", $op);
                let mode = DirectPageIndirectIndexedY;
                $this.eor(&mode, $mem);
            },
            0x52 => {
                print!("opcode {:x} eor ", $op);
                let mode = DirectPageIndirect;
                $this.eor(&mode, $mem);
            },
            0x53 => {
                print!("opcode {:x} eor ", $op);
                let mode = StackRelativeIndirectIndexedY;
                $this.eor(&mode, $mem);
            },
            0x54 => {
                print!("opcode {:x} mvn ", $op);
                let mode = BlockMove;
                $this.mvn(&mode, $mem);
            },
            0x55 => {
                print!("opcode {:x} eor ", $op);
                let mode = DirectPageIndexedX;
                $this.eor(&mode, $mem);
            },
            0x56 => {
                print!("opcode {:x} lsr ", $op);
                let mode = DirectPageIndexedX;
                $this.lsr(&mode, $mem);
            },
            0x57 => {
                print!("opcode {:x} eor ", $op);
                let mode = DirectPageIndirectLongIndexedY;
                $this.eor(&mode, $mem);
            },
            0x58 => {
                print!("opcode {:x} cli ", $op);
                $this.cli();
            },
            0x59 => {
                print!("opcode {:x} eor ", $op);
                let mode = AbsoluteIndexedY;
                $this.eor(&mode, $mem);
            },
            0x5A => {
                print!("opcode {:x} phy ", $op);
                let mode = StackPush;
                $this.phy(&mode, $mem);
            },
            0x5B => {
                print!("opcode {:x} tcd ", $op);
                $this.tcd();
            },
            0x5C => {
                print!("opcode {:x} jmp ", $op);
                let mode = AbsoluteLong { instruction_type: ControlTransfer };
                $this.jmp(&mode, $mem);
            },
            0x5D => {
                print!("opcode {:x} eor ", $op);
                let mode = AbsoluteIndexedX;
                $this.eor(&mode, $mem);
            },
            0x5E => {
                print!("opcode {:x} lsr ", $op);
                let mode = AbsoluteIndexedX;
                $this.lsr(&mode, $mem);
            },
            0x5F => {
                print!("opcode {:x} eor ", $op);
                let mode = AbsoluteLongIndexedX;
                $this.eor(&mode, $mem);
            },
            0x60 => {
                print!("opcode {:x} rts ", $op);
                let mode = StackRTS;
                $this.rts(&mode, $mem);
            },
            0x61 => {
                print!("opcode {:x} adc ", $op);
                let mode = DirectPageIndexedIndirectX;
                $this.adc(&mode, $mem);
            },
            0x62 => {
                print!("opcode {:x} per ", $op);
                let mode = StackProgramCounterRelative;
                $this.per(&mode, $mem);
            },
            0x63 => {
                print!("opcode {:x} adc ", $op);
                let mode = StackRelative;
                $this.adc(&mode, $mem);
            },
            0x64 => {
                print!("opcode {:x} stz ", $op);
                let mode = DirectPage;
                $this.stz(&mode, $mem);
            },
            0x65 => {
                print!("opcode {:x} adc ", $op);
                let mode = DirectPage;
                $this.adc(&mode, $mem);
            },
            0x66 => {
                print!("opcode {:x} ror ", $op);
                let mode = DirectPage;
                $this.ror(&mode, $mem);
            },
            0x67 => {
                print!("opcode {:x} adc ", $op);
                let mode = DirectPageIndirectLong;
                $this.adc(&mode, $mem);
            },
            0x68 => {
                print!("opcode {:x} pla ", $op);
                let mode = StackPull;
                $this.pla(&mode, $mem);
            },
            0x69 => {
                print!("opcode {:x} adc ", $op);
                let mode = Immediate;
                $this.adc(&mode, $mem);
            },
            0x6A => {
                print!("opcode {:x} ror ", $op);
                let mode = Accumulator;
                $this.ror(&mode, $mem);
            },
            0x6B => {
                print!("opcode {:x} rtl ", $op);
                let mode = StackRTL;
                $this.rtl(&mode, $mem);
            },
            0x6C => {
                print!("opcode {:x} jmp ", $op);
                let mode = AbsoluteIndirect;
                $this.jmp(&mode, $mem);
            },
            0x6D => {
                print!("opcode {:x} adc ", $op);
                let mode = Absolute { instruction_type: LocatingData };
                $this.adc(&mode, $mem);
            },
            0x6E => {
                print!("opcode {:x} ror ", $op);
                let mode = Absolute { instruction_type: LocatingData };
                $this.ror(&mode, $mem);
            },
            0x6F => {
                print!("opcode {:x} adc ", $op);
                let mode = AbsoluteLong { instruction_type: LocatingData };
                $this.adc(&mode, $mem);
            },
            0x70 => {
                print!("opcode {:x} bvs ", $op);
                let mode = ProgramCounterRelative;
                $this.bvs(&mode, $mem);
            },
            0x71 => {
                print!("opcode {:x} adc ", $op);
                let mode = DirectPageIndirectIndexedY;
                $this.adc(&mode, $mem);
            },
            0x72 => {
                print!("opcode {:x} adc ", $op);
                let mode = DirectPageIndirect;
                $this.adc(&mode, $mem);
            },
            0x73 => {
                print!("opcode {:x} adc ", $op);
                let mode = StackRelativeIndirectIndexedY;
                $this.adc(&mode, $mem);
            },
            0x74 => {
                print!("opcode {:x} stz ", $op);
                let mode = StackRelativeIndirectIndexedY;
                $this.stz(&mode, $mem);
            },
            0x75 => {
                print!("opcode {:x} adc ", $op);
                let mode = DirectPageIndexedX;
                $this.adc(&mode, $mem);
            },
            0x76 => {
                print!("opcode {:x} ror ", $op);
                let mode = DirectPageIndexedX;
                $this.ror(&mode, $mem);
            },
            0x77 => {
                print!("opcode {:x} adc ", $op);
                let mode = DirectPageIndirectLongIndexedY;
                $this.adc(&mode, $mem);
            },
            0x78 => {
                print!("opcode {:x} sei ", $op);
                $this.sei();
            },
            0x79 => {
                print!("opcode {:x} adc ", $op);
                let mode = AbsoluteIndexedY;
                $this.adc(&mode, $mem);
            },
            0x7A => {
                print!("opcode {:x} ply ", $op);
                let mode = StackPull;
                $this.ply(&mode, $mem);
            },
            0x7B => {
                print!("opcode {:x} tdc ", $op);
                $this.tdc();
            },
            0x7C => {
                print!("opcode {:x} jmp ", $op);
                let mode = AbsoluteIndexedIndirect;
                $this.jmp(&mode, $mem);
            },
            0x7D => {
                print!("opcode {:x} adc ", $op);
                let mode = AbsoluteIndexedX;
                $this.adc(&mode, $mem);
            },
            0x7E => {
                print!("opcode {:x} ror ", $op);
                let mode = AbsoluteIndexedX;
                $this.ror(&mode, $mem);
            },
            0x7F => {
                print!("opcode {:x} adc ", $op);
                let mode = AbsoluteLongIndexedX;
                $this.adc(&mode, $mem);
            },
            0x80 => {
                print!("opcode {:x} bra ", $op);
                let mode = ProgramCounterRelative;
                $this.bra(&mode, $mem);
            },
            0x81 => {
                print!("opcode {:x} sta ", $op);
                let mode = DirectPageIndexedIndirectX;
                $this.sta(&mode, $mem);
            },
            0x82 => {
                print!("opcode {:x} brl ", $op);
                let mode = ProgramCounterRelativeLong;
                $this.brl(&mode, $mem);
            },
            0x83 => {
                print!("opcode {:x} sta ", $op);
                let mode = StackRelative;
                $this.sta(&mode, $mem);
            },
            0x84 => {
                print!("opcode {:x} sty ", $op);
                let mode = DirectPage;
                $this.sty(&mode, $mem);
            },
            0x85 => {
                print!("opcode {:x} sta ", $op);
                let mode = DirectPage;
                $this.sta(&mode, $mem);
            },
            0x86 => {
                print!("opcode {:x} stx ", $op);
                let mode = DirectPage;
                $this.stx(&mode, $mem);
            },
            0x87 => {
                print!("opcode {:x} sta ", $op);
                let mode = DirectPageIndirectLong;
                $this.sta(&mode, $mem);
            },
            0x88 => {
                print!("opcode {:x} dey ", $op);
                $this.dey();
            },
            0x89 => {
                print!("opcode {:x} bit ", $op);
                let mode = Immediate;
                $this.bit(&mode, $mem);
            },
            0x8A => {
                print!("opcode {:x} txa ", $op);
                $this.txa();
            },
            0x8B => {
                print!("opcode {:x} phb ", $op);
                let mode = StackPush;
                $this.phb(&mode, $mem);
            },
            0x8C => {
                print!("opcode {:x} sty ", $op);
                let mode = Absolute { instruction_type: LocatingData };
                $this.sty(&mode, $mem);
            },
            0x8D => {
                print!("opcode {:x} sta ", $op);
                let mode = Absolute { instruction_type: LocatingData };
                $this.sta(&mode, $mem);
            },
            0x8E => {
                print!("opcode {:x} stx ", $op);
                let mode = Absolute { instruction_type: LocatingData };
                $this.stx(&mode, $mem);
            },
            0x8F => {
                print!("opcode {:x} sta ", $op);
                let mode = AbsoluteLong { instruction_type: LocatingData };
                $this.sta(&mode, $mem);
            },
            0x90 => {
                print!("opcode {:x} bcc ", $op);
                let mode = ProgramCounterRelative;
                $this.bcc(&mode, $mem);
            },
            0x91 => {
                print!("opcode {:x} sta ", $op);
                let mode = DirectPageIndirectIndexedY;
                $this.sta(&mode, $mem);
            },
            0x92 => {
                print!("opcode {:x} sta ", $op);
                let mode = DirectPageIndirect;
                $this.sta(&mode, $mem);
            },
            0x93 => {
                print!("opcode {:x} sta ", $op);
                let mode = StackRelativeIndirectIndexedY;
                $this.sta(&mode, $mem);
            },
            0x94 => {
                print!("opcode {:x} sty ", $op);
                let mode = DirectPageIndexedX;
                $this.sty(&mode, $mem);
            },
            0x95 => {
                print!("opcode {:x} sta ", $op);
                let mode = DirectPageIndexedX;
                $this.sta(&mode, $mem);
            },
            0x96 => {
                print!("opcode {:x} stx ", $op);
                let mode = DirectPageIndexedY;
                $this.stx(&mode, $mem);
            },
            0x97 => {
                print!("opcode {:x} sta ", $op);
                let mode = DirectPageIndirectLongIndexedY;
                $this.sta(&mode, $mem);
            },
            0x98 => {
                print!("opcode {:x} tya ", $op);
                $this.tya();
            },
            0x99 => {
                print!("opcode {:x} sta ", $op);
                let mode = AbsoluteIndexedY;
                $this.sta(&mode, $mem);
            },
            0x9A => {
                print!("opcode {:x} txs ", $op);
                $this.txs();
            },
            0x9B => {
                print!("opcode {:x} txy ", $op);
                $this.txy();
            },
            0x9C => {
                print!("opcode {:x} stz ", $op);
                let mode = Absolute { instruction_type: LocatingData };
                $this.stz(&mode, $mem);
            },
            0x9D => {
                print!("opcode {:x} sta ", $op);
                let mode = AbsoluteIndexedX;
                $this.sta(&mode, $mem);
            },
            0x9E => {
                print!("opcode {:x} stz ", $op);
                let mode = AbsoluteIndexedX;
                $this.stz(&mode, $mem);
            },
            0x9F => {
                print!("opcode {:x} sta ", $op);
                let mode = AbsoluteLongIndexedX;
                $this.sta(&mode, $mem);
            },
            0xA0 => {
                print!("opcode {:x} ldy ", $op);
                let mode = Immediate;
                $this.ldy(&mode, $mem);
            },
            0xA1 => {
                print!("opcode {:x} lda ", $op);
                let mode = DirectPageIndexedIndirectX;
                $this.lda(&mode, $mem);
            },
            0xA2 => {
                print!("opcode {:x} ldx ", $op);
                let mode = Immediate;
                $this.ldx(&mode, $mem);
            },
            0xA3 => {
                print!("opcode {:x} lda ", $op);
                let mode = StackRelative;
                $this.lda(&mode, $mem);
            },
            0xA4 => {
                print!("opcode {:x} ldy ", $op);
                let mode = DirectPage;
                $this.ldy(&mode, $mem);
            },
            0xA5 => {
                print!("opcode {:x} lda ", $op);
                let mode = DirectPage;
                $this.lda(&mode, $mem);
            },
            0xA6 => {
                print!("opcode {:x} ldx ", $op);
                let mode = DirectPage;
                $this.ldx(&mode, $mem);
            },
            0xA7 => {
                print!("opcode {:x} lda ", $op);
                let mode = DirectPageIndirectLong;
                $this.lda(&mode, $mem);
            },
            0xA8 => {
                print!("opcode {:x} tay ", $op);
                $this.tay();
            },
            0xA9 => {
                print!("opcode {:x} lda ", $op);
                let mode = Immediate;
                $this.lda(&mode, $mem);
            },
            0xAA => {
                print!("opcode {:x} tax ", $op);
                $this.tax();
            },
            0xAB => {
                print!("opcode {:x} plb ", $op);
                let mode = StackPull;
                $this.plb(&mode, $mem);
            },
            0xAC => {
                print!("opcode {:x} ldy ", $op);
                let mode = Absolute { instruction_type: LocatingData };
                $this.ldy(&mode, $mem);
            },
            0xAD => {
                print!("opcode {:x} lda ", $op);
                let mode = Absolute { instruction_type: LocatingData };
                $this.lda(&mode, $mem);
            },
            0xAE => {
                print!("opcode {:x} ldx ", $op);
                let mode = Absolute { instruction_type: LocatingData };
                $this.ldx(&mode, $mem);
            },
            0xAF => {
                print!("opcode {:x} lda ", $op);
                let mode = AbsoluteLong { instruction_type: LocatingData };
                $this.lda(&mode, $mem);
            },
            0xB0 => {
                print!("opcode {:x} bcs ", $op);
                let mode = ProgramCounterRelative;
                $this.bcs(&mode, $mem);
            },
            0xB1 => {
                print!("opcode {:x} lda ", $op);
                let mode = DirectPageIndirectIndexedY;
                $this.lda(&mode, $mem);
            },
            0xB2 => {
                print!("opcode {:x} lda ", $op);
                let mode = DirectPageIndirect;
                $this.lda(&mode, $mem);
            },
            0xB3 => {
                print!("opcode {:x} lda ", $op);
                let mode = StackRelativeIndirectIndexedY;
                $this.lda(&mode, $mem);
            },
            0xB4 => {
                print!("opcode {:x} ldy ", $op);
                let mode = DirectPageIndexedX;
                $this.ldy(&mode, $mem);
            },
            0xB5 => {
                print!("opcode {:x} lda ", $op);
                let mode = DirectPageIndexedX;
                $this.lda(&mode, $mem);
            },
            0xB6 => {
                print!("opcode {:x} ldx ", $op);
                let mode = DirectPageIndexedY;
                $this.ldx(&mode, $mem);
            },
            0xB7 => {
                print!("opcode {:x} lda ", $op);
                let mode = DirectPageIndirectLongIndexedY;
                $this.lda(&mode, $mem);
            },
            0xB8 => {
                print!("opcode {:x} clv ", $op);
                $this.clv();
            },
            0xB9 => {
                print!("opcode {:x} lda ", $op);
                let mode = AbsoluteIndexedY;
                $this.lda(&mode, $mem);
            },
            0xBA => {
                print!("opcode {:x} tsx ", $op);
                $this.tsx();
            },
            0xBB => {
                print!("opcode {:x} tyx ", $op);
                $this.tyx();
            },
            0xBC => {
                print!("opcode {:x} ldy ", $op);
                let mode = AbsoluteIndexedX;
                $this.ldy(&mode, $mem);
            },
            0xBD => {
                print!("opcode {:x} lda ", $op);
                let mode = AbsoluteIndexedX;
                $this.lda(&mode, $mem);
            },
            0xBE => {
                print!("opcode {:x} ldx ", $op);
                let mode = AbsoluteIndexedY;
                $this.ldx(&mode, $mem);
            },
            0xBF => {
                print!("opcode {:x} lda ", $op);
                let mode = AbsoluteLongIndexedX;
                $this.lda(&mode, $mem);
            },
            0xC0 => {
                print!("opcode {:x} cpy ", $op);
                let mode = Immediate;
                $this.cpy(&mode, $mem);
            },
            0xC1 => {
                print!("opcode {:x} cmp ", $op);
                let mode = DirectPageIndexedIndirectX;
                $this.cmp(&mode, $mem);
            },
            0xC2 => {
                print!("opcode {:x} rep ", $op);
                let mode = Immediate;
                $this.rep(&mode, $mem);
            },
            0xC3 => {
                print!("opcode {:x} cmp ", $op);
                let mode = StackRelative;
                $this.cmp(&mode, $mem);
            },
            0xC4 => {
                print!("opcode {:x} cpy ", $op);
                let mode = DirectPage;
                $this.cpy(&mode, $mem);
            },
            0xC5 => {
                print!("opcode {:x} cmp ", $op);
                let mode = DirectPage;
                $this.cmp(&mode, $mem);
            },
            0xC6 => {
                print!("opcode {:x} dec ", $op);
                let mode = DirectPage;
                $this.dec(&mode, $mem);
            },
            0xC7 => {
                print!("opcode {:x} cmp ", $op);
                let mode = DirectPageIndirectLong;
                $this.cmp(&mode, $mem);
            },
            0xC8 => {
                print!("opcode {:x} iny ", $op);
                $this.iny();
            },
            0xC9 => {
                print!("opcode {:x} cmp ", $op);
                let mode = Immediate;
                $this.cmp(&mode, $mem);
            },
            0xCA => {
                print!("opcode {:x} dex ", $op);
                $this.dex();
            },
            0xCB => {
                print!("opcode {:x} wai ", $op);
                $this.wai();
            },
            0xCC => {
                print!("opcode {:x} cpy ", $op);
                let mode = Absolute { instruction_type: LocatingData };
                $this.cpy(&mode, $mem);
            },
            0xCD => {
                print!("opcode {:x} cmp ", $op);
                let mode = Absolute { instruction_type: LocatingData };
                $this.cmp(&mode, $mem);
            },
            0xCE => {
                print!("opcode {:x} dec ", $op);
                let mode = Absolute { instruction_type: LocatingData };
                $this.dec(&mode, $mem);
            },
            0xCF => {
                print!("opcode {:x} cmp ", $op);
                let mode = AbsoluteLong { instruction_type: LocatingData };
                $this.cmp(&mode, $mem);
            },
            0xD0 => {
                print!("opcode {:x} bne ", $op);
                let mode = ProgramCounterRelative;
                $this.bne(&mode, $mem);
            },
            0xD1 => {
                print!("opcode {:x} cmp ", $op);
                let mode = DirectPageIndirectIndexedY;
                $this.cmp(&mode, $mem);
            },
            0xD2 => {
                print!("opcode {:x} cmp ", $op);
                let mode = DirectPageIndirect;
                $this.cmp(&mode, $mem);
            },
            0xD3 => {
                print!("opcode {:x} cmp ", $op);
                let mode = StackRelativeIndirectIndexedY;
                $this.cmp(&mode, $mem);
            },
            0xD4 => {
                print!("opcode {:x} pei ", $op);
                let mode = StackDirectPageIndirect;
                $this.pei(&mode, $mem);
            },
            0xD5 => {
                    print!("opcode {:x} cmp ", $op);
                let mode = DirectPageIndexedX;
                $this.cmp(&mode, $mem)
            },
            0xD6 => {
                print!("opcode {:x} dec ", $op);
                let mode = DirectPageIndexedX;
                $this.dec(&mode, $mem);
            },
            0xD7 => {
                print!("opcode {:x} cmp ", $op);
                let mode = DirectPageIndirectLongIndexedY;
                $this.cmp(&mode, $mem);
            },
            0xD8 => {
                print!("opcode {:x} cld ", $op);
                $this.cld();
            },
            0xD9 => {
                print!("opcode {:x} cmp ", $op);
                let mode = AbsoluteIndexedY;
                $this.cmp(&mode, $mem);
            },
            0xDA => {
                print!("opcode {:x} phx ", $op);
                let mode = StackPush;
                $this.phx(&mode, $mem);
            },
            0xDB => {
                print!("opcode {:x} stp ", $op);
                $this.stp();
            },
            0xDC => {
                print!("opcode {:x} jmp ", $op);
                let mode = AbsoluteIndirectLong;
                $this.jmp(&mode, $mem);
            },
            0xDD => {
                print!("opcode {:x} cmp ", $op);
                let mode = AbsoluteIndexedX;
                $this.cmp(&mode, $mem);
            },
            0xDE => {
                print!("opcode {:x} dec ", $op);
                let mode = AbsoluteIndexedX;
                $this.dec(&mode, $mem);
            },
            0xDF => {
                print!("opcode {:x} cmp ", $op);
                let mode = AbsoluteLongIndexedX;
                $this.cmp(&mode, $mem);
            },
            0xE0 => {
                print!("opcode {:x} cpx ", $op);
                let mode = Immediate;
                $this.cpx(&mode, $mem);
            },
            0xE1 => {
                print!("opcode {:x} sbc ", $op);
                let mode = DirectPageIndexedIndirectX;
                $this.sbc(&mode, $mem);
            },
            0xE2 => {
                print!("opcode {:x} sep ", $op);
                let mode = Immediate;
                $this.sep(&mode, $mem);
            },
            0xE3 => {
                print!("opcode {:x} sbc ", $op);
                let mode = DirectPage;
                $this.sbc(&mode, $mem);
            },
            0xE4 => {
                print!("opcode {:x} cpx ", $op);
                let mode = DirectPage;
                $this.cpx(&mode, $mem);
            },
            0xE5 => {
                print!("opcode {:x} sbc ", $op);
                let mode = DirectPage;
                $this.sbc(&mode, $mem);
            },
            0xE6 => {
                print!("opcode {:x} inc ", $op);
                let mode = DirectPage;
                $this.inc(&mode, $mem);
            },
            0xE7 => {
                print!("opcode {:x} sbc ", $op);
                let mode = DirectPageIndirectLong;
                $this.sbc(&mode, $mem);
            },
            0xE8 => {
                print!("opcode {:x} inx ", $op);
                $this.inx();
            },
            0xE9 => {
                print!("opcode {:x} sbc ", $op);
                let mode = Immediate;
                $this.sbc(&mode, $mem);
            },
            0xEA => {
                print!("opcode {:x} nop ", $op);
                $this.nop();
            },
            0xEB => {
                print!("opcode {:x} xba ", $op);
                $this.xba();
            },
            0xEC => {
                print!("opcode {:x} cpx ", $op);
                let mode = Absolute { instruction_type: LocatingData };
                $this.cpx(&mode, $mem);
            },
            0xED => {
                print!("opcode {:x} sbc ", $op);
                let mode = Absolute { instruction_type: LocatingData };
                $this.sbc(&mode, $mem);
            },
            0xEE => {
                print!("opcode {:x} inc ", $op);
                let mode = Absolute { instruction_type: LocatingData };
                $this.inc(&mode, $mem);
            },
            0xEF => {
                print!("opcode {:x} sbc ", $op);
                let mode = AbsoluteLong { instruction_type: LocatingData };
                $this.sbc(&mode, $mem);
            },
            0xF0 => {
                print!("opcode {:x} beq ", $op);
                let mode = ProgramCounterRelative;
                $this.beq(&mode, $mem);
            },
            0xF1 => {
                print!("opcode {:x} sbc ", $op);
                let mode = DirectPageIndirectIndexedY;
                $this.sbc(&mode, $mem);
            },
            0xF2 => {
                print!("opcode {:x} sbc ", $op);
                let mode = DirectPageIndirect;
                $this.sbc(&mode, $mem);
            },
            0xF3 => {
                print!("opcode {:x} sbc ", $op);
                let mode = StackRelativeIndirectIndexedY;
                $this.sbc(&mode, $mem);
            },
            0xF4 => {
                print!("opcode {:x} pea ", $op);
                let mode = StackAbsolute;
                $this.pea(&mode, $mem);
            },
            0xF5 => {
                print!("opcode {:x} sbc ", $op);
                let mode = DirectPageIndexedX;
                $this.sbc(&mode, $mem);
            },
            0xF6 => {
                print!("opcode {:x} inc ", $op);
                let mode = DirectPageIndexedX;
                $this.inc(&mode, $mem);
            },
            0xF7 => {
                print!("opcode {:x} sbc ", $op);
                let mode = DirectPageIndirectLongIndexedY;
                $this.sbc(&mode, $mem);
            },
            0xF8 => {
                print!("opcode {:x} sed ", $op);
                $this.sed();
            },
            0xF9 => {
                print!("opcode {:x} sbc ", $op);
                let mode = AbsoluteIndexedY;
                $this.sbc(&mode, $mem);
            },
            0xFA => {
                print!("opcode {:x} plx ", $op);
                let mode = StackPull;
                $this.plx(&mode, $mem);
            },
            0xFB => {
                print!("opcode {:x} xce ", $op);
                $this.xce();
            },
            0xFC => {
                print!("opcode {:x} jsr ", $op);
                let mode = AbsoluteIndexedIndirect;
                $this.jsr(&mode, $mem, false);
            },
            0xFD => {
                print!("opcode {:x} sbc ", $op);
                let mode = AbsoluteIndexedX;
                $this.sbc(&mode, $mem);
            },
            0xFE => {
                print!("opcode {:x} inc ", $op);
                let mode = AbsoluteIndexedX;
                $this.inc(&mode, $mem);
            },
            0xFF => {
                print!("opcode {:x} sbc ", $op);
                let mode = AbsoluteLongIndexedX;
                $this.sbc(&mode, $mem);
            },
            _ => panic!("{:x} is not an opcode", $op),
        }
    );
}

pub struct CPU {
    pub accumulator:                    u16,
    pub index_x:                        u16,
    pub index_y:                        u16,
    pub stack_pointer:                usize,
    pub data_bank:                    usize,
    pub direct_page:                  usize,
    pub program_bank:                 usize,
    pub processor_status:   ProcessorStatus,
    pub program_counter:              usize,
    pub emulation_mode:                bool,
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
        self.run_instruction(memory);
        //println!("{:?}\n{:?}", self, memory);
    }

    fn run_instruction(&mut self, memory: &mut Memory) {
        let addr = (self.program_bank << 16) | self.program_counter;

        let opcode = memory.get_byte(addr);

        self.program_counter = self.program_counter.wrapping_add(1);
        decode_op_and_execute!(opcode, self, memory);
        println!("at address {:x}", addr);
    }

    fn brk<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        use self::StatusFlags::IndexRegisterSize; // This is the break flag in emulation mode
        use self::StatusFlags::{IRQDisable, Decimal};

        let pc = (self.program_counter.wrapping_add(1)) as u32;
        let interrupt_vector = memory.get_interrupt_vector(self.emulation_mode);

        if self.emulation_mode {
            self.processor_status.set_flag(IndexRegisterSize, true);
            let ps = self.processor_status.as_byte() as u32;

            mode.store(self, memory, !IS_BYTE, pc);    
            mode.store(self, memory, IS_BYTE, ps);
        } else {
            let pb = self.program_bank as u32;
            let ps = self.processor_status.as_byte() as u32;

            mode.store(self, memory, IS_BYTE, pb);
            mode.store(self, memory, !IS_BYTE, pc);
            mode.store(self, memory, IS_BYTE, ps);

            self.program_bank = 0;
        }

        self.processor_status.set_flag(IRQDisable, true);
        self.processor_status.set_flag(Decimal, false); // Cleared "after" the break, but when is "after"?

        self.program_counter = interrupt_vector;
    }

    fn ora<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        use self::StatusFlags::{Negative, Zero, AccumulatorRegisterSize};

        let emu = self.processor_status.get_flag(AccumulatorRegisterSize);
        let data = mode.load(self, memory, emu);

        self.accumulator |= data as u16;
        let result = self.accumulator;

        self.processor_status.set_flag(Negative, result.rotate_left(1) & 1 == 1);
        self.processor_status.set_flag(Zero, self.accumulator == 0);
    }

    fn tsb<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("tsb unimplemented")
    }

    fn asl<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("asl unimplemented")
    }

    fn php<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        let data = self.processor_status.as_byte() as u32;

        mode.store(self, memory, IS_BYTE, data);
    }

    fn phd<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        let data = self.direct_page as u32;

        mode.store(self, memory, !IS_BYTE, data);
    }

    fn bpl<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        use self::StatusFlags::Negative;
        
        if !self.processor_status.get_flag(Negative) {
            let data = mode.load(self, memory, IS_BYTE) as i8;

            if data < 0 {
                self.program_counter = self.program_counter.wrapping_sub((!(data as usize)).wrapping_add(1));
            } else {
                self.program_counter = self.program_counter.wrapping_add(data as usize);
            }
        }

        self.program_counter = self.program_counter.wrapping_add(1);
    }

    fn trb<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("trb unimplemented")
    }

    fn clc(&mut self) {
        use self::StatusFlags::Carry;

        self.processor_status.set_flag(Carry, false);
    }

    fn inc<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        use self::StatusFlags::{AccumulatorRegisterSize, Negative, Zero};

        let emu = self.processor_status.get_flag(AccumulatorRegisterSize);
        let data = mode.load(self, memory, emu).wrapping_add(1);
        mode.store(self, memory, emu, data);

        let negative_check = if emu {
            ((data & 0x80) >> 7) == 1
        } else {
            ((data & 0x8000) >> 15) == 1
        };

        self.processor_status.set_flag(Negative, negative_check);
        self.processor_status.set_flag(Zero, data == 0);
    }

    fn tcs(&mut self) {
        panic!("tcs unimplemented")
    }

    fn jsr<T: Instruction>(&mut self, mode: &T, memory: &mut Memory, is_long: bool) {

        // PC starts just past opcode at this point, but before we push it onto the stack
        // it must be pointing at the last byte of the operand, which is either two or three
        // past the opcode depending on the addressing mode
        let pc = (if is_long { self.program_counter.wrapping_add(2) } else { self.program_counter.wrapping_add(1) }) as u32;
        let push = StackPush;

        if is_long {
            let pb = self.program_bank as u32;
            push.store(self, memory, IS_BYTE, pb);
        }

        push.store(self, memory, !IS_BYTE, pc);

        let (bank, addr) = {
            let tmp_addr = mode.load(self, memory, !IS_BYTE) as usize;
            let bank = (tmp_addr & 0xFF0000) >> 16;

            (bank, tmp_addr & 0x00FFFF)
        };

        self.program_counter = addr;
        
        if is_long {
            self.program_bank = bank;
        }

    //    println!("jsr addr {:x}{:x}", bank, addr);
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

        self.direct_page = mode.load(self, memory, !IS_BYTE) as usize;

        let negative_check = ((self.direct_page & 0x8000) >> 15) == 1;

        self.processor_status.set_flag(Negative, negative_check);
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

        let data = self.accumulator as u32;

        let emu = self.processor_status.get_flag(AccumulatorRegisterSize);
        mode.store(self, memory, emu, data);
    }

    fn phk<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        let data = self.program_bank as u32;
        mode.store(self, memory, IS_BYTE, data);
    }

    fn jmp<T: Instruction>(&mut self, mode: &T, memory: &Memory) {
        let jump_addr = mode.load(self, memory, !IS_BYTE) as usize;
        self.program_counter = jump_addr & 0x00FFFF;
        self.program_bank = (jump_addr & 0xFF0000) >> 16;
    //    println!("jump_addr {:x}", jump_addr);
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
        use self::StatusFlags::IndexRegisterSize;

        let data = self.index_y as u32;

        let emu = self.processor_status.get_flag(IndexRegisterSize);
        mode.store(self, memory, emu, data);
    }

    fn tcd(&mut self) {
        use self::StatusFlags::{Negative, Zero};

        self.direct_page = self.accumulator as usize;

        self.processor_status.set_flag(Negative, (self.accumulator as i16).is_negative());
        self.processor_status.set_flag(Zero, self.accumulator == 0);
    }

    fn rts<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        let addr = mode.load(self, memory, !IS_BYTE) as usize;

        self.program_counter = addr.wrapping_add(1);

        //println!("rts addr {:x}", addr + 1);
    }

    fn adc<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        use self::StatusFlags::{AccumulatorRegisterSize, Negative, Overflow, Zero, Carry};

        let emu = self.processor_status.get_flag(AccumulatorRegisterSize);
        let data = mode.load(self, memory, emu);

        let mut res = if emu {
            ((self.accumulator as u8) as u32).wrapping_add(data as u32)
        } else {
            (self.accumulator.wrapping_add(data as u16)) as u32
        };

        if self.processor_status.get_flag(Carry) {
            res = res.wrapping_add(1);
        }

        let (unsigned_overflow_check, signed_overflow_check) = if emu {
            self.accumulator = (res as u8) as u16;
            
            (((res & 0x100) >> 8) == 1, ((res & 0x80) >> 7) == 0)
        } else {
            self.accumulator = res as u16;

            (((res & 0x10000) >> 16) == 1, ((res & 0x8000) >> 15) == 0)
        };

        if emu {
            self.processor_status.set_flag(Negative, (res as u16).rotate_left(1) & 1 == 1);
        } else {
            self.processor_status.set_flag(Negative, (res as u16).rotate_left(1) & 1 == 1);
        }

        self.processor_status.set_flag(Overflow, signed_overflow_check);
        self.processor_status.set_flag(Zero, res == 0);
        self.processor_status.set_flag(Carry, unsigned_overflow_check);
    }

    fn per<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("per unimplemented")
    }

    fn stz<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        use self::StatusFlags::AccumulatorRegisterSize;

        let emu = self.processor_status.get_flag(AccumulatorRegisterSize);

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
        self.accumulator = self.direct_page as u16;

        self.processor_status.set_flag(Negative, self.accumulator.rotate_left(1) & 1 == 1);
        self.processor_status.set_flag(Zero, self.accumulator == 0);
    }

    fn bra<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("bra unimplemented")
    }

    fn sta<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        use self::StatusFlags::AccumulatorRegisterSize;

        let data = self.accumulator as u32;

        let emu = self.processor_status.get_flag(AccumulatorRegisterSize);
        mode.store(self, memory, emu, data);
    }

    fn brl<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("brl unimplemented")
    }

    fn sty<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        use self::StatusFlags::IndexRegisterSize;
        
        let data = self.index_y as u32;

        let emu = self.processor_status.get_flag(IndexRegisterSize);
        mode.store(self, memory, emu, data);
    }

    fn stx<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        use self::StatusFlags::IndexRegisterSize;

        let data = self.index_x as u32;

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
        let data = self.data_bank as u32;
        mode.store(self, memory, IS_BYTE, data);
    }

    fn bcc<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("bcc unimplemented")
    }

    fn tya(&mut self) {
        panic!("tya unimplemented")
    }

    fn txs(&mut self) {
        use self::StatusFlags::IndexRegisterSize;

        if self.emulation_mode || self.processor_status.get_flag(IndexRegisterSize) {
            self.stack_pointer = (0xFF & self.index_x) as usize;
        } else {
            self.stack_pointer = self.index_x as usize;
        }
    }

    fn txy(&mut self) {
        panic!("txy unimplemented")
    }

    fn ldy<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        use self::StatusFlags::{Negative, Zero, IndexRegisterSize};
        
        let emu = self.processor_status.get_flag(IndexRegisterSize);
        let data = mode.load(self, memory, emu);

        self.index_y = data as u16;

        self.processor_status.set_flag(Negative, data.rotate_left(1) & 1 == 1);
        self.processor_status.set_flag(Zero, self.index_y == 0);
    }

    fn lda<T: Instruction>(&mut self, mode: &T, memory: &Memory) {
        use self::StatusFlags::{AccumulatorRegisterSize, Negative, Zero};

        let emu = self.processor_status.get_flag(AccumulatorRegisterSize);
        let data = mode.load(self, memory, emu);
        self.accumulator = data as u16;
        
        self.processor_status.set_flag(Negative, data.rotate_left(1) & 1 == 1);
        self.processor_status.set_flag(Zero, self.accumulator == 0);
    }

    fn ldx<T: Instruction>(&mut self, mode: &T, memory: &Memory) {
        use self::StatusFlags::{IndexRegisterSize, Zero, Negative};

        let emu = self.processor_status.get_flag(IndexRegisterSize);
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

        self.processor_status.set_flag(Negative, self.index_x.rotate_left(1) & 1 == 1);
        self.processor_status.set_flag(Zero, self.index_x == 0);
    }

    fn plb<T: Instruction>(&mut self, mode: &T, memory: &Memory) {
        use self::StatusFlags::{Negative, Zero};

        let data = mode.load(self, memory, IS_BYTE);
        self.data_bank = data as usize;

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
        use self::StatusFlags::{Negative, Zero, Carry, IndexRegisterSize};

        let emu = self.processor_status.get_flag(IndexRegisterSize);
        let data = mode.load(self, memory, emu) as u16;

        let result = data.wrapping_sub(self.index_y) as i16;

        self.processor_status.set_flag(Negative, result.is_negative());
        self.processor_status.set_flag(Zero, result == 0);
        self.processor_status.set_flag(Carry, self.index_y >= data);
    }

    fn cmp<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        use self::StatusFlags::{Negative, Zero, Carry, AccumulatorRegisterSize};

        let emu = self.processor_status.get_flag(AccumulatorRegisterSize);
        let data = mode.load(self, memory, emu);

        let (result, carry) = if emu {
            let res = ((self.accumulator as u8) as u16).wrapping_sub(data as u16);

            (res, (self.accumulator as u8) >= (data as u8))
        } else {
            let res = ((self.accumulator as u32).wrapping_sub(data as u32)) as u16;

            (res, self.accumulator >= (data as u16))
        };

        self.processor_status.set_flag(Negative, result.rotate_left(1) & 1 == 1);
        self.processor_status.set_flag(Zero, result == 0);
        self.processor_status.set_flag(Carry, carry);
    }

    fn rep<T: Instruction>(&mut self, mode: &T, memory: &Memory) {
        let val = mode.load(self, memory, IS_BYTE);

        self.processor_status.set_by_byte(self.emulation_mode, val as u8, false);
    }

    fn iny(&mut self) {
        use self::StatusFlags::{Negative, Zero};

        self.index_y = self.index_y.wrapping_add(1);

        self.processor_status.set_flag(Negative, (self.index_y as i16).is_negative());
        self.processor_status.set_flag(Zero, self.index_y == 0);
    }

    fn dex(&mut self) {
        use self::StatusFlags::{Negative, Zero};
        
        self.index_x = self.index_x.wrapping_sub(1);

        self.processor_status.set_flag(Negative, (self.index_x as i16).is_negative());
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
                self.program_counter = self.program_counter.wrapping_sub((!(data as usize)).wrapping_add(1));
            } else {
                self.program_counter = self.program_counter.wrapping_add(data as usize);
            }
        }

        self.program_counter = self.program_counter.wrapping_add(1);
    }

    fn pei<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        panic!("pei unimplemented")
    }

    fn cld(&mut self) {
        panic!("cld unimplemented")
    }

    fn phx<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        use self::StatusFlags::IndexRegisterSize;

        let data = self.index_x as u32;

        let emu = self.processor_status.get_flag(IndexRegisterSize);
        mode.store(self, memory, emu, data);
    }

    fn stp(&mut self) {
        panic!("stp unimplemented")
    }

    fn cpx<T: Instruction>(&mut self, mode: &T, memory: &mut Memory) {
        use self::StatusFlags::{Negative, Zero, Carry, IndexRegisterSize};

        let emu = self.processor_status.get_flag(IndexRegisterSize);
        let data = mode.load(self, memory, emu) as u16;

        let result = data.wrapping_sub(self.index_x) as i16;

        self.processor_status.set_flag(Negative, result.is_negative());
        self.processor_status.set_flag(Zero, result == 0);
        self.processor_status.set_flag(Carry, self.index_x >= data);
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
        use self::StatusFlags::{Negative, Zero};

        let high_to_low = (self.accumulator & 0xFF00) >> 8;
        let low_to_high = (self.accumulator & 0x00FF) << 8;

        self.accumulator = 0;
        self.accumulator = low_to_high | high_to_low;

        let negative_check = (self.accumulator & 0x0080) >> 7 == 1;

        self.processor_status.set_flag(Negative, negative_check);
        self.processor_status.set_flag(Zero, high_to_low == 0);
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
        self.emulation_mode = self.processor_status.get_flag(Carry);
        self.processor_status.set_flag(Carry, emu_bit);
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

    pub fn as_byte(&self) -> u8 {
        let mut byte = 0;

        for i in 0..8 {
            if self.status[i] {
                byte |= 1 << i;
            }
        }

        byte
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

