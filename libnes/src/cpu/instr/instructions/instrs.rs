use byteorder::{ByteOrder, LittleEndian};

use super::helpers::*;
use crate::bits::{get_bit_val, lsb, msb, rotate, set_bit_val, RotateDirection};
use crate::cpu::cpu::Cpu;
use crate::cpu::instr::addressing::AddressingMode;

const BRK_INTERRUPT_VECTOR: u16 = 0xffe6;

pub fn adc(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    let registers = cpu.get_registers_mut();

    let carry_value = match registers.p.carry {
        true => 1,
        false => 0,
    };

    let to_add = value - carry_value;
    let old_acc = registers.acc;

    // overlowing_add actually checks if the value was CARRIED, not if there was a 2's complement overflow
    let (new_acc, did_carry) = (registers.acc as u8).overflowing_add(to_add as u8);

    let did_overflow = {
        let extended_acc = old_acc as i16;
        let extended_to_add = to_add as i16;

        let extended_result = extended_acc + extended_to_add;

        extended_result < -128 || extended_result > 127
    };

    registers.acc = new_acc as i8;

    registers.p.carry = did_carry;
    registers.p.zero = new_acc == 0;
    registers.p.overflow = did_overflow;
    registers.p.negative = get_bit_val(new_acc as u8, 7);
}

pub fn and(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    let registers = cpu.get_registers_mut();

    let new_acc = registers.acc & value;
    registers.acc = new_acc;

    set_zn_flags_from_result(cpu, new_acc as u8);
}

pub fn asl(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let (old_value, new_value) = match addr_mode {
        AddressingMode::Acc => {
            let registers = cpu.get_registers_mut();
            let old_value = registers.acc as u8;

            registers.acc = registers.acc << 1;

            (old_value, registers.acc as u8)
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

    let registers = cpu.get_registers_mut();

    registers.p.carry = get_bit_val(old_value as u8, 7);
    registers.p.zero = registers.acc == 0;
    registers.p.negative = get_bit_val(new_value, 7);
}

pub fn bcc(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    let registers = cpu.get_registers_mut();

    if !registers.p.carry {
        registers.pc = apply_branch_offset(registers.pc, value);
    }
}

pub fn bcs(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    let registers = cpu.get_registers_mut();

    if registers.p.carry {
        registers.pc = apply_branch_offset(registers.pc, value);
    }
}

pub fn beq(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    let registers = cpu.get_registers_mut();

    if registers.p.zero {
        registers.pc = apply_branch_offset(registers.pc, value);
    }
}

pub fn bit(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    let registers = cpu.get_registers_mut();

    let result = registers.acc & value;

    registers.p.zero = result == 0;
    registers.p.overflow = get_bit_val(value as u8, 6);
    registers.p.negative = get_bit_val(value as u8, 7);
}

pub fn bmi(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let offset = operand.resolve_value(cpu);

    let registers = cpu.get_registers_mut();

    if registers.p.negative {
        registers.pc = apply_branch_offset(registers.pc, offset);
    }
}

pub fn bne(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu) as i8;

    let registers = cpu.get_registers_mut();

    if !registers.p.zero {
        registers.pc = apply_branch_offset(registers.pc, value);
    }
}

pub fn bpl(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let offset = operand.resolve_value(cpu);

    let registers = cpu.get_registers_mut();

    if !registers.p.zero {
        registers.pc = apply_branch_offset(registers.pc, offset);
    }
}

pub fn brk(cpu: &mut impl Cpu, _: AddressingMode) {
    let (old_pc, old_p) = {
        let registers = cpu.get_registers_mut();

        (registers.pc.clone(), registers.p.clone())
    };

    cpu.push_u16(old_pc);
    cpu.push(old_p.into());

    let new_pc = cpu.read_u16_at(&(BRK_INTERRUPT_VECTOR.into()));

    let registers = cpu.get_registers_mut();
    registers.p.interrupt_disable = true;
    registers.pc = new_pc;
}

pub fn bvc(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let offset = operand.resolve_value(cpu);

    let registers = cpu.get_registers_mut();

    if !registers.p.negative {
        registers.pc = apply_branch_offset(registers.pc, offset);
    }
}

pub fn bvs(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let offset = operand.resolve_value(cpu);

    let registers = cpu.get_registers_mut();

    if registers.p.overflow {
        registers.pc = apply_branch_offset(registers.pc, offset);
    }
}

pub fn clc(cpu: &mut impl Cpu, _addr_mode: AddressingMode) {
    cpu.get_registers_mut().p.carry = false;
}

pub fn cld(cpu: &mut impl Cpu, _addr_mode: AddressingMode) {
    cpu.get_registers_mut().p.decimal_mode = false;
}

pub fn cli(cpu: &mut impl Cpu, _addr_mode: AddressingMode) {
    cpu.get_registers_mut().p.interrupt_disable = false;
}

pub fn clv(cpu: &mut impl Cpu, _addr_mode: AddressingMode) {
    cpu.get_registers_mut().p.overflow = false;
}

pub fn cmp(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu) as i8;

    let registers = cpu.get_registers_mut();

    let result = registers.acc - value;

    registers.p.carry = registers.acc >= value;
    registers.p.zero = registers.acc == value;
    registers.p.negative = get_bit_val(result as u8, 7);
}

