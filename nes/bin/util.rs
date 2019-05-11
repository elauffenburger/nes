use std::io::{self, BufRead};

pub fn read_stdio_line() -> String {
    io::stdin().lock().lines().next().unwrap().unwrap()
}
