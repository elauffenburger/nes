use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

use crate::cpu::helpers::*;
use crate::cpu::{Cpu, DefaultCpu};

#[test]
fn basic_program() {
    let mut cpu = DefaultCpu::new(true);

    load_program_str(&mut cpu, "a9 01 8d 00 02 a9 05 8d 01 02 a9 08 8d 02 02");

    cpu.run();

    assert_eq!(cpu.registers.acc, 0x08);
}

#[test]
fn lda_tax_inx_adc() {
    let mut cpu = DefaultCpu::new(true);

    load_program_str(&mut cpu, "a9 c0 aa e8 69 c4 00");

    cpu.run();

    assert_eq!(cpu.registers.acc as u8, 0x84);
    assert_eq!(cpu.registers.x as u8, 0xc1);
}

#[test]
fn ldx_dex_stx_cpx_bnx() {
    let mut cpu = DefaultCpu::new(true);

    load_program_str(&mut cpu, "a2 08 ca 8e 00 02 e0 03 d0 f8 8e 01 02 00");

    cpu.run();

    assert_eq!(cpu.registers.acc as u8, 0x00);
    assert_eq!(cpu.registers.x as u8, 0x03);
}

#[test]
fn lda_cmp_bne_sta_brk() {
    let mut cpu = DefaultCpu::new(true);

    load_program_str(&mut cpu, "a9 01 c9 02 d0 02 85 22 00");

    cpu.run();

    assert_eq!(cpu.registers.acc as u8, 0x01);
    assert_eq!(cpu.registers.p.into_u8(), 0b10110100);
}

#[test]
fn lda_sta_lda_sta_jmp() {
    let mut cpu = DefaultCpu::new(true);

    load_program_str(&mut cpu, "a9 01 85 f0 a9 cc 85 f1 6c f0 00");

    cpu.run();

    assert_eq!(cpu.registers.acc as u8, 0xcc);
    assert_eq!(cpu.registers.p.into_u8(), 0b10110100);
}

#[test]
fn jsr_lda_rts() {
    let mut cpu = DefaultCpu::new(true);

    load_program_str(&mut cpu, "a9 01 20 08 06 a9 03 00 a9 02 60");

    cpu.run();

    assert_eq!(cpu.registers.acc as u8, 0x03);
}

#[test]
fn rol_acc() {
    let mut cpu = DefaultCpu::new(true);

    load_program_str(&mut cpu, "a9 81 2a");

    cpu.run();

    assert_eq!(cpu.registers.acc as u8, 0x02);
}

#[test]
fn ror_acc() {
    let mut cpu = DefaultCpu::new(true);

    load_program_str(&mut cpu, "a9 81 6a");

    cpu.run();

    assert_eq!(cpu.registers.acc as u8, 0x40);
}

#[test]
fn rol_mem() {
    let mut cpu = DefaultCpu::new(true);

    load_program_str(&mut cpu, "a9 81 8d 00 80 2e 00 80");

    cpu.run();

    assert_eq!(cpu.registers.acc as u8, 0x81);
    assert_eq!(cpu.read_u8_at(&0x8000u16.into()), 0x02);
}

#[test]
fn ror_mem() {
    let mut cpu = DefaultCpu::new(true);

    load_program_str(&mut cpu, "a9 81 8d 00 80 6e 00 80");

    cpu.run();

    assert_eq!(cpu.registers.acc as u8, 0x81);
    assert_eq!(cpu.read_u8_at(&0x8000u16.into()), 0x40);
}

#[test]
#[ignore]
fn snake() {
    let mut cpu = DefaultCpu::new(true);

    let prog_parts = vec![
        "20 06 06 20 38 06 20 0d 06 20 2a 06 60 a9 02 85",
        "02 a9 04 85 03 a9 11 85 10 a9 10 85 12 a9 0f 85",
        "14 a9 04 85 11 85 13 85 15 60 a5 fe 85 00 a5 fe",
        "29 03 18 69 02 85 01 60 20 4d 06 20 8d 06 20 c3",
        "06 20 19 07 20 20 07 20 2d 07 4c 38 06 a5 ff c9",
        "77 f0 0d c9 64 f0 14 c9 73 f0 1b c9 61 f0 22 60",
        "a9 04 24 02 d0 26 a9 01 85 02 60 a9 08 24 02 d0",
        "1b a9 02 85 02 60 a9 01 24 02 d0 10 a9 04 85 02",
        "60 a9 02 24 02 d0 05 a9 08 85 02 60 60 20 94 06",
        "20 a8 06 60 a5 00 c5 10 d0 0d a5 01 c5 11 d0 07",
        "e6 03 e6 03 20 2a 06 60 a2 02 b5 10 c5 10 d0 06",
        "b5 11 c5 11 f0 09 e8 e8 e4 03 f0 06 4c aa 06 4c",
        "35 07 60 a6 03 ca 8a b5 10 95 12 ca 10 f9 a5 02",
        "4a b0 09 4a b0 19 4a b0 1f 4a b0 2f a5 10 38 e9",
        "20 85 10 90 01 60 c6 11 a9 01 c5 11 f0 28 60 e6",
        "10 a9 1f 24 10 f0 1f 60 a5 10 18 69 20 85 10 b0",
        "01 60 e6 11 a9 06 c5 11 f0 0c 60 c6 10 a5 10 29",
        "1f c9 1f f0 01 60 4c 35 07 a0 00 a5 fe 91 00 60",
        "a6 03 a9 00 81 10 a2 00 a9 01 81 10 60 a2 00 ea",
        "ea ca d0 fb 60",
    ];
    let prog = prog_parts.join(" ");

    load_program_string(&mut cpu, prog);

    cpu.run();

    assert_eq!(cpu.registers.acc as u8, 0x81);
    assert_eq!(cpu.read_u8_at(&0x8000u16.into()), 0x40);
}

#[test]
#[ignore]
fn klaus() {
    let mut klaus_prog_file = File::open(Path::new("./test/klaus/functional.bin")).unwrap();
    let mut klaus_prog_vec = vec![];

    klaus_prog_file.read_to_end(&mut klaus_prog_vec).unwrap();

    timeout_test(Box::new(move || {
        let mut cpu = DefaultCpu::new(true);
        load_program_bytes_with_options(
            &mut cpu,
            &klaus_prog_vec,
            LoadProgramOptions {
                load_addr: 0x0000,
                start_addr: 0x0400,
                debug: true,
            },
        );

        cpu.run();
    }));

    // fail the test to get output
    assert!(false);
}

fn timeout_test<F>(test: Box<F>)
where
    F: 'static + std::marker::Send + FnOnce() -> (),
{
    let (send, recv) = channel();
    std::thread::spawn(move || {
        test();
        send.send("done!").unwrap();
    });

    let result = recv.recv_timeout(Duration::from_millis(1000));
    result.unwrap();
}