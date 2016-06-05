use cpu::CPU;
use memory::Memory;

fn load_byte(memory: &Memory, addr: usize) -> u8 {
    memory.get_byte(addr)
}

fn load_two_bytes(memory: &Memory, addr: usize) -> u16 {
    let low_addr = memory.get_byte(addr) as u16;
    let high_addr = memory.get_byte(addr + 1) as u16;
    (high_addr << 8) | low_addr
}

fn load_three_bytes(memory: &Memory, addr: usize) -> u32 {
    let low = memory.get_byte(addr) as usize;
    let high = memory.get_byte(addr + 1) as usize;
    let bank = memory.get_byte(addr + 2) as usize;

    ((bank << 16) | (high << 8) | low) as u32
}

fn store_byte(memory: &mut Memory, addr: usize, data: u8) {
    memory.set_byte(addr, data);
}

fn store_two_bytes(memory: &mut Memory, addr: usize, data: u16) {
    let high = (data & 0xFF00) >> 8;
    let low = data & 0x00FF;

    memory.set_byte(addr, low as u8);
    memory.set_byte(addr + 1, high as u8);
}

fn store_three_bytes(memory: &mut Memory, addr: usize, data: u32) {
    let bank = (data & 0xFF0000) >> 16;
    let high = (data & 0xFF00) >> 8;
    let low = data & 0xFF;

    memory.set_byte(addr, low as u8);
    memory.set_byte(addr + 1, high as u8);
    memory.set_byte(addr + 2, bank as u8);
}

pub trait Instruction {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32;
    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32);
}

pub enum InstructionType {
    LocatingData,
    ControlTransfer,
}

pub struct Absolute { pub instruction_type: InstructionType }
impl Instruction for Absolute {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        let addr = self.get_addr(cpu, memory);

        cpu.program_counter += 2;

        match self.instruction_type {
            InstructionType::ControlTransfer => addr as u32,
            InstructionType::LocatingData => {
                if is_byte {
                    load_byte(memory, addr) as u32
                } else {
                    load_two_bytes(memory, addr) as u32
                }
            },
        }
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        let addr = self.get_addr(cpu, memory);

        if is_byte {
            store_byte(memory, addr, data as u8);
        } else {
            store_two_bytes(memory, addr, data as u16);
        }

        cpu.program_counter += 2;
    }
}

impl Absolute {
    fn get_addr(&self, cpu: &CPU, memory: &Memory) -> usize {
        let addr = (cpu.program_bank << 16) | cpu.program_counter;
        let bank = match self.instruction_type {
            InstructionType::LocatingData => cpu.data_bank,
            InstructionType::ControlTransfer => cpu.program_bank,
        };

        (bank << 16) | (load_two_bytes(memory, addr) as usize)
    }
}

pub struct AbsoluteIndexedX;
impl Instruction for AbsoluteIndexedX {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        panic!("AbsoluteIndexedX load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        panic!("AbsoluteIndexedX store not implemented")
    }
}

pub struct AbsoluteIndexedY;
impl Instruction for AbsoluteIndexedY {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        panic!("AbsoluteIndexedY load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        panic!("AbsoluteIndexedY store not implemented")
    }
}

pub struct AbsoluteIndexedIndirect;
impl Instruction for AbsoluteIndexedIndirect {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        panic!("AbsoluteIndexedIndirect load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        panic!("AbsoluteIndexedIndirect store not implemented")
    }
}

pub struct AbsoluteIndirect;
impl Instruction for AbsoluteIndirect {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        panic!("AbsoluteIndirect load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        panic!("AbsoluteIndirect store not implemented")
    }
}

pub struct AbsoluteIndirectLong;
impl Instruction for AbsoluteIndirectLong {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        panic!("AbsoluteIndirectLong load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        panic!("AbsoluteIndirectLong store not implemented")
    }
}

pub struct AbsoluteLong { pub instruction_type: InstructionType }
impl Instruction for AbsoluteLong {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        let addr = self.get_addr(cpu, memory);

        cpu.program_counter += 3;

        match self.instruction_type {
            InstructionType::LocatingData => load_three_bytes(memory, addr),
            InstructionType::ControlTransfer => addr as u32,
        }
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        panic!("AbsoluteLong store not implemented")
    }
}

impl AbsoluteLong {
    fn get_addr(&self, cpu: &CPU, memory: &Memory) -> usize {
        let addr = (cpu.program_bank << 16) | cpu.program_counter;
        load_three_bytes(memory, addr) as usize
    }
}

pub struct AbsoluteLongIndexedX;
impl Instruction for AbsoluteLongIndexedX {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        let long = AbsoluteLong { instruction_type: InstructionType::LocatingData };

        let addr = long.load(cpu, memory, is_byte) as usize;

        if is_byte {
            (addr + ((cpu.index_x as u8) as usize)) as u32
        } else {
            (addr + (cpu.index_x as usize)) as u32
        }
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        panic!("AbsoluteLongIndexedX store not implemented")
    }
}

pub struct Accumulator;
impl Instruction for Accumulator {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        if is_byte {
            (cpu.accumulator as u8) as u32
        } else {
            cpu.accumulator as u32
        }
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        if is_byte {
            cpu.accumulator = (cpu.accumulator & 0xFF00) + (data as u16);
        } else {
            cpu.accumulator = data as u16;
        }
    }
}

pub struct BlockMove;
impl Instruction for BlockMove {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        panic!("BlockMove load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        panic!("BlockMove store not implemented")
    }
}

pub struct DirectPage;
impl Instruction for DirectPage {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        let addr = cpu.direct_page + (load_byte(memory, cpu.program_counter) as usize);
        cpu.program_counter += 1;

