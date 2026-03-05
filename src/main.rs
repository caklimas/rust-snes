use rust_snes::{app::App, super_nintendo::SuperNintendo};
use std::{env, fs};
use winit::event_loop::EventLoop;

fn main() {
    let path = env::args().nth(1).expect("Usage: rust-snes <rom.smc>");

    if !path.ends_with(".smc") && !path.ends_with(".sfc") {
        eprintln!("Error: expected a .smc or .sfc file");
        std::process::exit(1);
    }

    let event_loop = EventLoop::new().unwrap();
    let data = fs::read(path).expect("Failed to read ROM file");
    let snes = SuperNintendo::new(data);
    let mut app = App::new(snes);

    event_loop.run_app(&mut app).unwrap();
}
