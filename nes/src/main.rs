extern crate clap;
extern crate libnes;

use std::io::BufRead;
use std::io::{self, Read, Write};

use clap::{App, Arg, ArgMatches, SubCommand};
use libnes::cpu::helpers::load_program_str;

use libnes::cpu::Cpu;

fn main() {
    let app = get_cli_app();
    let app_matches = app.get_matches();

    match app_matches.subcommand() {
        ("cpu", Some(options)) => {
            run_command_cpu(options);
        }
        ("", _) => println!("{}", app_matches.usage.unwrap()),
        (command @ _, _) => panic!("Command {} not implemented!", command),
    }
}

fn run_command_cpu<'a>(options: &ArgMatches<'a>) {
    let debug_mode = options.is_present("debug");
    let break_on_entry = options.is_present("break");

    let mut cpu = Cpu::new(debug_mode);

    match options.value_of("file") {
        Some(file) => panic!("file input not implemented!"),
        None => {
            let program = options
                .value_of("program")
                .expect("program arg should be provided if file arg is not");

            load_program_str(&mut cpu, program);

            match break_on_entry {
                true => run_debugger_mode(&mut cpu),
                false => cpu.run(),
            };

            println!("Done");
        }
    }
}

fn run_debugger_mode(cpu: &mut Cpu) {
    println!("Starting debugger...");

    cpu.startup();

    println!("Debugger started");

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
        .subcommand(
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
        )
}

#[cfg(test)]
mod test {
    #[test]
    fn can_init() {}
}
