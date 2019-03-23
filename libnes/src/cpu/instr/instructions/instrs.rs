use super::helpers::*;
use crate::bits::{get_bit_val, lsb, msb, set_bit_val};
use crate::cpu::instr::addressing::AddressingMode;
use crate::cpu::Cpu;

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

        cpu.registers.p = flags_from_compare(cpu.registers.p.clone(), cpu.registers.x, value);
    });
}

pub fn cpy(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        let operand = get_operand(cpu, &addr_mode);
        let value = operand.resolve_value(cpu);

        cpu.registers.p = flags_from_compare(cpu.registers.p.clone(), cpu.registers.y, value);
    });
}

pub fn bne(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        let operand = get_operand(cpu, &addr_mode);
        let value = operand.resolve_value(cpu) as i8;

        if !cpu.registers.p.zero {
            cpu.registers.pc = apply_branch_offset(cpu.registers.pc, value);
        }
    });
}

pub fn and(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        let operand = get_operand(cpu, &addr_mode);
        let value = operand.resolve_value(cpu);

        cpu.registers.acc &= value;
    });
}

pub fn asl(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        let old_value = match addr_mode {
            AddressingMode::Acc => {
                let old_value = cpu.registers.acc as u8;

                cpu.registers.acc <<= 1;

                old_value
            }
            _ => {
                let operand = get_operand(cpu, &addr_mode);
                let addr = operand.resolve_addr();
                let old_value = cpu.read_u8_at(&addr);

                cpu.write_bytes_to(&addr, &[old_value << 1]);

                old_value
            }
        };

        cpu.registers.p.carry = get_bit_val(old_value as u8, 7);
    });
}

pub fn bcc(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        let operand = get_operand(cpu, &addr_mode);
        let value = operand.resolve_value(cpu);

        if !cpu.registers.p.carry {
            cpu.registers.pc = apply_branch_offset(cpu.registers.pc, value);
        }
    });
}

pub fn bcs(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        let operand = get_operand(cpu, &addr_mode);
        let value = operand.resolve_value(cpu);

        if cpu.registers.p.carry {
            cpu.registers.pc = apply_branch_offset(cpu.registers.pc, value);
        }
    });
}

pub fn beq(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        let operand = get_operand(cpu, &addr_mode);
        let value = operand.resolve_value(cpu);

        if cpu.registers.p.zero {
            cpu.registers.pc = apply_branch_offset(cpu.registers.pc, value);
        }
    });
}

pub fn bit(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        let operand = get_operand(cpu, &addr_mode);
        let value = operand.resolve_value(cpu);

        let result = cpu.registers.acc & value;

        cpu.registers.p.overflow = get_bit_val(result as u8, 6);
        cpu.registers.p.negative = get_bit_val(result as u8, 7);
    });
}

pub fn bmi(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        let operand = get_operand(cpu, &addr_mode);
        let offset = operand.resolve_value(cpu);

        if cpu.registers.p.negative {
            cpu.registers.pc = apply_branch_offset(cpu.registers.pc, offset);
        }
    });
}

pub fn bpl(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        let operand = get_operand(cpu, &addr_mode);
        let offset = operand.resolve_value(cpu);

        if !cpu.registers.p.zero {
            cpu.registers.pc = apply_branch_offset(cpu.registers.pc, offset);
        }
    });
}

pub fn bvc(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        let operand = get_operand(cpu, &addr_mode);
        let offset = operand.resolve_value(cpu);

        if !cpu.registers.p.negative {
            cpu.registers.pc = apply_branch_offset(cpu.registers.pc, offset);
        }
    });
}

pub fn bvs(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        let operand = get_operand(cpu, &addr_mode);
        let offset = operand.resolve_value(cpu);

        if cpu.registers.p.overflow {
            cpu.registers.pc = apply_branch_offset(cpu.registers.pc, offset);
        }
    });
}

pub fn cld(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| panic!("instr not implemented!"));
}

pub fn cli(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| panic!("instr not implemented!"));
}

pub fn clv(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| panic!("instr not implemented!"));
}

pub fn cmp(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| panic!("instr not implemented!"));
}

pub fn clc(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        cpu.registers.p.carry = false;
    });
}

pub fn dec(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        let operand = get_operand(cpu, &addr_mode);
        let addr = operand.resolve_addr();

        let new_value = (operand.resolve_value(cpu) - 1) as u8;

        cpu.write_bytes_to(&addr, &[new_value]);
    });
}

pub fn dex(cpu: &mut Cpu, _: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        cpu.registers.x -= 1;
    });
}

pub fn dey(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        cpu.registers.y -= 1;
    });
}

pub fn eor(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        let operand = get_operand(cpu, &addr_mode);
        let value = operand.resolve_value(cpu);

        cpu.registers.acc = cpu.registers.acc ^ value;

        cpu.registers.p.carry = cpu.registers.acc == 0;
        cpu.registers.p.negative = get_bit_val(cpu.registers.acc as u8, 7);
    });
}

pub fn inc(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| {
        let operand = get_operand(cpu, &addr_mode);
        let addr = operand.resolve_addr();

        let new_value = (operand.resolve_value(cpu) + 1) as u8;

        cpu.write_bytes_to(&addr, &[new_value]);
    });
}

pub fn iny(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| panic!("instr not implemented!"));
}

pub fn jmp(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| panic!("instr not implemented!"));
}

pub fn jsr(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| panic!("instr not implemented!"));
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

pub fn tay(cpu: &mut Cpu, addr_mode: AddressingMode) {
    cpu.perform_instr(|cpu: &mut Cpu| panic!("instr not implemented!"));
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

fn apply_branch_offset(pc: u16, offset: i8) -> u16 {
    (pc as i32 + offset as i32) as u16
}