pub fn cpx(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    let registers = cpu.get_registers_mut();

    registers.p = flags_from_compare(registers.p.clone(), registers.x as i8, value);
}

pub fn cpy(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    let registers = cpu.get_registers_mut();

    registers.p = flags_from_compare(registers.p.clone(), registers.y as i8, value);
}

pub fn dec(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let addr = operand.resolve_addr();

    let result = (operand.resolve_value(cpu) - 1) as u8;

    cpu.write_bytes_to(&addr, &[result]);

    let registers = cpu.get_registers_mut();

    registers.p.zero = result == 0;
    registers.p.negative = get_bit_val(result as u8, 7);
}

pub fn dex(cpu: &mut impl Cpu, _: AddressingMode) {
    let registers = cpu.get_registers_mut();

    let new_x = registers.x.overflowing_sub(1).0;
    registers.x = new_x;

    set_zn_flags_from_result(cpu, new_x);
}

pub fn dey(cpu: &mut impl Cpu, _addr_mode: AddressingMode) {
    let registers = cpu.get_registers_mut();

    let new_y = registers.y.overflowing_sub(1).0;
    registers.y = new_y;

    set_zn_flags_from_result(cpu, new_y);
}

pub fn eor(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    let registers = cpu.get_registers_mut();

    let new_acc = registers.acc ^ value;
    registers.acc = new_acc;

    set_zn_flags_from_result(cpu, new_acc as u8);
}

pub fn inc(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let addr = operand.resolve_addr();

    let result = (operand.resolve_value(cpu) + 1) as u8;

    cpu.write_bytes_to(&addr, &[result]);

    set_zn_flags_from_result(cpu, result);
}

pub fn inx(cpu: &mut impl Cpu, _: AddressingMode) {
    let registers = cpu.get_registers_mut();

    let new_x = registers.x.overflowing_add(1).0;
    registers.x = new_x;

    set_zn_flags_from_result(cpu, new_x);
}

pub fn iny(cpu: &mut impl Cpu, _addr_mode: AddressingMode) {
    let registers = cpu.get_registers_mut();

    let new_y = registers.y.overflowing_add(1).0;
    registers.y = new_y;

    set_zn_flags_from_result(cpu, new_y);
}

pub fn jmp(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let addr = match addr_mode {
        AddressingMode::Indirect => {
            let raw_addr = cpu.next_u16();

            // 6502 jmp bug (https://en.wikipedia.org/wiki/MOS_Technology_6502#Bugs_and_quirks)
            match raw_addr & 0x00ff == 0x00ff {
                false => operand.resolve_addr(),
                true => {
                    let actual_addr_lo = cpu.read_u8_at(&raw_addr.into());
                    let actual_addr_hi = cpu.read_u8_at(&(raw_addr & 0xff00).into());
                    let actual_addr =
                        LittleEndian::read_u16(&[actual_addr_lo, actual_addr_hi]).into();

                    cpu.read_u16_at(&actual_addr).into()
                }
            }
        }
        _ => operand.resolve_addr(),
    };

    cpu.get_registers_mut().pc = addr.into();
}

pub fn jsr(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let return_addr = {
        let registers = cpu.get_registers_mut();

        registers.pc + 2
    };

    let operand = get_operand(cpu, &addr_mode);
    let addr = operand.resolve_addr();

    cpu.push_bytes(&[msb(return_addr), lsb(return_addr)]);

    let registers = cpu.get_registers_mut();
    registers.pc = addr.into();
}

