use crate::cpu::instr::addressing::AddressingMode;
use crate::cpu::Cpu;
use crate::mem::Address;

pub trait Instruction {
    fn run(self);
}

pub struct CpuInstruction<'op, 'cpu> {
    pub opcode: u8,
    pub addr_mode: AddressingMode,
    cpu: &'cpu mut Cpu,
    do_run: &'op Fn(&mut Cpu, AddressingMode),
}

impl<'op, 'cpu> CpuInstruction<'op, 'cpu> {
    pub fn new(
        opcode: u8,
        cpu: &'cpu mut Cpu,
        addr_mode: AddressingMode,
        do_run: &'op Fn(&mut Cpu, AddressingMode),
    ) -> Self {
        CpuInstruction {
            opcode: opcode,
            addr_mode: addr_mode,
            cpu: cpu,
            do_run: do_run,
        }
    }

    pub fn from(opcode: u8, cpu: &'cpu mut Cpu) -> Self {
        match opcode {
            0xa9 => CpuInstruction::new(opcode, cpu, AddressingMode::Immediate, &lda),
            _ => panic!("opcode {:x} is not implemented!", opcode),
        }
    }
}

impl<'op, 'cpu> Instruction for CpuInstruction<'op, 'cpu> {
    fn run(self) {
        (self.do_run)(self.cpu, self.addr_mode)
    }
}

fn get_operand(cpu: &mut Cpu, addr_mode: &AddressingMode) -> GetOperandResult {
    match addr_mode {
        AddressingMode::Implied => panic!("Implicit instructions do not have a resolved address"),
        AddressingMode::Acc => panic!("Accumulator instructions always operate on the accumulator"),
        AddressingMode::Immediate => cpu.next_u8().into(),
        AddressingMode::ZeroPage => cpu.next_u8().into(),
        AddressingMode::IndexedZeroPageX => {
            let base_addr: Address = cpu.next_u8().into();

            (&base_addr + cpu.registers.x).into()
        }
        AddressingMode::IndexedZeroPageY => {
            let base_addr: Address = cpu.next_u8().into();

            (&base_addr + cpu.registers.y).into()
        }
        AddressingMode::Relative => (cpu.next_u8() as u16).into(),
        AddressingMode::Absolute => cpu.next_u16().into(),
        AddressingMode::IndexedAbsoluteX => {
            let base_addr: Address = cpu.next_u16().into();

            (&base_addr + cpu.registers.x).into()
        }
        AddressingMode::IndexedAbsoluteY => {
            let base_addr: Address = cpu.next_u16().into();

            (&base_addr + cpu.registers.y).into()
        }
        AddressingMode::Indirect => {
            let addr = cpu.next_u16();

            cpu.read_u16_at(&addr.into()).into()
        }
        AddressingMode::IndexedIndirect => {
            let addr: Address = cpu.next_u16().into();

            cpu.read_u16_at(&(&addr + cpu.registers.x)).into()
        }
        _ => panic!("addressing mode {:?} not implemented!", addr_mode),
    }
}

#[derive(Debug)]
pub enum GetOperandResult {
    Value(u8),
    Address(Address),
}

impl From<u8> for GetOperandResult {
    fn from(value: u8) -> Self {
        GetOperandResult::Value(value)
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
    pub fn resolve_value(self, cpu: &mut Cpu) -> u8 {
        match self {
            GetOperandResult::Value(val) => val,
            GetOperandResult::Address(addr) => cpu.read_u8_at(&addr) as u8,
        }
    }
}

fn lda(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let operand_value = operand.resolve_value(cpu);

    cpu.perform_instr(|cpu: &mut Cpu|{
        cpu.registers.acc += operand_value as i8;
    });
}
