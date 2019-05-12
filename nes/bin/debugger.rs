use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;

use libnes::cpu::Cpu;

use crate::util::read_stdio_line;

pub fn start_debugger<'a>(cpu_ref: Rc<RefCell<Cpu>>)
{
    let mut cpu = cpu_ref.borrow_mut();

    println!("Starting debugger...");

    let mut always_print_status = false;

    while cpu.is_running() {
        if always_print_status {
            println!("{:?}", cpu);
        }

        print!("> ");
        io::stdout().flush().ok();

        let input = read_stdio_line();

        match input.as_str() {
            ".exit" => break,
            "r" => cpu.run(),
            "s" => cpu.step(),
            "p" => println!("{:?}", cpu),
            "p!" => always_print_status = !always_print_status,
            "h" => print_debugger_mode_help(),
            cmd @ _ => println!("Unrecognized command: {}\nEnter 'h' for help", cmd),
        }
    }
}

pub fn print_debugger_mode_help() {
    println!("Debugger mode help:");
    println!(".exit: exit debugging");
    println!("r: run to completion");
    println!("s: step");
    println!("p: print cpu status");
    println!("h: print help");
    println!("");
}
