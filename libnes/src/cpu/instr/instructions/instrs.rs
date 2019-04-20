use byteorder::{ByteOrder, LittleEndian};

use super::helpers::*;
use crate::bits::{get_bit_val, lsb, msb, rotate, set_bit_val, RotateDirection};
use crate::cpu::instr::addressing::AddressingMode;
use crate::cpu::Cpu;

pub fn adc(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    let carry_value = match cpu.registers.p.carry {
        true => 1,
        false => 0,
    };

    let to_add = value - carry_value;
    let old_acc = cpu.registers.acc;

    // overlowing_add actually checks if the value was CARRIED, not if there was a 2's complement overflow
    let (new_acc, did_carry) = (cpu.registers.acc as u8).overflowing_add(to_add as u8);

    let did_overflow = {
        let extended_acc = old_acc as i16;
        let extended_to_add = to_add as i16;

        let extended_result = extended_acc + extended_to_add;

        extended_result < -128 || extended_result > 127
    };

    cpu.registers.acc = new_acc as i8;

    cpu.registers.p.carry = did_carry;
    cpu.registers.p.zero = new_acc == 0;
    cpu.registers.p.overflow = did_overflow;
    cpu.registers.p.negative = get_bit_val(new_acc as u8, 7);
}

pub fn and(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    cpu.registers.acc &= value;

    set_zn_flags_from_result(cpu, cpu.registers.acc as u8);
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
    cpu.push_u16(cpu.registers.pc);
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

pub fn clc(cpu: &mut Cpu, _addr_mode: AddressingMode) {
    cpu.registers.p.carry = false;
}

pub fn cld(cpu: &mut Cpu, _addr_mode: AddressingMode) {
    cpu.registers.p.decimal_mode = false;
}

pub fn cli(cpu: &mut Cpu, _addr_mode: AddressingMode) {
    cpu.registers.p.interrupt_disable = false;
}

pub fn clv(cpu: &mut Cpu, _addr_mode: AddressingMode) {
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

    cpu.registers.p = flags_from_compare(cpu.registers.p.clone(), cpu.registers.x as i8, value);
}

pub fn cpy(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    cpu.registers.p = flags_from_compare(cpu.registers.p.clone(), cpu.registers.y as i8, value);
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
    cpu.registers.x = cpu.registers.x.overflowing_sub(1).0;

    set_zn_flags_from_result(cpu, cpu.registers.x);
}

pub fn dey(cpu: &mut Cpu, _addr_mode: AddressingMode) {
    cpu.registers.y = cpu.registers.y.overflowing_sub(1).0;

    set_zn_flags_from_result(cpu, cpu.registers.y);
}

pub fn eor(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    cpu.registers.acc ^= value;

    set_zn_flags_from_result(cpu, cpu.registers.acc as u8);
}

pub fn inc(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let addr = operand.resolve_addr();

    let result = (operand.resolve_value(cpu) + 1) as u8;

    cpu.write_bytes_to(&addr, &[result]);

    set_zn_flags_from_result(cpu, result);
}

pub fn inx(cpu: &mut Cpu, _: AddressingMode) {
    cpu.registers.x = cpu.registers.x.overflowing_add(1).0;

    set_zn_flags_from_result(cpu, cpu.registers.x);
}

pub fn iny(cpu: &mut Cpu, _addr_mode: AddressingMode) {
    cpu.registers.y = cpu.registers.y.overflowing_add(1).0;

    set_zn_flags_from_result(cpu, cpu.registers.y);
}

pub fn jmp(cpu: &mut Cpu, addr_mode: AddressingMode) {
    // TODO: handle 6502 jmp bug

    let operand = get_operand(cpu, &addr_mode);
    let addr = operand.resolve_addr();

    cpu.registers.pc = addr.into();
}

pub fn jsr(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let return_addr = cpu.registers.pc + 2;

    let operand = get_operand(cpu, &addr_mode);
    let addr = operand.resolve_addr();

    cpu.push_bytes(&[msb(return_addr), lsb(return_addr)]);

    cpu.registers.pc = addr.into();
}

pub fn lda(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    cpu.registers.acc = value;

    set_zn_flags_from_result(cpu, cpu.registers.acc as u8);
}

pub fn ldx(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    cpu.registers.x = value as u8;

    set_zn_flags_from_result(cpu, cpu.registers.x);
}

pub fn ldy(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    cpu.registers.y = value as u8;

    set_zn_flags_from_result(cpu, cpu.registers.y);
}

pub fn lsr(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);

    let (new_carry_flag, result) = match addr_mode {
        AddressingMode::Acc => {
            let old_bit_0 = get_bit_val(cpu.registers.acc as u8, 0);
            let value = set_bit_val((cpu.registers.acc as u8) >> 1, 7, false);

            cpu.registers.acc = value as i8;

            (old_bit_0, value)
        }
        _ => {
            let addr = operand.resolve_addr();
            let value = operand.resolve_value(cpu) as u8;

            let old_bit_0 = get_bit_val(value, 0);
            let new_value = set_bit_val(value >> 1, 7, false);

            cpu.write_bytes_to(&addr, &[new_value]);

            (old_bit_0, new_value)
        }
    };

    cpu.registers.p.carry = new_carry_flag;
    cpu.registers.p.zero = result == 0;
    cpu.registers.p.negative = get_bit_val(result, 7);
}

pub fn nop(cpu: &mut Cpu, addr_mode: AddressingMode) {
    // do...nothing
}

pub fn ora(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    cpu.registers.acc = cpu.registers.acc | value;

    cpu.registers.p.zero = cpu.registers.acc == 0;
    cpu.registers.p.negative = get_bit_val(cpu.registers.acc as u8, 7);
}

