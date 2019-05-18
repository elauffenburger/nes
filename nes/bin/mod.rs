extern crate clap;
extern crate glutin_window;
extern crate graphics;
extern crate libnes;
extern crate opengl_graphics;
extern crate piston;

mod debugger;
mod gui;
pub mod util;

use libnes::cpu::mem::CpuMemoryAccessEvent;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

use clap::{App, Arg, ArgMatches, SubCommand};

use libnes::cart::{get_cart_loader, CartLoader, RomFormat};
use libnes::cpu::helpers::load_program_str;
use libnes::cpu::{Cpu, DefaultCpu};
use libnes::nes::{DefaultNes, Nes};
use libnes::ppu::{DefaultPpu, Ppu};
use libnes::util::rc_ref;

use debugger::start_debugger;
use gui::start_gui;

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
                true => start_debugger(rc_ref(cpu)),
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
    let gui = options.value_of("gui");

    let rom_format = match rom_format_str {
        "ines" => RomFormat::iNes,
        _ => panic!(format!("Unsupported rom format '{}'", rom_format_str)),
    };

    let cpu = rc_ref(DefaultCpu::new(debug));
    let ppu = rc_ref(DefaultPpu::new());

    let nes = rc_ref(DefaultNes::new(cpu, ppu));

    let filename = options
        .value_of("file")
        .expect("File parameter is required");

    let cart_data = fs::read(filename).expect(&format!("Failed to read file {}", filename));

    let cart_loader = get_cart_loader(rom_format).expect(&format!(
        "Failed to resolve loader for rom format '{}'",
        rom_format_str
    ));

    cart_loader
        .load(nes.clone(), &cart_data)
        .expect("Failed to load rom");

    let cpu: Rc<RefCell<Cpu>> = nes.clone().borrow_mut().get_cpu().clone();
    cpu.borrow_mut().start();

    if let Some(addr) = start_addr {
        cpu.borrow_mut().get_registers_mut().pc = u16::from_str_radix(addr, 16).expect(&format!(
            "Failed to parse starting address '{}' as a hexadecimal u16 value",
            addr
        ));
    }

    match gui {
        Some("false") => {
            match break_mode {
                true => start_debugger(cpu),
                false => {
                    let mut nes = nes.borrow_mut();
                    let ppu = nes.get_ppu();

                    loop {
                        let nametable = ppu.borrow().get_active_nametable();
                        let pattern_table = ppu.borrow_mut().get_active_pattern_table();
                        // println!("nametable:\n{:?}\n", &nametable);

                        nes.tick();
                    }
                }
            };
        }
        _ => {
            start_gui(nes);
        }
    }
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
                        .required_unless("gui"),
                    Arg::with_name("gui")
                        .short("g")
                        .long("gui")
                        .default_value("true"),
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
