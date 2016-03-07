use std::fmt;

const LOROM_ROM_NAME_START: usize = 0x7FC0;
const LOROM_ROM_MAKEUP_BYTE: usize = 0x7FD5;
const LOROM_FASTROM_VAL: u8 = 0x30;
const EXLOROM_VAL: u8 = 0x32;

const HIROM_ROM_NAME_START: usize = 0xFFC0;
const HIROM_ROM_MAKEUP_BYTE: usize = 0xFFD5;
const HIROM_FASTROM_VAL: u8 = 0x31;
const EXHIROM_VAL: u8 = 0x35;

const HEADERED_OFFSET: usize = 512;

const RAM_SIZE: usize = 128 * 1024;
const VRAM_SIZE: usize = (64 * 1024) + 512 + 32 + (256 * 15); // main VRAM + sprite RAM + palette RAM
const ARAM_SIZE: usize =  64 * 1024;
const SRAM_SIZE: usize = 512 * 1024;

pub const LOROM_NATIVE_MODE_VECTORS: [usize; 6] = [
    0x7FE4, // COP, not used for SNES
    0x7FE6, // BRK
    0x7FE8, // ABORT
    0x7FEA, // NMI, called when vblank begins
    0x7FEC, // unused
    0x7FEE, // Interrupt request
];

pub const HIROM_NATIVE_MODE_VECTORS: [usize; 6] = [
    0xFFE4, // COP, not used for SNES
    0xFFE6, // BRK
    0xFFE8, // ABORT
    0xFFEA, // NMI, called when vblank begins
    0xFFEC, // unused
    0xFFEE, // Interrupt request
];

pub const LOROM_EMU_MODE_VECTORS: [usize; 6] = [
    0x7FF4, // COP, not used for SNES
    0x7FFE, // BRK
    0x7FF8, // ABORT
    0x7FFA, // NMI, called when vblank begins
    0x7FFC, // Reset vector, execution begins via this vector
    0x7FFE, // Interrupt request
];

pub const HIROM_EMU_MODE_VECTORS: [usize; 6] = [
    0xFFF4, // COP, not used for SNES
    0x7FFE, // BRK
    0xFFF8, // ABORT
    0xFFFA, // NMI, called when vblank begins
    0xFFFC, // Reset vector, execution begins via this vector
    0xFFFE, // Interrupt request
];

pub enum Vectors {
    COP = 0,
    BRK = 1,
    ABORT = 2,
    NMI = 3,
    RESET = 4,
    IRQ = 5,
}

pub struct Memory {
	ram: Box<[u8]>,
    pub rom: Rom,
    sram: [u8; SRAM_SIZE],
    bregs: [u8; 68], // address bus B registers
    wramregs: [u8; 4], // wram registers
    jpregs: [u8; 2], // old style joypad registers
    cpuregs: [u8; 32], // internal CPU registers; cannot write to 0x420E or 0x420F
    dmaregs: [u8; 88], // DMA registers
}

impl Memory {
    pub fn new(rom: Vec<u8>) -> Memory {
        Memory {
            ram: vec![0; RAM_SIZE].into_boxed_slice(),
            rom: Rom::new(rom),
            sram: [0; SRAM_SIZE],
            bregs: [0; 68],
            wramregs: [0; 4],
            jpregs: [0; 2],
            cpuregs: [0; 32],
            dmaregs: [0; 88],
        }
    }

