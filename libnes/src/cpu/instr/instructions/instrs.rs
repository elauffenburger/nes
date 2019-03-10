use crate::cpu::registers::ProcStatusFlags;
use crate::bits::{lsb, msb};
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

    pub fn resolve_addr(self) -> Address {
        match self {
            GetOperandResult::Value(_) => panic!("Cannot resolve addr on a Value result!"),
            GetOperandResult::Address(addr) => addr,
        }
    }
}

fn get_operand(cpu: &mut Cpu, addr_mode: &AddressingMode) -> GetOperandResult {
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

pub fn lda(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    cpu.perform_instr(|cpu: &mut Cpu| {
        cpu.registers.acc = value;
    });
}

pub fn sta(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let operand_addr = operand.resolve_addr();

    cpu.perform_instr(|cpu: &mut Cpu| {
        cpu.write_bytes_to(&operand_addr, &[cpu.registers.acc as u8]);
    });
}

pub fn brk(cpu: &mut Cpu, _: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        cpu.push(msb(cpu.registers.pc));
        cpu.push(lsb(cpu.registers.pc));
        cpu.push(cpu.registers.p.clone().into());

        cpu.registers.p.interrupt_disable = true;
        cpu.registers.pc = cpu.read_u16_at(&(0xffe6.into()));
    });
}

pub fn stp(cpu: &mut Cpu, _: AddressingMode) {
    cpu.stop();
}

pub fn tax(cpu: &mut Cpu, _: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        cpu.registers.x = cpu.registers.acc;
    });
}

pub fn inx(cpu: &mut Cpu, _: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        cpu.registers.x += 1;
    });
}

pub fn dex(cpu: &mut Cpu, _: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        cpu.registers.x -= 1;
    });
}

pub fn adc(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        let operand = get_operand(cpu, &addr_mode);
        let value = operand.resolve_value(cpu);

        cpu.registers.acc += value
            + match cpu.registers.p.carry {
                true => 1,
                false => 0,
            };
    });
}

pub fn ldx(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        let operand = get_operand(cpu, &addr_mode);
        let value = operand.resolve_value(cpu);

        cpu.registers.x = value;
    });
}

pub fn stx(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        let operand = get_operand(cpu, &addr_mode);
        let addr = operand.resolve_addr();

        cpu.write_bytes_to(&addr, &[cpu.registers.x as u8]);
    });
}

pub fn ldy(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        let operand = get_operand(cpu, &addr_mode);
        let value = operand.resolve_value(cpu);

        cpu.registers.y = value;
    });
}

pub fn sty(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        let operand = get_operand(cpu, &addr_mode);
        let addr = operand.resolve_addr();

        cpu.write_bytes_to(&addr, &[cpu.registers.y as u8]);
    });
}

pub fn cpx(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        let operand = get_operand(cpu, &addr_mode);
        let value = operand.resolve_value(cpu);

        cpu.registers.p = compare(cpu.registers.p.clone(), cpu.registers.x, value);
    });
}

pub fn cpy(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        let operand = get_operand(cpu, &addr_mode);
        let value = operand.resolve_value(cpu);

        cpu.registers.p = compare(cpu.registers.p.clone(), cpu.registers.y, value);
    });
}

pub fn bne(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        let operand = get_operand(cpu, &addr_mode);
        let value = operand.resolve_value(cpu) as i8;

        if !cpu.registers.p.zero {
            cpu.registers.pc = (cpu.registers.pc as i32  + value as i32) as u16;
        }
    });
}

pub fn and(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn asl(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn bcc(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn bcs(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn beq(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn bit(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn bmi(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}


pub fn bpl(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn bvc(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn bvs(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn cld(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn cli(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn clv(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn cmp(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn clc(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn dec(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn dey(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn eor(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn inc(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn iny(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn jmp(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn jsr(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn lsr(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn nop(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn ora(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn pha(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn php(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn pla(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn plp(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn rol(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn ror(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn rti(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn rts(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn sbc(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn sec(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn sed(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn sei(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn tay(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn tsx(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn txa(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn txs(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

pub fn tya(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        panic!("instr not implemented!")
    });
}

fn compare(cpu_status: ProcStatusFlags, left: i8, right: i8) -> ProcStatusFlags {
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