use std::fmt::Error;

use crate::super_nintendo::SuperNintendo;

mod cpu;
mod memory;
mod super_nintendo;

fn main() {
    let mut snes = SuperNintendo::new(vec![0; 100]);
    let do_steps = || -> Result<(), Error> {
        snes.step();
        Ok(())
    };

    println!("Loaded");
}
