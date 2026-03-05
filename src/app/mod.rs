use std::sync::Arc;

use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use winit::{application::ApplicationHandler, event::WindowEvent, window::Window};

use crate::{
    ppu::{SCREEN_HEIGHT, SCREEN_WIDTH, rgb::Rgb},
    super_nintendo::SuperNintendo,
};

pub struct App {
    pub pixels: Option<Pixels<'static>>,
    pub super_nintendo: SuperNintendo,
    pub window: Option<Arc<Window>>,
}

impl App {
    pub fn new(super_nintendo: SuperNintendo) -> Self {
        Self {
            pixels: None,
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

        let surface_texture = SurfaceTexture::new(
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
            Arc::clone(&window),
        );
        let pixels = PixelsBuilder::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture)
            .build()
            .unwrap();

        self.window = Some(window);
        self.pixels = Some(pixels);
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
                for (pixel, chunk) in self.super_nintendo.frame_buffer().iter().zip(
                    self.pixels
                        .as_mut()
                        .unwrap()
                        .frame_mut()
                        .chunks_exact_mut(4),
                ) {
                    let rgb = Rgb(*pixel);
                    chunk[0] = (rgb.red() << 3) as u8;
                    chunk[1] = (rgb.green() << 3) as u8;
                    chunk[2] = (rgb.blue() << 3) as u8;
                    chunk[3] = 255;
                }

                self.pixels.as_mut().unwrap().render().unwrap();
            }
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
