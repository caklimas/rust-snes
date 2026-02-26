use rust_snes::super_nintendo::SuperNintendo;
use std::fmt::Error;

fn main() {
    let mut snes = SuperNintendo::new(vec![0; 100]);
    let _do_steps = || -> Result<(), Error> {
        snes.step();
        Ok(())
    };

    println!("Loaded");
}
