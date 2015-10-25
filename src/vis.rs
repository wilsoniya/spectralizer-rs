//! Sequence visualizer. Meant for visualzing freq domain sequences, but will
//! just as hapily draw time domain or any other sequential stuff.

use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

//const WIN_WIDTH: u32 = 512;
//const WIN_HEIGHT: u32 = 384;
const WIN_WIDTH: u32 = 256;
const WIN_HEIGHT: u32 = 192;

/// Visualizer
pub struct Visualizer<'a> {
    sdl_renderer: sdl2::render::Renderer<'a>,
}

impl<'a> Visualizer<'a> {
    /// Creates a new visualizer.
    pub fn new() -> Visualizer<'a> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("FART", WIN_WIDTH, WIN_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let renderer = window.renderer().present_vsync().accelerated().build().unwrap();

        let ret = Visualizer {
            sdl_renderer: renderer,
        };

        ret
    }

    /// Draws a histogram using *freqs*.
    pub fn draw_hist(&mut self, freqs: &[f64]) {
        let height_offset = WIN_HEIGHT as f64;
        let scale_factor: f64 = height_offset / 32768.0;
//      let scale_factor: f64 = 10.0;

        self.sdl_renderer.set_draw_color(Color::RGB(0, 0, 0));
        self.sdl_renderer.clear();
        self.sdl_renderer.set_draw_color(Color::RGB(255, 255, 255));

        let width: u32 = WIN_WIDTH / freqs.len() as u32;

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

            let rect = Rect::new(x, y, width, height).unwrap().unwrap();
            self.sdl_renderer.fill_rect(rect);
        }

        self.sdl_renderer.present();
    }
}
