use cpu::CPU;

pub trait Instruction {
    fn load(&self, cpu: &mut CPU) -> u16;
    fn store(&self, cpu: &mut CPU, data: u16);
}

pub enum InstructionType {
    LocatingData,
    ControlTransfer,
}

pub struct Absolute { pub instruction_type: InstructionType }
impl Instruction for Absolute {
    fn load(&self, cpu: &mut CPU) -> u16 {
        use self::InstructionType::*;

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

pub struct AbsoluteIndexedX;
impl Instruction for AbsoluteIndexedX {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct AbsoluteIndexedY;
impl Instruction for AbsoluteIndexedY {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct AbsoluteIndexedIndirect;
impl Instruction for AbsoluteIndexedIndirect {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct AbsoluteIndirect;
impl Instruction for AbsoluteIndirect {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct AbsoluteIndirectLong;
impl Instruction for AbsoluteIndirectLong {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct AbsoluteLong;
impl Instruction for AbsoluteLong {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct AbsoluteLongIndexedX;
impl Instruction for AbsoluteLongIndexedX {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct Accumulator;
impl Instruction for Accumulator {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct BlockMove;
impl Instruction for BlockMove {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct DirectPage;
impl Instruction for DirectPage {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct DirectPageIndexedX;
impl Instruction for DirectPageIndexedX {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct DirectPageIndexedY;
impl Instruction for DirectPageIndexedY {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct DirectPageIndexedIndirectX;
impl Instruction for DirectPageIndexedIndirectX {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct DirectPageIndirect;
impl Instruction for DirectPageIndirect {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct DirectPageIndirectLong;
impl Instruction for DirectPageIndirectLong {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct DirectPageIndirectIndexedY;
impl Instruction for DirectPageIndirectIndexedY {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct DirectPageIndirectLongIndexedY;
impl Instruction for DirectPageIndirectLongIndexedY {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct Immediate;
impl Instruction for Immediate {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct Implied;
impl Instruction for Implied {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct ProgramCounterRelative;
impl Instruction for ProgramCounterRelative {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct ProgramCounterRelativeLong;
impl Instruction for ProgramCounterRelativeLong {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct StackAbsolute;
impl Instruction for StackAbsolute {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct StackDirectPageIndirect;
impl Instruction for StackDirectPageIndirect {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct StackInterrupt;
impl Instruction for StackInterrupt {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct StackProgramCounterRelative;
impl Instruction for StackProgramCounterRelative {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct StackPull;
impl Instruction for StackPull {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct StackPush;
impl Instruction for StackPush {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct StackRTI;
impl Instruction for StackRTI {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct StackRTL;
impl Instruction for StackRTL {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct StackRTS;
impl Instruction for StackRTS {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct StackRelative;
impl Instruction for StackRelative {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

pub struct StackRelativeIndirectIndexedY;
impl Instruction for StackRelativeIndirectIndexedY {
    fn load(&self, cpu: &mut CPU) -> u16 {
        1_u16
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
    }
}

