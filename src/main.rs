use crate::super_nintendo::SuperNintendo;

mod cpu;
mod memory;
mod super_nintendo;

fn main() {
    let snes = SuperNintendo::new(vec![0; 100]);

    println!("Loaded");
}