    pub fn get_byte(&self, addr: usize) -> u8 {
        use self::RomType::*;

        let header_offset = if self.rom.headered { HEADERED_OFFSET } else { 0 };
        let addr_offset = addr + header_offset;
        let bank = (addr & 0xFF0000) >> 16;
        let bank_header = (addr_offset & 0xFF0000) >> 16;
        let offset = addr & 0xFFFF;
        let offset_header = addr_offset & 0xFFFF;

        println!("addr {:x} h_offset: {:x} addr_offset: {:x} bank {:x} \
                  bank_header {:x} offset {:x} offset_header {:x}",
                 addr, header_offset, addr_offset, bank, bank_header, offset, offset_header);

        match self.rom.rom_type {
            LoROM | FastLoROM => {
                match bank {
                    0x00 ... 0x3F | 0x80 ... 0xBF => {
                        match offset {
                            0x0000 ... 0x1FFF => {
                                self.ram[offset]
                            },
                            0x2000 ... 0x20FF => {
                                unreachable!("Invalid address {:x}", addr)
                            },
                            0x2100 ... 0x21FF => {
                                panic!("Unimplemented: PPU1, APU, hardware registers {:x}", addr)
                            },
                            0x2200 ... 0x2FFF => {
                                unreachable!("Invalid address {:x}", addr)
                            },
                            0x3000 ... 0x3FFF => {
                                panic!("Unimplemented: DSP, SuperFX, hardware registers {:x}", addr)
                            },
                            0x4000 ... 0x40FF => {
                                panic!("Unimplemented: Old style joypad registers {:x}", addr)
                            },
                            0x4100 ... 0x41FF => {
                                unreachable!("Invalid address {:x}", addr)
                            },
                            0x4200 ... 0x420D | 0x4210 ... 0x421F => {
                                let adjusted_offset = offset - 0x4200;

                                self.cpuregs[adjusted_offset]
                            },
                            0x4300 ... 0x430A |
                            0x4310 ... 0x431A |
                            0x4320 ... 0x432A |
                            0x4330 ... 0x433A |
                            0x4340 ... 0x434A |
                            0x4350 ... 0x435A |
                            0x4360 ... 0x436A |
                            0x4370 ... 0x437A => {
                                let adjusted_offset = offset - 0x4300;

                                self.dmaregs[adjusted_offset]
                            },
                            0x4500 ... 0x5FFF => {
                                unreachable!("Invalid address {:x}", addr)
                            },
                            0x6000 ... 0x7FFF => {
                                panic!("Reserved memory {:x}", addr)
                            },
                            0x8000 ... 0xFFFF => {
                                let tmp_bank = if bank_header >= 0x80 {
                                    bank_header - 0x80
                                } else {
                                    bank_header
                                };

                                self.rom.data[(tmp_bank << 16) | offset_header - 0x8000]
                            },
                            _ => unreachable!("Invalid address {:x}", addr_offset)
                        }
                    },
                    0x40 ... 0x6F | 0xC0 ... 0xEF => {
                        match offset {
                            0x0000 ... 0x7FFF => {
                                unreachable!("Invalid address {:x}", addr)
                            },
                            0x8000 ... 0xFFFF => {
                                let tmp_bank = if bank_header >= 0xC0 {
                                    bank_header - 0x80
                                } else {
                                    bank_header
                                };

                                self.rom.data[(tmp_bank * 0x8000) + (offset_header - 0x8000)]
                            },
                            _ => unreachable!("Invalid address {:x}", addr)
                        }
                    },
                    0x70 ... 0x7D | 0xF0 ... 0xFD => {
                        match offset {
                            0x0000 ... 0xFFFF => {
                                panic!("SRAM {:x}", addr)
                            },
                            _ => unreachable!("Invalid address {:x}", addr)
                        }
                    },
                    0x7E => {
                        match offset {
                            0x0000 ... 0x1FFF => {
                                panic!("Unimplemented: LowRAM (WRAM) {:x}", addr)
                            },
                            0x2000 ... 0x7FFF => {
                                panic!("Unimplemented: HighRAM (WRAM) {:x}", addr)
                            },
                            0x8000 ... 0xFFFF => {
                                panic!("Unimplemented: Extended RAM (WRAM) {:x}", addr)
                            },
                            _ => unreachable!("Invalid address {:x}", addr)
                        }
                    },
                    0x7F => {
                        match offset {
                            0x0000 ... 0xFFFF => {
                                panic!("Unimplemented: Extended RAM (WRAM) {:x}", addr)
                            },
                            _ => unreachable!("Invalid address {:x}", addr)
                        }
                    },
                    0xFE ... 0xFF => {
                        match offset {
                            0x0000 ... 0x7FFF => {
                                panic!("SRAM {:x}", addr)
                            },
                            0x8000 ... 0xFFFF => {
                                self.rom.data[(bank_header * 0x8000) + (offset_header - 0x8000)]
                            },
                            _ => unreachable!("Invalid address {:x}", addr)
                        }
                    },
                    _ => unreachable!("Invalid address {:x}", addr)
                }
            },
            HiROM | FastHiROM => {
                match bank {
                    0x00 ... 0x1F | 0x80 ... 0x9F => {
                        match offset {
                            0x0000 ... 0x1FFF => {
                                self.ram[offset]
                            },
                            0x2000 ... 0x20FF => {
                                unreachable!("Invalid address {:x}", addr)
                            },
                            0x2100 ... 0x21FF => {
                                panic!("Unimplemented: PPU1, APU, hardware registers {:x}", addr)
                            },
                            0x2200 ... 0x2FFF => {
                                unreachable!("Invalid address {:x}", addr)
                            },
                            0x3000 ... 0x3FFF => {
                                panic!("Unimplemented: DSP, SuperFX, hardware registers {:x}", addr)
                            },
                            0x4000 ... 0x40FF => {
                                panic!("Unimplemented: Old style joypad registers {:x}", addr)
                            },
                            0x4100 ... 0x41FF => {
                                unreachable!("Invalid address {:x}", addr)
                            },
                            0x4200 ... 0x44FF => {
                                panic!("Unimplemented: DMA, PPU2, hardware registers {:x}", addr)
                            },
                            0x4500 ... 0x5FFF => {
                                unreachable!("Invalid address {:x}", addr)
                            },
                            0x6000 ... 0x7FFF => {
                                panic!("Reserved {:x}", addr)
                            },
                            0x8000 ... 0xFFFF => {
                                let tmp_bank = if bank_header >= 0x80 {
                                    bank_header - 0x80
                                } else {
                                    bank_header
                                };

                                self.rom.data[(tmp_bank << 16) + offset_header]
                            },
                            _ => unreachable!("Invalid address {:x}", addr)
                        }
                    },
                    0x20 ... 0x3F | 0xA0 ... 0xBF => {
                        match offset {
                            0x0000 ... 0x1FFF => {
                                panic!("Unimplemented: LowRAM, shadowed from bank 0x7E {:x}", addr)
                            },
                            0x2000 ... 0x20FF => {
                                unreachable!("Invalid address {:x}", addr)
                            },
                            0x2100 ... 0x21FF => {
                                panic!("Unimplemented: PPU1, APU, hardware registers {:x}", addr)
                            },
                            0x2200 ... 0x2FFF => {
                                unreachable!("Invalid address {:x}", addr)
                            },
                            0x3000 ... 0x3FFF => {
                                panic!("Unimplemented: DSP, SuperFX, hardware registers {:x}", addr)
                            },
                            0x4000 ... 0x40FF => {
                                panic!("Unimplemented: Old style joypad registers {:x}", addr)
                            },
                            0x4100 ... 0x41FF => {
                                unreachable!("Invalid address {:x}", addr)
                            },
                            0x4200 ... 0x44FF => {
                                panic!("Unimplemented: DMA, PPU2, hardware registers {:x}", addr)
                            },
                            0x4500 ... 0x5FFF => {
                                unreachable!("Invalid address {:x}", addr)
                            },
                            0x6000 ... 0x7FFF => {
                                panic!("SRAM {:x}", addr)
                            },
                            0x8000 ... 0xFFFF => {
                                let tmp_bank = if bank_header >= 0xA0 {
                                    bank_header - 0x80
                                } else {
                                    bank_header
                                };

                                self.rom.data[(tmp_bank << 16) + offset_header]
                            },
                            _ => unreachable!("Invalid address {:x}", addr)
                        }
                    },
                    0x40 ... 0x7D | 0xC0 ... 0xFD => {
                        match offset {
                            0x0000 ... 0xFFFF => {
                                let tmp_bank = if bank_header >= 0xC0 {
                                    bank_header - 0xC0
                                } else {
                                    bank_header - 0x40
                                };

                                self.rom.data[(tmp_bank << 16) + offset_header]
                            },
                            _ => unreachable!("Invalid address {:x}", addr)
                        }
                    },
                    0x7E => {
                        match offset {
                            0x0000 ... 0x1FFF => {
                                panic!("Unimplemented: LowRAM (WRAM) {:x}", addr)
                            },
                            0x2000 ... 0x7FFF => {
                                panic!("Unimplemented: HighRAM (WRAM) {:x}", addr)
                            },
                            0x8000 ... 0xFFFF => {
                                panic!("Unimplemented: Expanded RAM (WRAM) {:x}", addr)
                            },
                            _ => unreachable!("Invalid address {:x}", addr)
                        }
                    },
                    0x7F => {
                        match offset {
                            0x0000 ... 0xFFFF => {
                                panic!("Unimplemented: Expanded RAM (WRAM) {:x}", addr)
                            },
                            _ => unreachable!("Invalid address {:x}", addr)
                        }
                    },
                    0xFE ... 0xFF => {
                        match offset {
                            0x0000 ... 0xFFFF => {
                                let tmp_bank = bank_header - 0xC0;
                                self.rom.data[(tmp_bank << 16) + offset_header]
                            },
                            _ => unreachable!("Invalid address {:x}", addr)
                        }
                    },
                    _ => unreachable!("Invalid address {:x}", addr)
                }
            },
            ExLoROM => {
                unimplemented!()
            },
            ExHiROM => {
                unimplemented!()
            },
        }
    }