        if is_byte {
            load_byte(memory, addr) as u32
        } else {
            load_two_bytes(memory, addr) as u32
        }
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        let addr = cpu.direct_page + (load_byte(memory, cpu.program_counter) as usize);

        if is_byte {
            store_byte(memory, addr, data as u8);
        } else {
            store_two_bytes(memory, addr, data as u16);
        }

        cpu.program_counter += 1;
    }
}

pub struct DirectPageIndexedX;
impl Instruction for DirectPageIndexedX {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        panic!("DirectPageIndexedX load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        panic!("DirectPageIndexedX store not implemented")
    }
}

pub struct DirectPageIndexedY;
impl Instruction for DirectPageIndexedY {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        panic!("DirectPageIndexedY load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        panic!("DirectPageIndexedY store not implemented")
    }
}

pub struct DirectPageIndexedIndirectX;
impl Instruction for DirectPageIndexedIndirectX {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        panic!("DirectPageIndexedIndirectX load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        panic!("DirectPageIndexedIndirectX store not implemented")
    }
}

pub struct DirectPageIndirect;
impl Instruction for DirectPageIndirect {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        panic!("DirectPageIndirect load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        panic!("DirectPageIndirect store not implemented")
    }
}

pub struct DirectPageIndirectLong;
impl Instruction for DirectPageIndirectLong {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        panic!("DirectPageIndirectLong load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        panic!("DirectPageIndirectLong store not implemented")
    }
}

pub struct DirectPageIndirectIndexedY;
impl Instruction for DirectPageIndirectIndexedY {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        panic!("DirectPageIndirectIndexedY load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        panic!("DirectPageIndirectIndexedY store not implemented")
    }
}

pub struct DirectPageIndirectLongIndexedY;
impl Instruction for DirectPageIndirectLongIndexedY {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        let offset = load_byte(memory, (cpu.program_bank << 16) + cpu.program_counter);
        cpu.program_counter += 1;

        let addr = load_three_bytes(memory, cpu.direct_page + offset as usize) as usize;

        if is_byte {
            load_byte(memory, addr + ((cpu.index_y as u8) as usize)) as u32
        } else {
            load_byte(memory, addr + (cpu.index_y as usize)) as u32
        }
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        panic!("DirectPageIndirectLongIndexedY store not implemented")
    }
}

pub struct Immediate;
impl Instruction for Immediate {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        let addr = (cpu.program_bank << 16) + cpu.program_counter;

        if is_byte {
            cpu.program_counter += 1;
            load_byte(memory, addr) as u32
        } else {
            cpu.program_counter += 2;
            load_two_bytes(memory, addr) as u32
        }
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        panic!("Immediate store not implemented")
    }
}

pub struct ProgramCounterRelative;
impl Instruction for ProgramCounterRelative {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        load_byte(memory, (cpu.program_bank << 16) + cpu.program_counter) as u32
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        panic!("ProgramCounterRelative store not implemented")
    }
}

pub struct ProgramCounterRelativeLong;
impl Instruction for ProgramCounterRelativeLong {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        panic!("ProgramCounterRelativeLong load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        panic!("ProgramCounterRelativeLong store not implemented")
    }
}

pub struct StackAbsolute;
impl Instruction for StackAbsolute {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        panic!("StackAbsolute load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        panic!("StackAbsolute store not implemented")
    }
}

pub struct StackDirectPageIndirect;
impl Instruction for StackDirectPageIndirect {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        panic!("StackDirectPageIndirect load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        panic!("StackDirectPageIndirect store not implemented")
    }
}

pub struct StackProgramCounterRelative;
impl Instruction for StackProgramCounterRelative {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        panic!("StackProgramCounterRelative load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        panic!("StackProgramCounterRelative store not implemented")
    }
}

pub struct StackPull;
impl Instruction for StackPull {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        let addr = cpu.stack_pointer + 1;

        if is_byte {
            cpu.stack_pointer += 1;
            load_byte(memory, addr) as u32
        } else {
            cpu.stack_pointer += 2;
            load_two_bytes(memory, addr) as u32
        }
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        unreachable!("StackPull doesn't have a store")
    }
}

pub struct StackPush;
impl Instruction for StackPush {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        unreachable!("StackPush doesn't have a load")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        let addr = cpu.stack_pointer;

        if is_byte {
            store_byte(memory, addr, data as u8);
            cpu.stack_pointer -= 1;
        } else {
            store_two_bytes(memory, addr, data as u16);
            cpu.stack_pointer -= 2;
        }
    }
}

pub struct StackRTI;
impl Instruction for StackRTI {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        panic!("StackRTI load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        panic!("StackRTI store not implemented")
    }
}

pub struct StackRTL;
impl Instruction for StackRTL {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        panic!("StackRTL load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        panic!("StackRTL store not implemented")
    }
}

pub struct StackRTS;
impl Instruction for StackRTS {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        let pull = StackPull;

        let data = pull.load(cpu, memory, is_byte);
        let low = ((data & 0xFF00) >> 8) as u32;
        let high = (data & 0xFF) as u32;

        (high << 8) + low
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        unreachable!("StackRTS store doesn't exist")
    }
}

pub struct StackRelative;
impl Instruction for StackRelative {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        panic!("StackRelative load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        panic!("StackRelative store not implemented")
    }
}

pub struct StackRelativeIndirectIndexedY;
impl Instruction for StackRelativeIndirectIndexedY {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> u32 {
        panic!("StackRelativeIndirectIndexedY load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: u32) {
        panic!("StackRelativeIndirectIndexedY store not implemented")
    }
}

