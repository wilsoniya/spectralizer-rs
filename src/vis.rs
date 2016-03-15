//! Sequence visualizer. Meant for visualzing freq domain sequences, but will
//! just as hapily draw time domain or any other sequential stuff.

use std::process::exit;

use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::{Event, WindowEventId};
use sdl2::keyboard::Keycode;

/// Visualizer
pub struct Visualizer<'a> {
    sdl_renderer: sdl2::render::Renderer<'a>,
    sdl_event_pump: sdl2::EventPump,
    win_width: u32,
    win_height: u32,
    scale_x: f32,
    scale_y: f32,
}

impl<'a> Visualizer<'a> {
    /// Creates a new visualizer.
    pub fn new(win_name: &str, win_width: u32, win_height: u32) -> Visualizer<'a> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window(win_name, win_width, win_height)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut renderer = window.renderer().present_vsync().accelerated().build().unwrap();
        renderer.set_blend_mode(sdl2::render::BlendMode::Blend);

        let ret = Visualizer {
            sdl_renderer: renderer,
            sdl_event_pump: sdl_context.event_pump().unwrap(),
            win_width: win_width,
            win_height: win_height,
            scale_x: 1.0,
            scale_y: 1.0,
        };

        ret
    }

    /// Draws a histogram using *freqs*.
    pub fn draw_hist(&mut self, freqs: &[f64]) {
        let height_offset = self.win_height as f64;
        let scale_factor: f64 = height_offset / 32768.0;

        self.sdl_renderer.set_scale(self.scale_x, self.scale_y);

        self.sdl_renderer.set_draw_color(Color::RGBA(0, 0, 0, 60));
        self.sdl_renderer.fill_rect(Rect::new_unwrap(0, 0, self.win_width,
                                                     self.win_height));
        self.sdl_renderer.set_draw_color(Color::RGB(255, 255, 255));

        let width: u32 = if self.win_width >= freqs.len() as u32 {
            self.win_width / freqs.len() as u32
        } else {
            1
        };

        for (i, &freq) in freqs.iter().enumerate() {
            let _freq = freq * -1.0;
            let x: i32 = width as i32 * i as i32;
            let y: i32;
            let height: u32 = 1 + (scale_factor * _freq.abs()) as u32;

            if _freq < 0.0 {
                y = (height_offset as i32) - height as i32;
            } else {
                y = height_offset as i32;
            }

            let rect = Rect::new_unwrap(x, y, width, height);
            self.sdl_renderer.fill_rect(rect);
        }

        self.sdl_renderer.present();
    }

    /// Handles UI events
    ///
    /// At the moment this just means listens for the user pressing escape or
    /// closing the window.
    pub fn handle_events(&mut self) {
        for event in self.sdl_event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    exit(0);
                },
                Event::Window { win_event_id: WindowEventId::Resized,
                    data1: x, data2: y, .. } => {
                        self.scale_x = x as f32 / self.win_width as f32;
                        self.scale_y = y as f32 / self.win_height as f32;
                }
                _ => {}
            }
        }
    }
}
