use cpu::CPU;

pub trait Instruction {
    fn load(&self, cpu: &mut CPU) -> usize;
    fn store(&self, cpu: &mut CPU, data: usize);

    fn is_long(&self) -> bool {
        false
    }
}

pub enum InstructionType {
    LocatingData,
    ControlTransfer,
}

pub struct Absolute { pub instruction_type: InstructionType }
impl Instruction for Absolute {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("Absolute load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("Absolute store not implemented")
    }
}

pub struct AbsoluteIndexedX;
impl Instruction for AbsoluteIndexedX {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("AbsoluteIndexedX load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("AbsoluteIndexedX store not implemented")
    }
}

pub struct AbsoluteIndexedY;
impl Instruction for AbsoluteIndexedY {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("AbsoluteIndexedY load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("AbsoluteIndexedY store not implemented")
    }
}

pub struct AbsoluteIndexedIndirect;
impl Instruction for AbsoluteIndexedIndirect {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("AbsoluteIndexedIndirect load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("AbsoluteIndexedIndirect store not implemented")
    }
}

pub struct AbsoluteIndirect;
impl Instruction for AbsoluteIndirect {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("AbsoluteIndirect load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("AbsoluteIndirect store not implemented")
    }
}

pub struct AbsoluteIndirectLong;
impl Instruction for AbsoluteIndirectLong {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("AbsoluteIndirectLong load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("AbsoluteIndirectLong store not implemented")
    }
    
    fn is_long(&self) -> bool {
        true
    }
}

pub struct AbsoluteLong;
impl Instruction for AbsoluteLong {
    fn load(&self, cpu: &mut CPU) -> usize {
        let pc = cpu.program_counter;

        let low = cpu.memory.get_byte(pc + 1);
        let high = cpu.memory.get_byte(pc + 2);
        let bank = cpu.memory.get_byte(pc + 3);

        ((bank as usize) << 16) | ((high as usize) << 8) | (low as usize)
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("AbsoluteLong store not implemented")
    }

    fn is_long(&self) -> bool {
        true
    }
}

pub struct AbsoluteLongIndexedX;
impl Instruction for AbsoluteLongIndexedX {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("AbsoluteLongIndexedX load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("AbsoluteLongIndexedX store not implemented")
    }

    fn is_long(&self) -> bool {
        true
    }
}

pub struct Accumulator;
impl Instruction for Accumulator {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("Accumulator load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("Accumulator store not implemented")
    }
}

pub struct BlockMove;
impl Instruction for BlockMove {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("BlockMove load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("BlockMove store not implemented")
    }
}

pub struct DirectPage;
impl Instruction for DirectPage {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("DirectPage load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("DirectPage store not implemented")
    }
}

pub struct DirectPageIndexedX;
impl Instruction for DirectPageIndexedX {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("DirectPageIndexedX load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("DirectPageIndexedX store not implemented")
    }
}

pub struct DirectPageIndexedY;
impl Instruction for DirectPageIndexedY {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("DirectPageIndexedY load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("DirectPageIndexedY store not implemented")
    }
}

pub struct DirectPageIndexedIndirectX;
impl Instruction for DirectPageIndexedIndirectX {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("DirectPageIndexedIndirectX load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("DirectPageIndexedIndirectX store not implemented")
    }
}

pub struct DirectPageIndirect;
impl Instruction for DirectPageIndirect {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("DirectPageIndirect load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("DirectPageIndirect store not implemented")
    }
}

pub struct DirectPageIndirectLong;
impl Instruction for DirectPageIndirectLong {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("DirectPageIndirectLong load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("DirectPageIndirectLong store not implemented")
    }

    fn is_long(&self) -> bool {
        true
    }
}

pub struct DirectPageIndirectIndexedY;
impl Instruction for DirectPageIndirectIndexedY {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("DirectPageIndirectIndexedY load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("DirectPageIndirectIndexedY store not implemented")
    }
}