    pub fn set_byte(&mut self, addr: usize, data: u8) {
        use self::RomType::*;

        let header_offset = if self.rom.headered { HEADERED_OFFSET } else { 0 };
        let addr_offset = addr + header_offset;
        let bank = (addr & 0xFF0000) >> 16;
        let bank_header = (addr_offset & 0xFF0000) >> 16;
        let offset = addr & 0xFFFF;
        let offset_header = addr_offset & 0xFFFF;

        println!("addr {:x} h_offset: {:x} addr_offset: {:x} bank {:x} \
                  bank_header {:x} offset {:x} offset_header {:x} data {:x}",
                 addr, header_offset, addr_offset, bank, bank_header, offset, offset_header, data);

        match self.rom.rom_type {
            LoROM | FastLoROM => {
                match bank {
                    0x00 ... 0x3F | 0x80 ... 0xBF => {
                        match offset {
                            0x0000 ... 0x1FFF => {
                                self.ram[offset] = data;
                            },
                            0x2000 ... 0x20FF => {
                                unreachable!("Invalid address {:x}", addr)
                            },
                            0x2100 ... 0x21FF => {
                                panic!("Unimplemented: PPU1, APU, hardware registers {:x}", addr)
                            },
                            0x2200 ... 0x2FFF => {
                                unreachable!("Invalid address {:x}", addr)
                            },
                            0x3000 ... 0x3FFF => {
                                panic!("Unimplemented: DSP, SuperFX, hardware registers {:x}", addr)
                            },
                            0x4000 ... 0x40FF => {
                                panic!("Unimplemented: Old style joypad registers {:x}", addr)
                            },
                            0x4100 ... 0x41FF => {
                                unreachable!("Invalid address {:x}", addr)
                            },
                            0x4200 ... 0x420D | 0x4210 ... 0x421F => {
                                let adjusted_offset = offset - 0x4200;

                                self.cpuregs[adjusted_offset] = data;
                            },
                            0x4300 ... 0x430A |
                            0x4310 ... 0x431A |
                            0x4320 ... 0x432A |
                            0x4330 ... 0x433A |
                            0x4340 ... 0x434A |
                            0x4350 ... 0x435A |
                            0x4360 ... 0x436A |
                            0x4370 ... 0x437A => {
                                let adjusted_offset = offset - 0x4300;

                                self.dmaregs[adjusted_offset] = data;
                            },
                            0x4500 ... 0x5FFF => {
                                unreachable!("Invalid address {:x}", addr)
                            },
                            0x6000 ... 0x7FFF => {
                                panic!("Reserved memory {:x}", addr)
                            },
                            0x8000 ... 0xFFFF => {
                                panic!("Cannot write to ROM {:x}", addr)
                            },
                            _ => unreachable!("Invalid address {:x}", addr)
                        }
                    },
                    0x40 ... 0x6F | 0xC0 ... 0xEF => {
                        match offset {
                            0x0000 ... 0x7FFF => {
                                unreachable!("Invalid address {:x}", addr)
                            },
                            0x8000 ... 0xFFFF => {
                                panic!("Cannot write to ROM {:x}", addr)
                            },
                            _ => unreachable!("Invalid address {:x}", addr)
                        }
                    },
                    0x70 ... 0x7D | 0xF0 ... 0xFD => {
                        match offset {
                            0x0000 ... 0xFFFF => {
                                panic!("SRAM {:x}", addr)
                            },
                            _ => unreachable!("Invalid address {:x}", addr)
                        }
                    },
                    0x7E => {
                        match offset {
                            0x0000 ... 0x1FFF => {
                                panic!("Unimplemented: LowRAM (WRAM) {:x}", addr)
                            },
                            0x2000 ... 0x7FFF => {
                                panic!("Unimplemented: HighRAM (WRAM) {:x}", addr)
                            },
                            0x8000 ... 0xFFFF => {
                                panic!("Unimplemented: Extended RAM (WRAM) {:x}", addr)
                            },
                            _ => unreachable!("Invalid address {:x}", addr)
                        }
                    },
                    0x7F => {
                        match offset {
                            0x0000 ... 0xFFFF => {
                                panic!("Unimplemented: Extended RAM (WRAM) {:x}", addr)
                            },
                            _ => unreachable!("Invalid address {:x}", addr)
                        }
                    },
                    0xFE ... 0xFF => {
                        match offset {
                            0x0000 ... 0x7FFF => {
                                panic!("SRAM {:x}", addr)
                            },
                            0x8000 ... 0xFFFF => {
                                panic!("Cannot write to ROM {:x}", addr)
                            },
                            _ => unreachable!("Invalid address {:x}", addr)
                        }
                    },
                    _ => unreachable!("Invalid address {:x}", addr)
                }
            },
            HiROM | FastHiROM => {
                match bank {
                    0x00 ... 0x1F | 0x80 ... 0x9F => {
                        match offset {
                            0x0000 ... 0x1FFF => {
                                self.ram[offset] = data;
                            },
                            0x2000 ... 0x20FF => {
                                unreachable!("Invalid address {:x}", addr)
                            },
                            0x2100 ... 0x2143 => {
                                let adjusted_offset = offset - 0x2100;

                                self.bregs[adjusted_offset] = data;
                            },
                            0x2180 ... 0x2183 => {
                                let adjusted_offset = offset - 0x2180;

                                self.wramregs[adjusted_offset] = data;
                            },
                            0x2200 ... 0x2FFF => {
                                unreachable!("Invalid address {:x}", addr)
                            },
                            0x3000 ... 0x3FFF => {
                                panic!("Unimplemented: DSP, SuperFX, hardware registers {:x}", addr)
                            },
                            0x4000 ... 0x40FF => {
                                panic!("Unimplemented: Old style joypad registers {:x}", addr)
                            },
                            0x4100 ... 0x41FF => {
                                unreachable!("Invalid address {:x}", addr)
                            },
                            0x4200 ... 0x420D | 0x4210 ... 0x421F => {
                                let adjusted_offset = offset - 0x4200;

                                self.cpuregs[adjusted_offset] = data;
                            },
                            0x4300 ... 0x430A |
                            0x4310 ... 0x431A |
                            0x4320 ... 0x432A |
                            0x4330 ... 0x433A |
                            0x4340 ... 0x434A |
                            0x4350 ... 0x435A |
                            0x4360 ... 0x436A |
                            0x4370 ... 0x437A => {
                                let adjusted_offset = offset - 0x4300;

                                self.dmaregs[adjusted_offset] = data;
                            },
                            0x4500 ... 0x5FFF => {
                                unreachable!("Invalid address {:x}", addr)
                            },
                            0x6000 ... 0x7FFF => {
                                panic!("Reserved {:x}", addr)
                            },
                            0x8000 ... 0xFFFF => {
                                panic!("Cannot write to ROM {:x}", addr)
                            },
                            _ => unreachable!("Invalid address {:x}", addr)
                        }
                    },
                    0x20 ... 0x3F | 0xA0 ... 0xBF => {
                        match offset {
                            0x0000 ... 0x1FFF => {
                                panic!("Unimplemented: LowRAM, shadowed from bank 0x7E {:x}", addr)
                            },
                            0x2000 ... 0x20FF => {
                                unreachable!("Invalid address {:x}", addr)
                            },
                            0x2100 ... 0x21FF => {
                                panic!("Unimplemented: PPU1, APU, hardware registers {:x}", addr)
                            },
                            0x2200 ... 0x2FFF => {
                                unreachable!("Invalid address {:x}", addr)
                            },
                            0x3000 ... 0x3FFF => {
                                panic!("Unimplemented: DSP, SuperFX, hardware registers {:x}", addr)
                            },
                            0x4000 ... 0x40FF => {
                                panic!("Unimplemented: Old style joypad registers {:x}", addr)
                            },
                            0x4100 ... 0x41FF => {
                                unreachable!("Invalid address {:x}", addr)
                            },
                            0x4200 ... 0x44FF => {
                                panic!("Unimplemented: DMA, PPU2, hardware registers {:x}", addr)
                            },
                            0x4500 ... 0x5FFF => {
                                unreachable!("Invalid address {:x}", addr)
                            },
                            0x6000 ... 0x7FFF => {
                                panic!("SRAM {:x}", addr)
                            },
                            0x8000 ... 0xFFFF => {
                                panic!("Cannot write to ROM {:x}", addr)
                            },
                            _ => unreachable!("Invalid address {:x}", addr)
                        }
                    },
                    0x40 ... 0x7D | 0xC0 ... 0xFD => {
                        match offset {
                            0x0000 ... 0xFFFF => {
                                panic!("Cannot write to ROM {:x}", addr)
                            },
                            _ => unreachable!("Invalid address {:x}", addr)
                        }
                    },
                    0x7E => {
                        match offset {
                            0x0000 ... 0x1FFF => {
                                panic!("Unimplemented: LowRAM (WRAM) {:x}", addr)
                            },
                            0x2000 ... 0x7FFF => {
                                panic!("Unimplemented: HighRAM (WRAM) {:x}", addr)
                            },
                            0x8000 ... 0xFFFF => {
                                panic!("Unimplemented: Expanded RAM (WRAM) {:x}", addr)
                            },
                            _ => unreachable!("Invalid address {:x}", addr)
                        }
                    },
                    0x7F => {
                        match offset {
                            0x0000 ... 0xFFFF => {
                                panic!("Unimplemented: Expanded RAM (WRAM) {:x}", addr)
                            },
                            _ => unreachable!("Invalid address {:x}", addr)
                        }
                    },
                    0xFE ... 0xFF => {
                        match offset {
                            0x0000 ... 0xFFFF => {
                                panic!("Cannot write to ROM {:x}", addr)
                            },
                            _ => unreachable!("Invalid address {:x}", addr)
                        }
                    },
                    _ => unreachable!("Invalid address {:x}", addr)
                }
            },
            ExLoROM => {
                unimplemented!()
            },
            ExHiROM => {
                unimplemented!()
            },
        }
    }

