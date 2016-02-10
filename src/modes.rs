use cpu::CPU;

pub trait Instruction {
    fn load(&self, cpu: &mut CPU) -> u16;
    fn store(&self, cpu: &mut CPU, data: u16);

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
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("Absolute load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("Absolute store not implemented")
    }
}

pub struct AbsoluteIndexedX;
impl Instruction for AbsoluteIndexedX {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("AbsoluteIndexedX load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("AbsoluteIndexedX store not implemented")
    }
}

pub struct AbsoluteIndexedY;
impl Instruction for AbsoluteIndexedY {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("AbsoluteIndexedY load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("AbsoluteIndexedY store not implemented")
    }
}

pub struct AbsoluteIndexedIndirect;
impl Instruction for AbsoluteIndexedIndirect {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("AbsoluteIndexedIndirect load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("AbsoluteIndexedIndirect store not implemented")
    }
}

pub struct AbsoluteIndirect;
impl Instruction for AbsoluteIndirect {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("AbsoluteIndirect load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("AbsoluteIndirect store not implemented")
    }
}

pub struct AbsoluteIndirectLong;
impl Instruction for AbsoluteIndirectLong {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("AbsoluteIndirectLong load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("AbsoluteIndirectLong store not implemented")
    }
    
    fn is_long(&self) -> bool {
        true
    }
}

pub struct AbsoluteLong;
impl Instruction for AbsoluteLong {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("AbsoluteLong load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("AbsoluteLong store not implemented")
    }

    fn is_long(&self) -> bool {
        true
    }
}

pub struct AbsoluteLongIndexedX;
impl Instruction for AbsoluteLongIndexedX {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("AbsoluteLongIndexedX load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("AbsoluteLongIndexedX store not implemented")
    }

    fn is_long(&self) -> bool {
        true
    }
}

pub struct Accumulator;
impl Instruction for Accumulator {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("Accumulator load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("Accumulator store not implemented")
    }
}

pub struct BlockMove;
impl Instruction for BlockMove {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("BlockMove load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("BlockMove store not implemented")
    }
}

pub struct DirectPage;
impl Instruction for DirectPage {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("DirectPage load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("DirectPage store not implemented")
    }
}

pub struct DirectPageIndexedX;
impl Instruction for DirectPageIndexedX {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("DirectPageIndexedX load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("DirectPageIndexedX store not implemented")
    }
}

pub struct DirectPageIndexedY;
impl Instruction for DirectPageIndexedY {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("DirectPageIndexedY load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("DirectPageIndexedY store not implemented")
    }
}

pub struct DirectPageIndexedIndirectX;
impl Instruction for DirectPageIndexedIndirectX {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("DirectPageIndexedIndirectX load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("DirectPageIndexedIndirectX store not implemented")
    }
}

pub struct DirectPageIndirect;
impl Instruction for DirectPageIndirect {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("DirectPageIndirect load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("DirectPageIndirect store not implemented")
    }
}

pub struct DirectPageIndirectLong;
impl Instruction for DirectPageIndirectLong {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("DirectPageIndirectLong load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("DirectPageIndirectLong store not implemented")
    }

    fn is_long(&self) -> bool {
        true
    }
}

pub struct DirectPageIndirectIndexedY;
impl Instruction for DirectPageIndirectIndexedY {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("DirectPageIndirectIndexedY load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("DirectPageIndirectIndexedY store not implemented")
    }
}

pub struct DirectPageIndirectLongIndexedY;
impl Instruction for DirectPageIndirectLongIndexedY {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("DirectPageIndirectLongIndexedY load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("DirectPageIndirectLongIndexedY store not implemented")
    }

    fn is_long(&self) -> bool {
        true
    }
}

pub struct Immediate;
impl Instruction for Immediate {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("Immediate load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("Immediate store not implemented")
    }
}

pub struct Dummy;
impl Instruction for Dummy {
    fn load(&self, cpu: &mut CPU) -> u16 {
        unreachable!()
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        unreachable!()
    }
}

pub struct ProgramCounterRelative;
impl Instruction for ProgramCounterRelative {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("ProgramCounterRelative load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("ProgramCounterRelative store not implemented")
    }
}

pub struct ProgramCounterRelativeLong;
impl Instruction for ProgramCounterRelativeLong {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("ProgramCounterRelativeLong load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("ProgramCounterRelativeLong store not implemented")
    }

    fn is_long(&self) -> bool {
        true
    }
}

pub struct StackAbsolute;
impl Instruction for StackAbsolute {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("StackAbsolute load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("StackAbsolute store not implemented")
    }
}

pub struct StackDirectPageIndirect;
impl Instruction for StackDirectPageIndirect {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("StackDirectPageIndirect load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("StackDirectPageIndirect store not implemented")
    }
}

pub struct StackInterrupt;
impl Instruction for StackInterrupt {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("StackInterrupt load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("StackInterrupt store not implemented")
    }
}

pub struct StackProgramCounterRelative;
impl Instruction for StackProgramCounterRelative {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("StackProgramCounterRelative load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("StackProgramCounterRelative store not implemented")
    }
}

pub struct StackPull;
impl Instruction for StackPull {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("StackPull load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("StackPull store not implemented")
    }
}

pub struct StackPush;
impl Instruction for StackPush {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("StackPush load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("StackPush store not implemented")
    }
}

pub struct StackRTI;
impl Instruction for StackRTI {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("StackRTI load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("StackRTI store not implemented")
    }
}

pub struct StackRTL;
impl Instruction for StackRTL {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("StackRTL load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("StackRTL store not implemented")
    }
}

pub struct StackRTS;
impl Instruction for StackRTS {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("StackRTS load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("StackRTS store not implemented")
    }
}

pub struct StackRelative;
impl Instruction for StackRelative {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("StackRelative load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("StackRelative store not implemented")
    }
}

pub struct StackRelativeIndirectIndexedY;
impl Instruction for StackRelativeIndirectIndexedY {
    fn load(&self, cpu: &mut CPU) -> u16 {
        panic!("StackRelativeIndirectIndexedY load not implemented")
    }

    fn store(&self, cpu: &mut CPU, data: u16) {
        panic!("StackRelativeIndirectIndexedY store not implemented")
    }
}