pub fn lda(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    cpu.get_registers_mut().acc = value;

    set_zn_flags_from_result(cpu, value as u8);
}

pub fn ldx(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu) as u8;

    cpu.get_registers_mut().x = value;

    set_zn_flags_from_result(cpu, value);
}

pub fn ldy(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu) as u8;

    cpu.get_registers_mut().y = value;

    set_zn_flags_from_result(cpu, value);
}

pub fn lsr(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);

    let (new_carry_flag, result) = match addr_mode {
        AddressingMode::Acc => {
            let registers = cpu.get_registers_mut();

            let old_bit_0 = get_bit_val(registers.acc as u8, 0);
            let value = set_bit_val((registers.acc as u8) >> 1, 7, false);

            registers.acc = value as i8;

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

    let registers = cpu.get_registers_mut();

    registers.p.carry = new_carry_flag;
    registers.p.zero = result == 0;
    registers.p.negative = get_bit_val(result, 7);
}

pub fn nop(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    // do...nothing
}

pub fn ora(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    let registers = cpu.get_registers_mut();

    registers.acc = registers.acc | value;

    registers.p.zero = registers.acc == 0;
    registers.p.negative = get_bit_val(registers.acc as u8, 7);
}

pub fn pha(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let acc = cpu.get_registers_mut().acc;

    cpu.push(acc as u8);
}

pub fn php(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let p = cpu.get_registers_mut().p.clone();

    cpu.push(p.into());
}

pub fn pla(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let value = cpu.pop();

    let registers = cpu.get_registers_mut();

    registers.acc = value as i8;

    registers.p.zero = registers.acc == 0;
    registers.p.negative = get_bit_val(registers.acc as u8, 7);
}

pub fn plp(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let value = cpu.pop();

    cpu.get_registers_mut().p = value.into();
}

pub fn rol(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let new_carry_flag = match &addr_mode {
        &AddressingMode::Acc => {
            let registers = cpu.get_registers_mut();

            let (new_value, old_bit_7) = rotate(registers.acc as u8, RotateDirection::Left);

            // acc <- acc shifted one bit with bit 0 set to the old carry flag
            registers.acc = set_bit_val(new_value, 0, registers.p.carry) as i8;

            old_bit_7
        }
        _ => {
            let operand = get_operand(cpu, &addr_mode);
            let addr = operand.resolve_addr();
            let value = operand.resolve_value(cpu) as u8;

            let registers = cpu.get_registers_mut();

            let (new_value, old_bit_7) = rotate(value, RotateDirection::Left);
            let carry = registers.p.carry;

            cpu.write_bytes_to(&addr, &[set_bit_val(new_value, 0, carry)]);

            old_bit_7
        }
    };

    let registers = cpu.get_registers_mut();
    registers.p.carry = new_carry_flag;
}

pub fn ror(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let new_carry_flag = match &addr_mode {
        &AddressingMode::Acc => {
            let registers = cpu.get_registers_mut();
            let (new_value, old_bit_0) = rotate(registers.acc as u8, RotateDirection::Right);

            // acc <- acc shifted one bit with bit 7 set to the old carry flag
            registers.acc = set_bit_val(new_value, 7, registers.p.carry) as i8;

            old_bit_0
        }
        _ => {
            let operand = get_operand(cpu, &addr_mode);
            let addr = operand.resolve_addr();
            let value = operand.resolve_value(cpu) as u8;

            let registers = cpu.get_registers_mut();

            let (new_value, old_bit_0) = rotate(value, RotateDirection::Right);
            let carry = registers.p.carry;

            cpu.write_bytes_to(&addr, &[set_bit_val(new_value, 7, carry)]);

            old_bit_0
        }
    };

    let registers = cpu.get_registers_mut();
    registers.p.carry = new_carry_flag;
}

pub fn rti(cpu: &mut impl Cpu, _addr_mode: AddressingMode) {
    let (new_p, new_pc) = (cpu.pop().clone().into(), cpu.pop_u16().clone());

    let registers = cpu.get_registers_mut();

    registers.p = new_p;
    registers.pc = new_pc;
}

pub fn rts(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let addr_lo = cpu.pop();
    let addr_hi = cpu.pop();

    let addr = LittleEndian::read_u16(&[addr_lo, addr_hi]);

    cpu.get_registers_mut().pc = addr;
}

pub fn sbc(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let value = operand.resolve_value(cpu);

    let registers = cpu.get_registers_mut();

    let not_of_carry_value = match registers.p.carry {
        true => 0,
        false => 1,
    };

    let to_subtract = value - not_of_carry_value;
    let old_acc = registers.acc;

    // overlowing_add actually checks if the value was CARRIED, not if there was a 2's complement overflow
    let (new_acc, did_carry) = (registers.acc as u8).overflowing_sub(to_subtract as u8);

    let did_overflow = {
        let extended_acc = old_acc as i16;
        let extended_to_subtract = to_subtract as i16;

        let extended_result = extended_acc - extended_to_subtract;

        extended_result < -128 || extended_result > 127
    };

    registers.acc = new_acc as i8;

    registers.p.carry = did_carry;
    registers.p.zero = new_acc == 0;
    registers.p.overflow = did_overflow;
    registers.p.negative = get_bit_val(new_acc as u8, 7);
}

pub fn sec(cpu: &mut impl Cpu, _addr_mode: AddressingMode) {
    cpu.get_registers_mut().p.carry = true;
}

pub fn sed(cpu: &mut impl Cpu, _addr_mode: AddressingMode) {
    cpu.get_registers_mut().p.decimal_mode = true;
}

pub fn sei(cpu: &mut impl Cpu, _addr_mode: AddressingMode) {
    cpu.get_registers_mut().p.interrupt_disable = true;
}

pub fn sta(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let operand_addr = operand.resolve_addr();

    let acc = cpu.get_registers_mut().acc;
    cpu.write_bytes_to(&operand_addr, &[acc as u8]);
}

pub fn stp(cpu: &mut impl Cpu, _: AddressingMode) {
    cpu.stop();
}

pub fn stx(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let addr = operand.resolve_addr();

    let x = cpu.get_registers_mut().x;
    cpu.write_bytes_to(&addr, &[x]);
}

pub fn sty(cpu: &mut impl Cpu, addr_mode: AddressingMode) {
    let operand = get_operand(cpu, &addr_mode);
    let addr = operand.resolve_addr();

    let y = cpu.get_registers_mut().y;
    cpu.write_bytes_to(&addr, &[y]);
}

pub fn tax(cpu: &mut impl Cpu, _: AddressingMode) {
    let registers = cpu.get_registers_mut();

    let new_x = registers.acc as u8;
    registers.x = new_x;

    set_zn_flags_from_result(cpu, new_x);
}

pub fn tay(cpu: &mut impl Cpu, _addr_mode: AddressingMode) {
    let registers = cpu.get_registers_mut();

    let new_y = registers.acc as u8;
    registers.y = new_y;

    set_zn_flags_from_result(cpu, new_y);
}

pub fn tsx(cpu: &mut impl Cpu, _addr_mode: AddressingMode) {
    let registers = cpu.get_registers_mut();

    let new_x = registers.sp;
    registers.x = new_x;

    set_zn_flags_from_result(cpu, new_x);
}

pub fn txa(cpu: &mut impl Cpu, _addr_mode: AddressingMode) {
    let registers = cpu.get_registers_mut();

    let new_acc = registers.x as i8;
    registers.acc = new_acc;

    set_zn_flags_from_result(cpu, new_acc as u8);
}

pub fn txs(cpu: &mut impl Cpu, _addr_mode: AddressingMode) {
    cpu.get_registers_mut().sp = cpu.get_registers_mut().x;
}

pub fn tya(cpu: &mut impl Cpu, _addr_mode: AddressingMode) {
    let registers = cpu.get_registers_mut();

    let new_acc = registers.y as i8;
    registers.acc = new_acc;

    set_zn_flags_from_result(cpu, new_acc as u8);
}

// helpers

fn apply_branch_offset(pc: u16, offset: i8) -> u16 {
    (pc as i32 + offset as i32) as u16
}

fn set_zn_flags_from_result(cpu: &mut impl Cpu, result: u8) {
    let registers = cpu.get_registers_mut();

    registers.p.zero = result == 0;
    registers.p.negative = get_bit_val(result, 7);
}