    pub fn get_interrupt_vector(&self, emu: bool) -> usize {
        use self::RomType::*;

        match self.rom.rom_type {
            LoROM | FastLoROM => {
                if emu {
                    LOROM_EMU_MODE_VECTORS[Vectors::IRQ as usize]
                } else {
                    LOROM_NATIVE_MODE_VECTORS[Vectors::IRQ as usize]
                }
            },
            HiROM | FastHiROM => {
                if emu {
                    HIROM_EMU_MODE_VECTORS[Vectors::IRQ as usize]
                } else {
                    HIROM_NATIVE_MODE_VECTORS[Vectors::IRQ as usize]
                }
            },
            _ => panic!("ExLo/HiROM formats not supported")
        }
    }
}

pub struct Rom {
    data: Box<[u8]>,
    pub rom_type: RomType,
    rom_name: String,
    pub headered: bool,
    pub reset_vector: usize,
}

impl Rom {
    fn new(rom: Vec<u8>) -> Rom {
        use self::RomType::*;

        let headered = rom.len() % 1024 == HEADERED_OFFSET;
        let offset = if headered { 0x200 } else { 0 };
        let (rom_type, rom_name) = Rom::get_type_name(&rom, headered);

        let reset_vector = {
            let vector_loc = match rom_type {
                LoROM | FastLoROM => {
                    LOROM_EMU_MODE_VECTORS[Vectors::RESET as usize]
                },
                HiROM | FastHiROM => {
                    HIROM_EMU_MODE_VECTORS[Vectors::RESET as usize]
                },
                _ => panic!("ExLo/HiROM formats not supported")
            };

            rom[vector_loc + offset] as usize | ((rom[vector_loc + offset + 1] as usize) << 8)
        };

        Rom {
            data: rom.into_boxed_slice(),
            rom_type: rom_type,
            rom_name: rom_name,
            headered: headered,
            reset_vector: reset_vector,
        }
    }

