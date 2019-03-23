use super::helpers::*;
use crate::bits::{get_bit_val, lsb, msb, set_bit_val};
use crate::cpu::instr::addressing::AddressingMode;
use crate::cpu::Cpu;

pub fn adc(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    cpu.registers.acc += value
        + match cpu.registers.p.carry {
            true => 1,
            false => 0,
        };

    // TODO: impl flags
}

pub fn and(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    cpu.registers.acc &= value;

    set_zn_flags_from_result(cpu, cpu.registers.acc);
}

pub fn asl(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let (old_value, new_value) = match addr_mode {
        AddressingMode::Acc => {
            let old_value = cpu.registers.acc as u8;

            cpu.registers.acc <<= 1;

            (old_value, cpu.registers.acc as u8)
        }
        _ => {
            let operand = get_operand(cpu, &addr_mode);
            let addr = operand.resolve_addr();
            let old_value = cpu.read_u8_at(&addr);

            let result = old_value << 1;

            cpu.write_bytes_to(&addr, &[result]);

            (old_value, result)
        }
    };

    cpu.registers.p.carry = get_bit_val(old_value as u8, 7);
    cpu.registers.p.zero = cpu.registers.acc == 0;
    cpu.registers.p.negative = get_bit_val(new_value, 7);
}

pub fn bcc(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    if !cpu.registers.p.carry {
        cpu.registers.pc = apply_branch_offset(cpu.registers.pc, value);
    }
}

pub fn bcs(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    if cpu.registers.p.carry {
        cpu.registers.pc = apply_branch_offset(cpu.registers.pc, value);
    }
}

pub fn beq(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    if cpu.registers.p.zero {
        cpu.registers.pc = apply_branch_offset(cpu.registers.pc, value);
    }
}

pub fn bit(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    let result = cpu.registers.acc & value;

    cpu.registers.p.zero = result == 0;
    cpu.registers.p.overflow = get_bit_val(value as u8, 6);
    cpu.registers.p.negative = get_bit_val(value as u8, 7);
}

pub fn bmi(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let offset = operand.resolve_value(cpu);

    if cpu.registers.p.negative {
        cpu.registers.pc = apply_branch_offset(cpu.registers.pc, offset);
    }
}

pub fn bne(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu) as i8;

    if !cpu.registers.p.zero {
        cpu.registers.pc = apply_branch_offset(cpu.registers.pc, value);
    }
}

pub fn bpl(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let offset = operand.resolve_value(cpu);

    if !cpu.registers.p.zero {
        cpu.registers.pc = apply_branch_offset(cpu.registers.pc, offset);
    }
}

pub fn brk(cpu: &mut Cpu, _: AddressingMode) {
    cpu.push(msb(cpu.registers.pc));
    cpu.push(lsb(cpu.registers.pc));
    cpu.push(cpu.registers.p.clone().into());

    cpu.registers.p.interrupt_disable = true;
    cpu.registers.pc = cpu.read_u16_at(&(0xffe6.into()));
}

pub fn bvc(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let offset = operand.resolve_value(cpu);

    if !cpu.registers.p.negative {
        cpu.registers.pc = apply_branch_offset(cpu.registers.pc, offset);
    }
}

pub fn bvs(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let offset = operand.resolve_value(cpu);

    if cpu.registers.p.overflow {
        cpu.registers.pc = apply_branch_offset(cpu.registers.pc, offset);
    }
}

pub fn clc(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.registers.p.carry = false;
}

pub fn cld(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.registers.p.decimal_mode = false;
}

pub fn cli(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.registers.p.interrupt_disable = false;
}

pub fn clv(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.registers.p.overflow = false;
}

pub fn cmp(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu) as i8;

    let result = cpu.registers.acc - value;

    cpu.registers.p.carry = cpu.registers.acc >= value;
    cpu.registers.p.zero = cpu.registers.acc == value;
    cpu.registers.p.negative = get_bit_val(result as u8, 7);
}

pub fn cpx(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    cpu.registers.p = flags_from_compare(cpu.registers.p.clone(), cpu.registers.x, value);
}

pub fn cpy(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    cpu.registers.p = flags_from_compare(cpu.registers.p.clone(), cpu.registers.y, value);
}

