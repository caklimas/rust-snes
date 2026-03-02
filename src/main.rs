use rust_snes::super_nintendo::SuperNintendo;
use std::{env, fs};

fn main() {
    let path = env::args().nth(1).expect("Usage: rust-snes <rom.smc>");

    if !path.ends_with(".smc") && !path.ends_with(".sfc") {
        eprintln!("Error: expected a .smc or .sfc file");
        std::process::exit(1);
    }

    let data = fs::read(path).expect("Failed to read ROM file");
    let mut snes = SuperNintendo::new(data);

    loop {
        snes.step();
    }
}