pub fn pha(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.push(cpu.registers.acc as u8);
}

pub fn php(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.push(cpu.registers.p.clone().into());
}

pub fn pla(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let value = cpu.pop();

    cpu.registers.acc = value as i8;

    cpu.registers.p.zero = cpu.registers.acc == 0;
    cpu.registers.p.negative = get_bit_val(cpu.registers.acc as u8, 7);
}

pub fn plp(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let value = cpu.pop();

    cpu.registers.p = value.into();
}

pub fn rol(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let new_carry_flag = match &addr_mode {
        &AddressingMode::Acc => {
            let (new_value, old_bit_7) = rotate(cpu.registers.acc as u8, RotateDirection::Left);

            // acc <- acc shifted one bit with bit 0 set to the old carry flag
            cpu.registers.acc = set_bit_val(new_value, 0, cpu.registers.p.carry) as i8;

            old_bit_7
        }
        _ => {
            let operand = get_operand(cpu, &addr_mode);
            let addr = operand.resolve_addr();
            let value = operand.resolve_value(cpu) as u8;

            let (new_value, old_bit_7) = rotate(value, RotateDirection::Left);

            cpu.write_bytes_to(&addr, &[set_bit_val(new_value, 0, cpu.registers.p.carry)]);

            old_bit_7
        }
    };

    cpu.registers.p.carry = new_carry_flag;
}

pub fn ror(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let new_carry_flag = match &addr_mode {
        &AddressingMode::Acc => {
            let (new_value, old_bit_0) = rotate(cpu.registers.acc as u8, RotateDirection::Right);

            // acc <- acc shifted one bit with bit 7 set to the old carry flag
            cpu.registers.acc = set_bit_val(new_value, 7, cpu.registers.p.carry) as i8;

            old_bit_0
        }
        _ => {
            let operand = get_operand(cpu, &addr_mode);
            let addr = operand.resolve_addr();
            let value = operand.resolve_value(cpu) as u8;

            let (new_value, old_bit_0) = rotate(value, RotateDirection::Right);

            cpu.write_bytes_to(&addr, &[set_bit_val(new_value, 7, cpu.registers.p.carry)]);

            old_bit_0
        }
    };

    cpu.registers.p.carry = new_carry_flag;
}

pub fn rti(cpu: &mut Cpu, _addr_mode: AddressingMode) {
    cpu.registers.p = cpu.pop().clone().into();
    cpu.registers.pc = cpu.pop_u16().clone();
}

pub fn rts(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let addr_lo = cpu.pop();
    let addr_hi = cpu.pop();

    let addr = LittleEndian::read_u16(&[addr_lo, addr_hi]);

    cpu.registers.pc = addr;
}

pub fn sbc(cpu: &mut Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    let not_of_carry_value = match cpu.registers.p.carry {
        true => 0,
        false => 1,
    };

    let to_subtract = value - not_of_carry_value;
    let old_acc = cpu.registers.acc;

    // overlowing_add actually checks if the value was CARRIED, not if there was a 2's complement overflow
    let (new_acc, did_carry) = (cpu.registers.acc as u8).overflowing_sub(to_subtract as u8);

    let did_overflow = {
        let extended_acc = old_acc as i16;
        let extended_to_subtract = to_subtract as i16;

        let extended_result = extended_acc - extended_to_subtract;

        extended_result < -128 || extended_result > 127
    };

    cpu.registers.acc = new_acc as i8;

    cpu.registers.p.carry = did_carry;
    cpu.registers.p.zero = new_acc == 0;
    cpu.registers.p.overflow = did_overflow;
    cpu.registers.p.negative = get_bit_val(new_acc as u8, 7);
}

pub fn sec(cpu: &mut Cpu, _addr_mode: AddressingMode) {
    cpu.registers.p.carry = true;
}

pub fn sed(cpu: &mut Cpu, _addr_mode: AddressingMode) {
    cpu.registers.p.decimal_mode = true;
}

pub fn sei(cpu: &mut Cpu, _addr_mode: AddressingMode) {
    cpu.registers.p.interrupt_disable = true;
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
    cpu.registers.x = cpu.registers.acc as u8;

    set_zn_flags_from_result(cpu, cpu.registers.x);
}

pub fn tay(cpu: &mut Cpu, _addr_mode: AddressingMode) {
    cpu.registers.y = cpu.registers.acc as u8;

    set_zn_flags_from_result(cpu, cpu.registers.y);
}

pub fn tsx(cpu: &mut Cpu, _addr_mode: AddressingMode) {
    cpu.registers.x = cpu.registers.sp;

    set_zn_flags_from_result(cpu, cpu.registers.x);
}

pub fn txa(cpu: &mut Cpu, _addr_mode: AddressingMode) {
    cpu.registers.acc = cpu.registers.x as i8;

    set_zn_flags_from_result(cpu, cpu.registers.acc as u8);
}

pub fn txs(cpu: &mut Cpu, _addr_mode: AddressingMode) {
    cpu.registers.sp = cpu.registers.x;
}

pub fn tya(cpu: &mut Cpu, _addr_mode: AddressingMode) {
    cpu.registers.acc = cpu.registers.y as i8;

    set_zn_flags_from_result(cpu, cpu.registers.acc as u8);
}

// helpers

fn apply_branch_offset(pc: u16, offset: i8) -> u16 {
    (pc as i32 + offset as i32) as u16
}

fn set_zn_flags_from_result(cpu: &mut Cpu, result: u8) {
    cpu.registers.p.zero = result == 0;
    cpu.registers.p.negative = get_bit_val(result, 7);
}
