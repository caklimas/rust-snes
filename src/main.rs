mod cpu;

fn main() {
    let cpu = cpu::Cpu::default();

    println!("{cpu:?}");
}
