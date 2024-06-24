use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut line = String::new();
    for arg in env::args().skip(1) {
        match File::open(arg) {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut count = 0;
                while reader.read_line(&mut line).unwrap_or(0) > 0 {
                    count += 1;
                    line.clear();
                }
                println!("{count}");
            },
            Err(_) => ()
        }
    }
}
