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
}

impl Memory {
    pub fn new(rom: Vec<u8>) -> Memory {
        Memory {
            ram: vec![0; RAM_SIZE].into_boxed_slice(),
            rom: Rom::new(rom),
            sram: [0; SRAM_SIZE],
        }
    }

    pub fn get_byte(&self, addr: usize) -> u8 {
        use self::RomType::*;
        let header_offset = if self.rom.headered { HEADERED_OFFSET } else { 0 };
        let addr_offset = addr + header_offset;
        let bank = (addr_offset & 0xFF0000) >> 16;
        let offset = addr_offset & 0xFFFF;

        match self.rom.rom_type {
            LoROM | FastLoROM => {
                match bank {
                    0x00 ... 0x3F | 0x80 ... 0xBF => {
                        match offset {
                            0x0000 ... 0x1FFF => {
                                panic!("Unimplemented: LowRAM, shadowed from bank 0x7E {:x}", addr_offset)
                            },
                            0x2000 ... 0x20FF => {
                                unreachable!("Invalid address {:x}", addr_offset)
                            },
                            0x2100 ... 0x21FF => {
                                panic!("Unimplemented: PPU1, APU, hardware registers {:x}", addr_offset)
                            },
                            0x2200 ... 0x2FFF => {
                                unreachable!("Invalid address {:x}", addr_offset)
                            },
                            0x3000 ... 0x3FFF => {
                                panic!("Unimplemented: DSP, SuperFX, hardware registers {:x}", addr_offset)
                            },
                            0x4000 ... 0x40FF => {
                                panic!("Unimplemented: Old style joypad registers {:x}", addr_offset)
                            },
                            0x4100 ... 0x41FF => {
                                unreachable!("Invalid address {:x}", addr_offset)
                            },
                            0x4200 ... 0x44FF => {
                                panic!("Unimplemented: DMA, PPU2, hardware registers {:x}", addr_offset)
                            },
                            0x4500 ... 0x5FFF => {
                                unreachable!("Invalid address {:x}", addr_offset)
                            },
                            0x6000 ... 0xFFFF => {
                                self.rom.data[(0x8000 * if bank >= 0x80 { bank - 0x80 } else { bank }) + offset]
                            },
                            _ => unreachable!("Invalid address {:x}", addr_offset)
                        }
                    },
                    0x40 ... 0x6F | 0xC0 ... 0xEF => {
                        match offset {
                            0x0000 ... 0x7FFF => {
                                unreachable!("Invalid address {:x}", addr_offset)
                            },
                            0x8000 ... 0xFFFF => {
                                self.rom.data[(0x8000 * if bank >= 0xC0 { bank - 0x80 } else { bank }) + offset]
                            },
                            _ => unreachable!("Invalid address {:x}", addr_offset)
                        }
                    },
                    0x70 ... 0x7D | 0xF0 ... 0xFD => {
                        match offset {
                            0x0000 ... 0xFFFF => {
                                self.rom.data[(0x8000 * if bank >= 0xF0 { bank - 0x80 } else { bank }) + offset]
                            },
                            _ => unreachable!("Invalid address {:x}", addr_offset)
                        }
                    },
                    0x7E => {
                        match offset {
                            0x0000 ... 0x1FFF => {
                                panic!("Unimplemented: LowRAM (WRAM) {:x}", addr_offset)
                            },
                            0x2000 ... 0x7FFF => {
                                panic!("Unimplemented: HighRAM (WRAM) {:x}", addr_offset)
                            },
                            0x8000 ... 0xFFFF => {
                                panic!("Unimplemented: Extended RAM (WRAM) {:x}", addr_offset)
                            },
                            _ => unreachable!("Invalid address {:x}", addr_offset)
                        }
                    },
                    0x7F => {
                        match offset {
                            0x0000 ... 0xFFFF => {
                                panic!("Unimplemented: Extended RAM (WRAM) {:x}", addr_offset)
                            },
                            _ => unreachable!("Invalid address {:x}", addr_offset)
                        }
                    },
                    0xFE ... 0xFF => {
                        match offset {
                            0x0000 ... 0xFFFF => {
                                self.rom.data[(bank << 16) + offset]
                            },
                            _ => unreachable!("Invalid address {:x}", addr_offset)
                        }
                    },
                    _ => unreachable!("Invalid address {:x}", addr_offset)
                }
            },
            HiROM | FastHiROM => {
                match bank {
                    0x00 ... 0x1F | 0x80 ... 0x9F => {
                        match offset {
                            0x0000 ... 0x1FFF => {
                                    self.rom.data[addr_offset]
                            },
                            0x2000 ... 0x20FF => {
                                unreachable!("Invalid address {:x}", addr_offset)
                            },
                            0x2100 ... 0x21FF => {
                                panic!("Unimplemented: PPU1, APU, hardware registers {:x}", addr_offset)
                            },
                            0x2200 ... 0x2FFF => {
                                unreachable!("Invalid address {:x}", addr_offset)
                            },
                            0x3000 ... 0x3FFF => {
                                panic!("Unimplemented: DSP, SuperFX, hardware registers {:x}", addr_offset)
                            },
                            0x4000 ... 0x40FF => {
                                panic!("Unimplemented: Old style joypad registers {:x}", addr_offset)
                            },
                            0x4100 ... 0x41FF => {
                                unreachable!("Invalid address {:x}", addr_offset)
                            },
                            0x4200 ... 0x44FF => {
                                panic!("Unimplemented: DMA, PPU2, hardware registers {:x}", addr_offset)
                            },
                            0x4500 ... 0x5FFF => {
                                unreachable!("Invalid address {:x}", addr_offset)
                            },
                            0x6000 ... 0xFFFF => {
                                self.rom.data[((if bank >= 0x80 { bank - 0x80 } else { bank }) << 16) + offset]
                            },
                            _ => unreachable!("Invalid address {:x}", addr_offset)
                        }
                    },
                    0x20 ... 0x3F | 0xA0 ... 0xBF => {
                        match offset {
                            0x0000 ... 0x1FFF => {
                                panic!("Unimplemented: LowRAM, shadowed from bank 0x7E {:x}", addr_offset)
                            },
                            0x2000 ... 0x20FF => {
                                unreachable!("Invalid address {:x}", addr_offset)
                            },
                            0x2100 ... 0x21FF => {
                                panic!("Unimplemented: PPU1, APU, hardware registers {:x}", addr_offset)
                            },
                            0x2200 ... 0x2FFF => {
                                unreachable!("Invalid address {:x}", addr_offset)
                            },
                            0x3000 ... 0x3FFF => {
                                panic!("Unimplemented: DSP, SuperFX, hardware registers {:x}", addr_offset)
                            },
                            0x4000 ... 0x40FF => {
                                panic!("Unimplemented: Old style joypad registers {:x}", addr_offset)
                            },
                            0x4100 ... 0x41FF => {
                                unreachable!("Invalid address {:x}", addr_offset)
                            },
                            0x4200 ... 0x44FF => {
                                panic!("Unimplemented: DMA, PPU2, hardware registers {:x}", addr_offset)
                            },
                            0x4500 ... 0x5FFF => {
                                unreachable!("Invalid address {:x}", addr_offset)
                            },
                            0x6000 ... 0x7FFF => {
                                self.sram[((if bank >= 0xA0 { bank - 0x80 - 0x20 } else { bank - 0x20 }) << 16) + offset]
                            },
                            0x8000 ... 0xFFFF => {
                                self.rom.data[((if bank >= 0xA0 { bank - 0x80 } else { bank }) << 16) + offset]
                            },
                            _ => unreachable!("Invalid address {:x}", addr_offset)
                        }
                    },
                    0x40 ... 0x7D | 0xC0 ... 0xFD => {
                        match offset {
                            0x0000 ... 0xFFFF => {
                                self.rom.data[(((if bank >= 0xC0 { bank - 0x80 - 0x40 } else { bank - 0x40})) * 0x8000) + offset]
                            },
                            _ => unreachable!("Invalid address {:x}", addr_offset)
                        }
                    },
                    0x7E => {
                        match offset {
                            0x0000 ... 0x1FFF => {
                                panic!("Unimplemented: LowRAM (WRAM) {:x}", addr_offset)
                            },
                            0x2000 ... 0x7FFF => {
                                panic!("Unimplemented: HighRAM (WRAM) {:x}", addr_offset)
                            },
                            0x8000 ... 0xFFFF => {
                                panic!("Unimplemented: Expanded RAM (WRAM) {:x}", addr_offset)
                            },
                            _ => unreachable!("Invalid address {:x}", addr_offset)
                        }
                    },
                    0x7F => {
                        match offset {
                            0x0000 ... 0xFFFF => {
                                panic!("Unimplemented: Expanded RAM (WRAM) {:x}", addr_offset)
                            },
                            _ => unreachable!("Invalid address {:x}", addr_offset)
                        }
                    },
                    0xFE ... 0xFF => {
                        match offset {
                            0x0000 ... 0xFFFF => {
                                self.rom.data[(bank << 16) + offset]
                            },
                            _ => unreachable!("Invalid address {:x}", addr_offset)
                        }
                    },
                    _ => unreachable!("Invalid address {:x}", addr_offset)
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
        let bank = (addr_offset & 0xFF0000) >> 16;
        let offset = addr_offset & 0xFFFF;

        match self.rom.rom_type {
            LoROM | FastLoROM => {
                match bank {
                    0x00 ... 0x3F | 0x80 ... 0xBF => {
                        match offset {
                            0x0000 ... 0x1FFF => {
                                panic!("Unimplemented: LowRAM, shadowed from bank 0x7E {:x}", addr_offset)
                            },
                            0x2000 ... 0x20FF => {
                                unreachable!("Invalid address {:x}", addr_offset)
                            },
                            0x2100 ... 0x21FF => {
                                panic!("Unimplemented: PPU1, APU, hardware registers {:x}", addr_offset)
                            },
                            0x2200 ... 0x2FFF => {
                                unreachable!("Invalid address {:x}", addr_offset)
                            },
                            0x3000 ... 0x3FFF => {
                                panic!("Unimplemented: DSP, SuperFX, hardware registers {:x}", addr_offset)
                            },
                            0x4000 ... 0x40FF => {
                                panic!("Unimplemented: Old style joypad registers {:x}", addr_offset)
                            },
                            0x4100 ... 0x41FF => {
                                unreachable!("Invalid address {:x}", addr_offset)
                            },
                            0x4200 ... 0x44FF => {
                                panic!("Unimplemented: DMA, PPU2, hardware registers {:x}", addr_offset)
                            },
                            0x4500 ... 0x5FFF => {
                                unreachable!("Invalid address {:x}", addr_offset)
                            },
                            0x6000 ... 0xFFFF => {
                                self.rom.data[(0x8000 * if bank >= 0x80 { bank - 0x80 } else { bank }) + offset] = data;
                            },
                            _ => unreachable!("Invalid address {:x}", addr_offset)
                        }
                    },
                    0x40 ... 0x6F | 0xC0 ... 0xEF => {
                        match offset {
                            0x0000 ... 0x7FFF => {
                                unreachable!("Invalid address {:x}", addr_offset)
                            },
                            0x8000 ... 0xFFFF => {
                                self.rom.data[(0x8000 * if bank >= 0xC0 { bank - 0x80 } else { bank }) + offset] = data;
                            },
                            _ => unreachable!("Invalid address {:x}", addr_offset)
                        }
                    },
                    0x70 ... 0x7D | 0xF0 ... 0xFD => {
                        match offset {
                            0x0000 ... 0xFFFF => {
                                self.rom.data[(0x8000 * if bank >= 0xF0 { bank - 0x80 } else { bank }) + offset] = data;
                            },
                            _ => unreachable!("Invalid address {:x}", addr_offset)
                        }
                    },
                    0x7E => {
                        match offset {
                            0x0000 ... 0x1FFF => {
                                panic!("Unimplemented: LowRAM (WRAM) {:x}", addr_offset)
                            },
                            0x2000 ... 0x7FFF => {
                                panic!("Unimplemented: HighRAM (WRAM) {:x}", addr_offset)
                            },
                            0x8000 ... 0xFFFF => {
                                panic!("Unimplemented: Extended RAM (WRAM) {:x}", addr_offset)
                            },
                            _ => unreachable!("Invalid address {:x}", addr_offset)
                        }
                    },
                    0x7F => {
                        match offset {
                            0x0000 ... 0xFFFF => {
                                panic!("Unimplemented: Extended RAM (WRAM) {:x}", addr_offset)
                            },
                            _ => unreachable!("Invalid address {:x}", addr_offset)
                        }
                    },
                    0xFE ... 0xFF => {
                        match offset {
                            0x0000 ... 0xFFFF => {
                                self.rom.data[(bank << 16) + offset] = data;
                            },
                            _ => unreachable!("Invalid address {:x}", addr_offset)
                        }
                    },
                    _ => unreachable!("Invalid address {:x}", addr_offset)
                }
            },
            HiROM | FastHiROM => {
                match bank {
                    0x00 ... 0x1F | 0x80 ... 0x9F => {
                        match offset {
                            0x0000 ... 0x1FFF => {
                                    self.rom.data[addr_offset] = data;
                            },
                            0x2000 ... 0x20FF => {
                                unreachable!("Invalid address {:x}", addr_offset)
                            },
                            0x2100 ... 0x21FF => {
                                panic!("Unimplemented: PPU1, APU, hardware registers {:x}", addr_offset)
                            },
                            0x2200 ... 0x2FFF => {
                                unreachable!("Invalid address {:x}", addr_offset)
                            },
                            0x3000 ... 0x3FFF => {
                                panic!("Unimplemented: DSP, SuperFX, hardware registers {:x}", addr_offset)
                            },
                            0x4000 ... 0x40FF => {
                                panic!("Unimplemented: Old style joypad registers {:x}", addr_offset)
                            },
                            0x4100 ... 0x41FF => {
                                unreachable!("Invalid address {:x}", addr_offset)
                            },
                            0x4200 ... 0x44FF => {
                                panic!("Unimplemented: DMA, PPU2, hardware registers {:x}", addr_offset)
                            },
                            0x4500 ... 0x5FFF => {
                                unreachable!("Invalid address {:x}", addr_offset)
                            },
                            0x6000 ... 0xFFFF => {
                                self.rom.data[((if bank >= 0x80 { bank - 0x80 } else { bank }) << 16) + offset] = data;
                            },
                            _ => unreachable!("Invalid address {:x}", addr_offset)
                        }
                    },
                    0x20 ... 0x3F | 0xA0 ... 0xBF => {
                        match offset {
                            0x0000 ... 0x1FFF => {
                                panic!("Unimplemented: LowRAM, shadowed from bank 0x7E {:x}", addr_offset)
                            },
                            0x2000 ... 0x20FF => {
                                unreachable!("Invalid address {:x}", addr_offset)
                            },
                            0x2100 ... 0x21FF => {
                                panic!("Unimplemented: PPU1, APU, hardware registers {:x}", addr_offset)
                            },
                            0x2200 ... 0x2FFF => {
                                unreachable!("Invalid address {:x}", addr_offset)
                            },
                            0x3000 ... 0x3FFF => {
                                panic!("Unimplemented: DSP, SuperFX, hardware registers {:x}", addr_offset)
                            },
                            0x4000 ... 0x40FF => {
                                panic!("Unimplemented: Old style joypad registers {:x}", addr_offset)
                            },
                            0x4100 ... 0x41FF => {
                                unreachable!("Invalid address {:x}", addr_offset)
                            },
                            0x4200 ... 0x44FF => {
                                panic!("Unimplemented: DMA, PPU2, hardware registers {:x}", addr_offset)
                            },
                            0x4500 ... 0x5FFF => {
                                unreachable!("Invalid address {:x}", addr_offset)
                            },
                            0x6000 ... 0x7FFF => {
                                self.sram[((if bank >= 0xA0 { bank - 0x80 - 0x20 } else { bank - 0x20 }) << 16) + offset] = data;
                            },
                            0x8000 ... 0xFFFF => {
                                self.rom.data[((if bank >= 0xA0 { bank - 0x80 } else { bank }) << 16) + offset] = data;
                            },
                            _ => unreachable!("Invalid address {:x}", addr_offset)
                        }
                    },
                    0x40 ... 0x7D | 0xC0 ... 0xFD => {
                        match offset {
                            0x0000 ... 0xFFFF => {
                                self.rom.data[(((if bank >= 0xC0 { bank - 0x80 - 0x40 } else { bank - 0x40})) * 0x8000) + offset] = data;
                            },
                            _ => unreachable!("Invalid address {:x}", addr_offset)
                        }
                    },
                    0x7E => {
                        match offset {
                            0x0000 ... 0x1FFF => {
                                panic!("Unimplemented: LowRAM (WRAM) {:x}", addr_offset)
                            },
                            0x2000 ... 0x7FFF => {
                                panic!("Unimplemented: HighRAM (WRAM) {:x}", addr_offset)
                            },
                            0x8000 ... 0xFFFF => {
                                panic!("Unimplemented: Expanded RAM (WRAM) {:x}", addr_offset)
                            },
                            _ => unreachable!("Invalid address {:x}", addr_offset)
                        }
                    },
                    0x7F => {
                        match offset {
                            0x0000 ... 0xFFFF => {
                                panic!("Unimplemented: Expanded RAM (WRAM) {:x}", addr_offset)
                            },
                            _ => unreachable!("Invalid address {:x}", addr_offset)
                        }
                    },
                    0xFE ... 0xFF => {
                        match offset {
                            0x0000 ... 0xFFFF => {
                                self.rom.data[(bank << 16) + offset] = data;
                            },
                            _ => unreachable!("Invalid address {:x}", addr_offset)
                        }
                    },
                    _ => unreachable!("Invalid address {:x}", addr_offset)
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

}

pub struct Rom {
    data: Box<[u8]>,
    pub rom_type: RomType,
    rom_name: String,
    pub headered: bool,
}

impl Rom {
    fn new(rom: Vec<u8>) -> Rom {
        let headered = rom.len() % 1024 == HEADERED_OFFSET;
        let (rom_type, rom_name) = Rom::get_type_name(&rom, headered);

        Rom {
            data: rom.into_boxed_slice(),
            rom_type: rom_type,
            rom_name: rom_name,
            headered: headered,
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

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Memory has too much to print")
    }
}

pub enum RomType {
    LoROM,
    HiROM,
    FastLoROM,
    FastHiROM,
    ExLoROM,
    ExHiROM,
}
