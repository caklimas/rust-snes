use std::fs::OpenOptions;
use std::io::Write;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::SystemTime;

use softbuffer::{Context, Surface};
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use crate::{
    ppu::{SCREEN_HEIGHT, SCREEN_WIDTH, rgb::Rgb},
    super_nintendo::SuperNintendo,
};

pub struct App {
    pub paused: bool,
    pub surface: Option<Surface<Arc<Window>, Arc<Window>>>,
    pub super_nintendo: SuperNintendo,
    pub window: Option<Arc<Window>>,
}

impl App {
    pub fn new(super_nintendo: SuperNintendo) -> Self {
        Self {
            paused: false,
            surface: None,
            super_nintendo,
            window: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        let context = Context::new(Arc::clone(&window)).unwrap();
        let surface = Surface::new(&context, Arc::clone(&window)).unwrap();

        self.window = Some(window);
        self.surface = Some(surface);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                let window = self.window.as_ref().unwrap();
                let surface = self.surface.as_mut().unwrap();

                let width = window.inner_size().width;
                let height = window.inner_size().height;

                if width == 0 || height == 0 {
                    return;
                }

                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();

                let mut buffer = surface.buffer_mut().unwrap();

                let scale = (width / SCREEN_WIDTH as u32)
                    .min(height / SCREEN_HEIGHT as u32)
                    .max(1);

                let frame = self.super_nintendo.frame_buffer();

                for y in 0..SCREEN_HEIGHT as u32 {
                    for x in 0..SCREEN_WIDTH as u32 {
                        let pixel = frame[(y * SCREEN_WIDTH as u32 + x) as usize];
                        let rgb = Rgb(pixel);
                        let r = (rgb.red() << 3) as u32;
                        let g = (rgb.green() << 3) as u32;
                        let b = (rgb.blue() << 3) as u32;
                        let color = (r << 16) | (g << 8) | b;

                        for dy in 0..scale {
                            for dx in 0..scale {
                                let sx = x * scale + dx;
                                let sy = y * scale + dy;
                                if sx < width && sy < height {
                                    buffer[(sy * width + sx) as usize] = color;
                                }
                            }
                        }
                    }
                }

                buffer.present().unwrap();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key_code),
                        state,
                        ..
                    },
                ..
            } => match key_code {
                KeyCode::ArrowUp => self
                    .super_nintendo
                    .bus
                    .input_output
                    .controller_1
                    .set_up(state.is_pressed()),
                KeyCode::ArrowDown => self
                    .super_nintendo
                    .bus
                    .input_output
                    .controller_1
                    .set_down(state.is_pressed()),
                KeyCode::ArrowLeft => self
                    .super_nintendo
                    .bus
                    .input_output
                    .controller_1
                    .set_left(state.is_pressed()),
                KeyCode::ArrowRight => self
                    .super_nintendo
                    .bus
                    .input_output
                    .controller_1
                    .set_right(state.is_pressed()),
                KeyCode::KeyZ => self
                    .super_nintendo
                    .bus
                    .input_output
                    .controller_1
                    .set_b(state.is_pressed()),
                KeyCode::KeyX => self
                    .super_nintendo
                    .bus
                    .input_output
                    .controller_1
                    .set_a(state.is_pressed()),
                KeyCode::KeyA => self
                    .super_nintendo
                    .bus
                    .input_output
                    .controller_1
                    .set_y(state.is_pressed()),
                KeyCode::KeyS => self
                    .super_nintendo
                    .bus
                    .input_output
                    .controller_1
                    .set_x(state.is_pressed()),
                KeyCode::KeyQ => self
                    .super_nintendo
                    .bus
                    .input_output
                    .controller_1
                    .set_l(state.is_pressed()),
                KeyCode::KeyW => self
                    .super_nintendo
                    .bus
                    .input_output
                    .controller_1
                    .set_r(state.is_pressed()),
                KeyCode::Enter => self
                    .super_nintendo
                    .bus
                    .input_output
                    .controller_1
                    .set_start(state.is_pressed()),
                KeyCode::ShiftRight => self
                    .super_nintendo
                    .bus
                    .input_output
                    .controller_1
                    .set_select(state.is_pressed()),
                KeyCode::KeyD => {
                    if state.is_pressed() && self.paused {
                        let path = "docs/bugs/debug_dump.txt";
                        let timestamp = SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_millis();
                        let entry = format!(
                            "=== DEBUG DUMP @ {} ===\n{}\n\n",
                            timestamp,
                            self.super_nintendo.debug_info()
                        );
                        match OpenOptions::new().create(true).append(true).open(path) {
                            Ok(mut file) => {
                                let _ = file.write_all(entry.as_bytes());
                                eprintln!("Debug dump appended to {}", path);
                            }
                            Err(e) => eprintln!("Failed to write debug dump: {}", e),
                        }
                    }
                }
                KeyCode::KeyF => {
                    if state.is_pressed() && self.paused {
                        let timestamp = SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_millis();
                        let path = format!("docs/bugs/frame_{}.png", timestamp);
                        let frame = self.super_nintendo.frame_buffer();

                        let mut rgb_bytes = Vec::with_capacity(
                            (SCREEN_WIDTH as usize) * (SCREEN_HEIGHT as usize) * 3,
                        );
                        for &pixel in frame.iter() {
                            let rgb = Rgb(pixel);
                            rgb_bytes.push((rgb.red() << 3) as u8);
                            rgb_bytes.push((rgb.green() << 3) as u8);
                            rgb_bytes.push((rgb.blue() << 3) as u8);
                        }

                        let file = std::fs::File::create(&path).unwrap();
                        let writer = std::io::BufWriter::new(file);
                        let mut encoder =
                            png::Encoder::new(writer, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
                        encoder.set_color(png::ColorType::Rgb);
                        encoder.set_depth(png::BitDepth::Eight);
                        let mut png_writer = encoder.write_header().unwrap();
                        png_writer.write_image_data(&rgb_bytes).unwrap();

                        eprintln!("Frame dumped to {}", path);
                    }
                }
                KeyCode::KeyP => {
                    if state.is_pressed() {
                        self.paused = !self.paused;
                    }
                }
                KeyCode::Digit1 => {
                    if state.is_pressed() {
                        self.super_nintendo.bus.ppu.debug_disabled_layers ^= 0x01;
                        eprintln!(
                            "debug_disabled_layers = 0x{:02X}",
                            self.super_nintendo.bus.ppu.debug_disabled_layers
                        );
                    }
                }
                KeyCode::Digit2 => {
                    if state.is_pressed() {
                        self.super_nintendo.bus.ppu.debug_disabled_layers ^= 0x02;
                        eprintln!(
                            "debug_disabled_layers = 0x{:02X}",
                            self.super_nintendo.bus.ppu.debug_disabled_layers
                        );
                    }
                }
                KeyCode::Digit3 => {
                    if state.is_pressed() {
                        self.super_nintendo.bus.ppu.debug_disabled_layers ^= 0x04;
                        eprintln!(
                            "debug_disabled_layers = 0x{:02X}",
                            self.super_nintendo.bus.ppu.debug_disabled_layers
                        );
                    }
                }
                KeyCode::Digit4 => {
                    if state.is_pressed() {
                        self.super_nintendo.bus.ppu.debug_disabled_layers ^= 0x10;
                        eprintln!(
                            "debug_disabled_layers = 0x{:02X}",
                            self.super_nintendo.bus.ppu.debug_disabled_layers
                        );
                    }
                }
                KeyCode::KeyM => {
                    if state.is_pressed() {
                        self.super_nintendo.bus.ppu.debug_disabled_layers ^= 0x20;
                        eprintln!(
                            "debug_disabled_layers = 0x{:02X} (bit 5 = color math)",
                            self.super_nintendo.bus.ppu.debug_disabled_layers
                        );
                    }
                }
                _ => {}
            },
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _: &winit::event_loop::ActiveEventLoop) {
        if !self.paused {
            loop {
                self.super_nintendo.step();
                if self.super_nintendo.frame_complete() {
                    break;
                }
            }
        }

        self.window.as_ref().unwrap().request_redraw();
    }
}
