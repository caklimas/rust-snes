mod cpu;
mod memory;

fn main() {
    let cpu = cpu::Cpu::default();

    println!("{cpu:?}");
}