    fn get_type_name(data: &Vec<u8>, headered: bool) -> (RomType, String) {
        use std::ascii::AsciiExt;
        use self::RomType::*;

        let mut name = String::new();
        let offset = if headered { HEADERED_OFFSET } else { 0 };

        if data[LOROM_ROM_NAME_START + offset].is_ascii() {
            for character in data.into_iter().skip(LOROM_ROM_NAME_START + offset).take(21) {
                name.push(*character as char);
            }

            let rom_type = {
                match data[LOROM_ROM_MAKEUP_BYTE] {
                    LOROM_FASTROM_VAL => FastLoROM,
                    EXLOROM_VAL => ExLoROM,
                    _ => LoROM,
                }
            };

            (rom_type, name)
        } else if data[HIROM_ROM_NAME_START + offset].is_ascii() {
            for character in data.into_iter().skip(HIROM_ROM_NAME_START + offset).take(21) {
                name.push(*character as char);
            }

            let rom_type = {
                match data[HIROM_ROM_MAKEUP_BYTE] {
                    HIROM_FASTROM_VAL => FastHiROM,
                    EXHIROM_VAL => ExHiROM,
                    _ => HiROM,
                }
            };

            (rom_type, name)
        } else {
            unreachable!("Malformed header")
        }
    }
}

impl fmt::Debug for Rom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rom {{
                    rom_type: {:?},
                    rom_name: {},
                    headered: {},
                    reset_vector: {:x}
                  }}",
               self.rom_type, self.rom_name, self.headered, self.reset_vector)
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Memory {{ rom: {:?} }}", self.rom)
    }
}

#[derive(Debug)]
pub enum RomType {
    LoROM,
    HiROM,
    FastLoROM,
    FastHiROM,
    ExLoROM,
    ExHiROM,
}

