use crate::cpu::registers::ProcStatusFlags;
use crate::cpu::instr::addressing::AddressingMode;
use crate::cpu::Cpu;
use crate::mem::Address;

#[derive(Debug)]
pub enum GetOperandResult {
    Value(i8),
    Address(Address),
}

impl From<u8> for GetOperandResult {
    fn from(value: u8) -> Self {
        GetOperandResult::Value(value as i8)
    }
}

impl From<u16> for GetOperandResult {
    fn from(addr: u16) -> Self {
        GetOperandResult::Address(addr.into())
    }
}

impl From<Address> for GetOperandResult {
    fn from(addr: Address) -> Self {
        let raw_addr: u16 = addr.into();

        raw_addr.into()
    }
}

impl GetOperandResult {
    pub fn resolve_value(self, cpu: &mut Cpu) -> i8 {
        match self {
            GetOperandResult::Value(val) => val,
            GetOperandResult::Address(addr) => cpu.read_u8_at(&addr) as i8,
        }
    }

    pub fn resolve_addr(&self) -> Address {
        match self {
            GetOperandResult::Value(_) => panic!("Cannot resolve addr on a Value result!"),
            GetOperandResult::Address(ref addr) => addr.clone(),
        }
    }
}

pub fn get_operand(cpu: &mut Cpu, addr_mode: &AddressingMode) -> GetOperandResult {
    match addr_mode {
        AddressingMode::Implied => panic!("Implicit instructions do not have a resolved address"),
        AddressingMode::Acc => panic!("Accumulator instructions always operate on the accumulator"),
        AddressingMode::Immediate => cpu.next_u8().into(),
        AddressingMode::ZeroPage => cpu.next_u8().into(),
        AddressingMode::ZeroPageX => {
            let base_addr: Address = cpu.next_u8().into();

            (&base_addr + cpu.registers.x).into()
        }
        AddressingMode::ZeroPageY => {
            let base_addr: Address = cpu.next_u8().into();

            (&base_addr + cpu.registers.y).into()
        }
        AddressingMode::Relative => cpu.next_u8().into(),
        AddressingMode::Absolute => cpu.next_u16().into(),
        AddressingMode::AbsoluteX => {
            let base_addr: Address = cpu.next_u16().into();

            (&base_addr + cpu.registers.x).into()
        }
        AddressingMode::AbsoluteY => {
            let base_addr: Address = cpu.next_u16().into();

            (&base_addr + cpu.registers.y).into()
        }
        AddressingMode::Indirect => {
            let addr = cpu.next_u16();

            cpu.read_u16_at(&addr.into()).into()
        }
        AddressingMode::IndexedIndirect => {
            let operand = cpu.next_u8();

            // lda ($20, X)
            // X: $04
            // 0024: 74 20
            // 2074: 55 __

            // X + $20 = $24
            let indir_addr = (cpu.registers.x as u8 + operand).into();

            // A <- **($24)
            let addr = cpu.read_u16_at(&indir_addr).into();

            // A <- *($2074)
            // A <- $55
            cpu.read_u8_at(&addr).into()
        }
        AddressingMode::IndirectIndexed => {
            // lda ($86), Y
            // Y: $10
            // 0086: 28 40
            // 4038: 23 __

            // A <- *(Y + *($86))
            // A <- *($10 + $4028)
            let operand = cpu.next_u8();
            let operand_addr = cpu.read_u16_at(&operand.into());

            // A <- *($4038)
            let indir_addr = (cpu.registers.y as u16 + operand_addr).into();

            // A <- $23
            cpu.read_u8_at(&indir_addr).into()
        }
    }
}

pub fn flags_from_compare(cpu_status: ProcStatusFlags, left: i8, right: i8) -> ProcStatusFlags {
    ProcStatusFlags {
        carry: left >= right,
        zero: left == right,
        interrupt_disable: cpu_status.interrupt_disable,
        decimal_mode: cpu_status.decimal_mode,
        break_command: cpu_status.break_command,
        overflow: cpu_status.overflow,
        negative: (left - right) < 0
    }
}