extern crate reqwest;

use std::fs::{File, DirBuilder, self};
use std::io::Write;
use std::path::Path;

const klaus_functional_tests_url: &'static str = "https://github.com/Klaus2m5/6502_65C02_functional_tests/blob/master/bin_files/6502_functional_test.bin?raw=true";
const nestest_url: &'static str = "http://nickmass.com/images/nestest.nes";

fn main() {
    generate_test_files();
}

fn generate_test_files() {
    create_dir_if_not_exists("./test");

    generate_klaus_tests();
    generate_nestest_tests();
}

fn generate_klaus_tests() {
    create_dir_if_not_exists("./test/klaus");

    create_file_if_not_exists("./test/klaus/functional.bin", |file: &mut File| {
        reqwest::get(klaus_functional_tests_url)
            .expect("expected to fetch functional tests content")
            .copy_to(file)
            .unwrap();
    });
}

fn generate_nestest_tests() {
    create_dir_if_not_exists("./test/nes");

    create_file_if_not_exists("./test/nes/nestest.nes", |file: &mut File| {
        reqwest::get(nestest_url)
            .expect("expected to fetch nes test content")
            .copy_to(file)
            .unwrap();
    });
}

fn create_dir_if_not_exists<'a>(path: &'a str) {
    let dir_path = Path::new(path);
    if !dir_path.exists() {
        DirBuilder::new()
            .recursive(true)
            .create(dir_path)
            .unwrap();
    }
}

fn create_file_if_not_exists<'a, F>(path: &'a str, if_not_exists: F) where F: FnOnce(&mut File) {
    let file_path = Path::new(path);
    if !file_path.exists() {
        let file = &mut File::create(file_path).expect(format!("expected to create file: '{:?}'", path).as_str());

        if_not_exists(file);
    }
}