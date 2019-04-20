use itertools::*;

use super::{Cpu, BRK_INTERRUPT_ADDR_START, RESET_INTERRUPT_ADDR_START};
use crate::bits::{lsb, msb, to_bytes};
use crate::mem::Address;

const DEFAULT_LOAD_PROG_OPTS: LoadProgramOptions = LoadProgramOptions {
    start_addr: 0x0600,
    load_addr: 0x0600,
    debug: false,
};

pub fn load_program_bytes(cpu: &mut Cpu, prog: &[u8]) {
    load_program_bytes_with_options(cpu, prog, DEFAULT_LOAD_PROG_OPTS);
}

pub fn load_program_bytes_with_options(cpu: &mut Cpu, prog: &[u8], opts: LoadProgramOptions) {
    let prog_str: String = prog.iter().map(|b| format!("{:x}", b)).join(" ");

    load_program_str_with_options(cpu, prog_str.as_str(), opts)
}

pub fn load_program_string(cpu: &mut Cpu, prog: String) {
    load_program_string_with_options(cpu, prog, DEFAULT_LOAD_PROG_OPTS)
}

pub fn load_program_string_with_options(cpu: &mut Cpu, prog: String, opts: LoadProgramOptions) {
    load_program_str_with_options(cpu, prog.as_str(), opts)
}

pub fn load_program_str<'a>(cpu: &mut Cpu, prog: &'a str) {
    load_program_str_with_options(cpu, prog, DEFAULT_LOAD_PROG_OPTS)
}

pub fn load_program_str_with_options<'a>(cpu: &mut Cpu, prog: &'a str, opts: LoadProgramOptions) {
    if opts.debug {
        println!("Loading program at addr {:#06x}", opts.load_addr);
    }

    cpu.write_bytes_to(&Address::from(opts.load_addr), &to_bytes(prog));

    if opts.debug {
        println!(
            "Writing reset interrupt vec to point to addr {:#06x}",
            opts.start_addr
        );
    }

    // write interrupt routine addr
    cpu.write_bytes_to(
        &Address::from(RESET_INTERRUPT_ADDR_START),
        &[lsb(opts.start_addr), msb(opts.start_addr)],
    );

    // write stp instr to brk irq vector address (so that'll be run at the first brk)
    cpu.write_bytes_to(&Address::from(BRK_INTERRUPT_ADDR_START), &[0xef, 0xbe]);
    cpu.write_bytes_to(&Address::from(0xbeef), &to_bytes("db"));

    if opts.debug {
        println!("peeking load addr block:");
        peek_mem(cpu, opts.load_addr);

        println!("peeking start addr block:");
        peek_mem(cpu, opts.start_addr);
    }
}

pub fn peek_mem(cpu: &mut Cpu, start_addr: u16) {
    print!("{:#06x}:\t", start_addr);

    for i in 0x00..0x0f {
        print!(" {:#04x}", cpu.read_u8_at(&(start_addr + i).into()));
    }

    print!("\n");
}

pub struct LoadProgramOptions {
    pub load_addr: u16,
    pub start_addr: u16,
    pub debug: bool,
}
