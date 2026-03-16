use std::num::NonZeroU32;
use std::sync::Arc;

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
    pub surface: Option<Surface<Arc<Window>, Arc<Window>>>,
    pub super_nintendo: SuperNintendo,
    pub window: Option<Arc<Window>>,
}

impl App {
    pub fn new(super_nintendo: SuperNintendo) -> Self {
        Self {
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
                _ => {}
            },
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _: &winit::event_loop::ActiveEventLoop) {
        loop {
            self.super_nintendo.step();
            if self.super_nintendo.frame_complete() {
                break;
            }
        }

        self.window.as_ref().unwrap().request_redraw();
    }
}