pub fn dec(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let addr = operand.resolve_addr();

    let result = (operand.resolve_value(cpu) - 1) as u8;

    cpu.write_bytes_to(&addr, &[result]);

    cpu.registers.p.zero = result == 0;
    cpu.registers.p.negative = get_bit_val(result as u8, 7);
}

pub fn dex(cpu: &mut Cpu, _: AddressingMode) {
    cpu.registers.x -= 1;

    set_zn_flags_from_result(cpu, cpu.registers.x);
}

pub fn dey(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.registers.y -= 1;

    set_zn_flags_from_result(cpu, cpu.registers.y);
}

pub fn eor(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    cpu.registers.acc ^= value;

    set_zn_flags_from_result(cpu, cpu.registers.acc);
}

pub fn inc(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let addr = operand.resolve_addr();

    let result = (operand.resolve_value(cpu) + 1) as u8;

    cpu.write_bytes_to(&addr, &[result]);

    set_zn_flags_from_result(cpu, result as i8);
}

pub fn inx(cpu: &mut Cpu, _: AddressingMode) {
    cpu.registers.x += 1;

    set_zn_flags_from_result(cpu, cpu.registers.x);
}

pub fn iny(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.registers.y += 1;

    set_zn_flags_from_result(cpu, cpu.registers.y);
}

pub fn jmp(cpu: &mut Cpu, addr_mode: AddressingMode) {
    // TODO: handle 6502 jmp bug

    let operand = get_operand(cpu, &addr_mode);
    let addr = operand.resolve_addr();

    cpu.registers.pc = addr.into();
}

pub fn jsr(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| panic!("instr not implemented!"));
}

pub fn lda(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    cpu.registers.acc = value;

    set_zn_flags_from_result(cpu, cpu.registers.acc);
}

pub fn ldx(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    cpu.registers.x = value;

    set_zn_flags_from_result(cpu, cpu.registers.x);
}

pub fn ldy(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    cpu.registers.y = value;

    set_zn_flags_from_result(cpu, cpu.registers.y);
}

pub fn lsr(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| panic!("instr not implemented!"));
}

pub fn nop(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| panic!("instr not implemented!"));
}

pub fn ora(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| panic!("instr not implemented!"));
}

pub fn pha(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| panic!("instr not implemented!"));
}

pub fn php(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| panic!("instr not implemented!"));
}

pub fn pla(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| panic!("instr not implemented!"));
}

pub fn plp(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| panic!("instr not implemented!"));
}

pub fn rol(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| panic!("instr not implemented!"));
}

pub fn ror(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| panic!("instr not implemented!"));
}

pub fn rti(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| panic!("instr not implemented!"));
}

pub fn rts(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| panic!("instr not implemented!"));
}

pub fn sbc(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| panic!("instr not implemented!"));
}

pub fn sec(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| panic!("instr not implemented!"));
}

pub fn sed(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| panic!("instr not implemented!"));
}

pub fn sei(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| panic!("instr not implemented!"));
}

pub fn sta(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let operand_addr = operand.resolve_addr();

    cpu.write_bytes_to(&operand_addr, &[cpu.registers.acc as u8]);
}

pub fn stp(cpu: &mut Cpu, _: AddressingMode) {
    cpu.stop();
}

pub fn stx(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let addr = operand.resolve_addr();

    cpu.write_bytes_to(&addr, &[cpu.registers.x as u8]);
}

pub fn sty(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let addr = operand.resolve_addr();

    cpu.write_bytes_to(&addr, &[cpu.registers.y as u8]);
}

pub fn tax(cpu: &mut Cpu, _: AddressingMode) {
    cpu.registers.x = cpu.registers.acc;

    set_zn_flags_from_result(cpu, cpu.registers.x);
}

pub fn tay(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.registers.y = cpu.registers.acc;

    set_zn_flags_from_result(cpu, cpu.registers.y);
}

pub fn tsx(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| panic!("instr not implemented!"));
}

pub fn txa(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| panic!("instr not implemented!"));
}

pub fn txs(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| panic!("instr not implemented!"));
}

pub fn tya(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| panic!("instr not implemented!"));
}

// helpers

fn apply_branch_offset(pc: u16, offset: i8) -> u16 {
    (pc as i32 + offset as i32) as u16
}

fn set_zn_flags_from_result(cpu: &mut Cpu, result: i8) {
    cpu.registers.p.zero = result == 0;
    cpu.registers.p.negative = get_bit_val(result as u8, 7);
}