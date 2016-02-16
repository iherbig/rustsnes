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
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize;
    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize);
}

pub enum InstructionType {
    LocatingData,
    ControlTransfer,
}

pub struct Absolute { pub instruction_type: InstructionType }
impl Instruction for Absolute {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        panic!("Absolute load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        let mut addr = (cpu.program_bank << 16) | cpu.program_counter;
        let bank = match self.instruction_type {
            InstructionType::LocatingData => cpu.data_bank,
            InstructionType::ControlTransfer => cpu.program_bank,
        };

        addr = (bank << 16) | (load_two_bytes(memory, addr) as usize);

        if is_byte {
            store_byte(memory, addr, data as u8);
            cpu.program_counter += 1;
        } else {
            store_two_bytes(memory, addr, data as u16);
            cpu.program_counter += 2;
        }
    }
}

pub struct AbsoluteIndexedX;
impl Instruction for AbsoluteIndexedX {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        panic!("AbsoluteIndexedX load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("AbsoluteIndexedX store not implemented")
    }
}

pub struct AbsoluteIndexedY;
impl Instruction for AbsoluteIndexedY {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        panic!("AbsoluteIndexedY load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("AbsoluteIndexedY store not implemented")
    }
}

pub struct AbsoluteIndexedIndirect;
impl Instruction for AbsoluteIndexedIndirect {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        panic!("AbsoluteIndexedIndirect load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("AbsoluteIndexedIndirect store not implemented")
    }
}

pub struct AbsoluteIndirect;
impl Instruction for AbsoluteIndirect {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        panic!("AbsoluteIndirect load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("AbsoluteIndirect store not implemented")
    }
}

pub struct AbsoluteIndirectLong;
impl Instruction for AbsoluteIndirectLong {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        panic!("AbsoluteIndirectLong load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("AbsoluteIndirectLong store not implemented")
    }
}

pub struct AbsoluteLong;
impl Instruction for AbsoluteLong {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        let addr = (cpu.program_bank << 16) + cpu.program_counter;
        cpu.program_counter += 3;

        load_three_bytes(memory, addr) as usize
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("AbsoluteLong store not implemented")
    }
}

pub struct AbsoluteLongIndexedX;
impl Instruction for AbsoluteLongIndexedX {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        panic!("AbsoluteLongIndexedX load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("AbsoluteLongIndexedX store not implemented")
    }
}

pub struct Accumulator;
impl Instruction for Accumulator {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        panic!("Accumulator load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("Accumulator store not implemented")
    }
}

pub struct BlockMove;
impl Instruction for BlockMove {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        panic!("BlockMove load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("BlockMove store not implemented")
    }
}

pub struct DirectPage;
impl Instruction for DirectPage {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        panic!("DirectPage load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("DirectPage store not implemented")
    }
}

pub struct DirectPageIndexedX;
impl Instruction for DirectPageIndexedX {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        panic!("DirectPageIndexedX load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("DirectPageIndexedX store not implemented")
    }
}

pub struct DirectPageIndexedY;
impl Instruction for DirectPageIndexedY {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        panic!("DirectPageIndexedY load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("DirectPageIndexedY store not implemented")
    }
}

pub struct DirectPageIndexedIndirectX;
impl Instruction for DirectPageIndexedIndirectX {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        panic!("DirectPageIndexedIndirectX load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("DirectPageIndexedIndirectX store not implemented")
    }
}

pub struct DirectPageIndirect;
impl Instruction for DirectPageIndirect {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        panic!("DirectPageIndirect load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("DirectPageIndirect store not implemented")
    }
}

pub struct DirectPageIndirectLong;
impl Instruction for DirectPageIndirectLong {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        panic!("DirectPageIndirectLong load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("DirectPageIndirectLong store not implemented")
    }
}

pub struct DirectPageIndirectIndexedY;
impl Instruction for DirectPageIndirectIndexedY {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        panic!("DirectPageIndirectIndexedY load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("DirectPageIndirectIndexedY store not implemented")
    }
}

pub struct DirectPageIndirectLongIndexedY;
impl Instruction for DirectPageIndirectLongIndexedY {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        panic!("DirectPageIndirectLongIndexedY load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("DirectPageIndirectLongIndexedY store not implemented")
    }
}

pub struct Immediate;
impl Instruction for Immediate {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        let addr = (cpu.program_bank << 16) + cpu.program_counter;

        if is_byte {
            cpu.program_counter += 1;
            load_byte(memory, addr) as usize
        } else {
            cpu.program_counter += 2;
            load_two_bytes(memory, addr) as usize
        }
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("Immediate store not implemented")
    }
}

pub struct ProgramCounterRelative;
impl Instruction for ProgramCounterRelative {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        panic!("ProgramCounterRelative load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("ProgramCounterRelative store not implemented")
    }
}

pub struct ProgramCounterRelativeLong;
impl Instruction for ProgramCounterRelativeLong {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        panic!("ProgramCounterRelativeLong load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("ProgramCounterRelativeLong store not implemented")
    }
}

pub struct StackAbsolute;
impl Instruction for StackAbsolute {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        panic!("StackAbsolute load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("StackAbsolute store not implemented")
    }
}

pub struct StackDirectPageIndirect;
impl Instruction for StackDirectPageIndirect {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        panic!("StackDirectPageIndirect load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("StackDirectPageIndirect store not implemented")
    }
}

pub struct StackInterrupt;
impl Instruction for StackInterrupt {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        panic!("StackInterrupt load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("StackInterrupt store not implemented")
    }
}

pub struct StackProgramCounterRelative;
impl Instruction for StackProgramCounterRelative {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        panic!("StackProgramCounterRelative load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("StackProgramCounterRelative store not implemented")
    }
}

pub struct StackPull;
impl Instruction for StackPull {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        let addr = (cpu.program_bank << 16) + cpu.stack_pointer;

        if is_byte {
            cpu.stack_pointer += 1;
            load_byte(memory, addr) as usize
        } else {
            cpu.stack_pointer += 2;
            load_two_bytes(memory, addr) as usize
        }
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        unreachable!("StackPull doesn't have a store")
    }
}

pub struct StackPush;
impl Instruction for StackPush {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        unreachable!("StackPush doesn't have a load")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        let addr = (cpu.program_bank << 16) + cpu.stack_pointer;

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
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        panic!("StackRTI load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("StackRTI store not implemented")
    }
}

pub struct StackRTL;
impl Instruction for StackRTL {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        panic!("StackRTL load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("StackRTL store not implemented")
    }
}

pub struct StackRTS;
impl Instruction for StackRTS {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        panic!("StackRTS load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("StackRTS store not implemented")
    }
}

pub struct StackRelative;
impl Instruction for StackRelative {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        panic!("StackRelative load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("StackRelative store not implemented")
    }
}

pub struct StackRelativeIndirectIndexedY;
impl Instruction for StackRelativeIndirectIndexedY {
    fn load(&self, cpu: &mut CPU, memory: &Memory, is_byte: bool) -> usize {
        panic!("StackRelativeIndirectIndexedY load not implemented")
    }

    fn store(&self, cpu: &mut CPU, memory: &mut Memory, is_byte: bool, data: usize) {
        panic!("StackRelativeIndirectIndexedY store not implemented")
    }
}

