extern crate clap;
extern crate libnes;

use std::fmt::Debug;
use std::fs;
use std::io::{self, Write, BufRead};

use clap::{App, Arg, ArgMatches, SubCommand};

use libnes::cart::{get_cart_loader, CartLoader, RomFormat};
use libnes::cpu::helpers::load_program_str;
use libnes::cpu::{Cpu, DefaultCpu};

fn main() {
    let app = get_cli_app();
    let app_matches = app.get_matches();

    match app_matches.subcommand() {
        ("cpu", Some(options)) => {
            exec_command_cpu(options);
        }
        ("run", Some(options)) => exec_command_run(options),
        ("", _) => println!("{}", app_matches.usage.unwrap()),
        (command @ _, _) => panic!("Command {} not implemented!", command),
    }
}

fn exec_command_cpu<'a>(options: &ArgMatches<'a>) {
    let debug_mode = options.is_present("debug");
    let break_on_entry = options.is_present("break");

    let mut cpu = DefaultCpu::new(debug_mode);

    match options.value_of("file") {
        Some(_) => panic!("file input not implemented!"),
        None => {
            let program = options
                .value_of("program")
                .expect("program arg should be provided if file arg is not");

            load_program_str(&mut cpu, program);

            cpu.start();

            match break_on_entry {
                true => start_debugger(&mut cpu),
                false => cpu.run(),
            };

            println!("Done");
        }
    }
}

fn exec_command_run<'a>(options: &ArgMatches<'a>) {
    let debug = options.is_present("debug");
    let break_mode = options.is_present("break");
    let rom_format_str = options.value_of("format").expect("format is required");
    let start_addr = options.value_of("startaddr");

    let rom_format = match rom_format_str {
        "ines" => RomFormat::iNes,
        _ => panic!(format!("Unsupported rom format '{}'", rom_format_str)),
    };

    let filename = options
        .value_of("file")
        .expect("File parameter is required");

    let cart_data = fs::read(filename).expect(&format!("Failed to read file {}", filename));

    let mut cpu = DefaultCpu::new(debug);

    let cart_loader = get_cart_loader(rom_format).expect(&format!(
        "Failed to resolve loader for rom format '{}'",
        rom_format_str
    ));

    cart_loader
        .load(&mut cpu, &cart_data)
        .expect("Failed to load rom");

    cpu.start();

    if let Some(addr) = start_addr {
        cpu.registers.pc = u16::from_str_radix(addr, 16).expect(&format!(
            "Failed to parse starting address '{}' as a hexadecimal u16 value",
            addr
        ));
    }

    match break_mode {
        true => start_debugger(&mut cpu),
        false => cpu.run(),
    };
}

fn start_debugger<'a, T>(cpu: &'a mut T)
where
    T: Cpu + Debug,
{
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

fn print_debugger_mode_help() {
    println!("Debugger mode help:");
    println!(".exit: exit debugging");
    println!("r: run to completion");
    println!("s: step");
    println!("p: print cpu status");
    println!("h: print help");
    println!("");
}

fn read_stdio_line() -> String {
    io::stdin().lock().lines().next().unwrap().unwrap()
}

fn get_cli_app<'a, 'b>() -> App<'a, 'b> {
    App::new("nes")
        .version("0.1")
        .author("Eric L. <elauffenburger@gmail.com>")
        .about("An NES emulator")
        .subcommands(vec![
            SubCommand::with_name("cpu")
                .about("Enters cpu-only emulation mode")
                .args(&[
                    Arg::with_name("debug")
                        .short("d")
                        .long("debug")
                        .takes_value(false),
                    Arg::with_name("break")
                        .short("b")
                        .long("break")
                        .takes_value(false),
                    Arg::with_name("file")
                        .short("f")
                        .long("file")
                        .value_name("FILE")
                        .required_unless("program"),
                    Arg::with_name("program")
                        .short("p")
                        .long("program")
                        .value_name("PROGRAM")
                        .required_unless("file"),
                ]),
            SubCommand::with_name("run")
                .about("Runs nes in full emulation mode")
                .args(&[
                    Arg::with_name("debug")
                        .short("d")
                        .long("debug")
                        .takes_value(false),
                    Arg::with_name("break")
                        .short("b")
                        .long("break")
                        .takes_value(false),
                    Arg::with_name("file")
                        .short("f")
                        .long("file")
                        .value_name("FILE")
                        .required(true),
                    Arg::with_name("format")
                        .long("format")
                        .value_name("FORMAT")
                        .default_value("ines"),
                    Arg::with_name("startaddr")
                        .long("start-addr")
                        .value_name("START_ADDRESS"),
                ]),
        ])
}

#[cfg(test)]
mod test {
    #[test]
    fn can_init() {}
}