pub struct DirectPageIndirectLongIndexedY;
impl Instruction for DirectPageIndirectLongIndexedY {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("DirectPageIndirectLongIndexedY load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("DirectPageIndirectLongIndexedY store not implemented")
    }

    fn is_long(&self) -> bool {
        true
    }
}

pub struct Immediate;
impl Instruction for Immediate {
    fn load(&self, cpu: &mut CPU) -> usize {
        cpu.program_counter += 1;
        cpu.memory.get_byte(cpu.program_counter) as usize
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("Immediate store not implemented")
    }
}

pub struct Dummy;
impl Instruction for Dummy {
    fn load(&self, cpu: &mut CPU) -> usize {
        unreachable!()
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        unreachable!()
    }
}

pub struct ProgramCounterRelative;
impl Instruction for ProgramCounterRelative {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("ProgramCounterRelative load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("ProgramCounterRelative store not implemented")
    }
}

pub struct ProgramCounterRelativeLong;
impl Instruction for ProgramCounterRelativeLong {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("ProgramCounterRelativeLong load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("ProgramCounterRelativeLong store not implemented")
    }

    fn is_long(&self) -> bool {
        true
    }
}

pub struct StackAbsolute;
impl Instruction for StackAbsolute {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("StackAbsolute load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("StackAbsolute store not implemented")
    }
}

pub struct StackDirectPageIndirect;
impl Instruction for StackDirectPageIndirect {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("StackDirectPageIndirect load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("StackDirectPageIndirect store not implemented")
    }
}

pub struct StackInterrupt;
impl Instruction for StackInterrupt {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("StackInterrupt load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("StackInterrupt store not implemented")
    }
}

pub struct StackProgramCounterRelative;
impl Instruction for StackProgramCounterRelative {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("StackProgramCounterRelative load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("StackProgramCounterRelative store not implemented")
    }
}

pub struct StackPull;
impl Instruction for StackPull {
    fn load(&self, cpu: &mut CPU) -> usize {
        cpu.stack_pointer += 1;
        let low = cpu.memory.get_byte(cpu.stack_pointer) as usize;
        cpu.stack_pointer += 1;
        let high = cpu.memory.get_byte(cpu.stack_pointer) as usize; 

        (high << 8) | low
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        unreachable!("StackPull doesn't have a store")
    }
}

pub struct StackPush;
impl Instruction for StackPush {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("StackPush load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        use super::cpu::StatusFlags::IndexRegisterSize;

        if cpu.processor_status.status[IndexRegisterSize as usize] {
            cpu.memory.set_byte(cpu.stack_pointer, data as u8);
            cpu.stack_pointer -= 1;
        } else {
            let high = (data & 0xFF00) >> 8;
            let low = data & 0x00FF;

            cpu.memory.set_byte(cpu.stack_pointer, high as u8);
            cpu.stack_pointer -= 1;
            cpu.memory.set_byte(cpu.stack_pointer, low as u8);
            cpu.stack_pointer -= 1;
        }
    }
}

pub struct StackRTI;
impl Instruction for StackRTI {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("StackRTI load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("StackRTI store not implemented")
    }
}

pub struct StackRTL;
impl Instruction for StackRTL {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("StackRTL load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("StackRTL store not implemented")
    }
}

pub struct StackRTS;
impl Instruction for StackRTS {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("StackRTS load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("StackRTS store not implemented")
    }
}

pub struct StackRelative;
impl Instruction for StackRelative {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("StackRelative load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("StackRelative store not implemented")
    }
}

pub struct StackRelativeIndirectIndexedY;
impl Instruction for StackRelativeIndirectIndexedY {
    fn load(&self, cpu: &mut CPU) -> usize {
        panic!("StackRelativeIndirectIndexedY load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: usize) {
        panic!("StackRelativeIndirectIndexedY store not implemented")
    }
}

