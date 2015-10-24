use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const WIN_WIDTH: u32 = 1024;
const WIN_HEIGHT: u32 = 768;

pub struct Visualizer<'a> {
    sdl_ctx: sdl2::Sdl,
    sdl_renderer: sdl2::render::Renderer<'a>,
}

impl<'a> Visualizer<'a> {
    pub fn new() -> Visualizer<'a> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("FART", WIN_WIDTH, WIN_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut renderer = window.renderer().present_vsync().accelerated().build().unwrap();
//      let mut renderer = window.renderer().build().unwrap();

        let mut ret = Visualizer {
            sdl_ctx: sdl_context,
            sdl_renderer: renderer,
        };

        ret
    }

    pub fn draw_hist(&mut self, freqs: &[f64]) {
        let height_offset = (WIN_HEIGHT as f64 / 2.0);
//      let scale_factor: f64 = height_offset / 512.0;
        let scale_factor: f64 = height_offset / 32768.0;

        self.sdl_renderer.set_draw_color(Color::RGB(0, 0, 0));
        self.sdl_renderer.clear();
        self.sdl_renderer.set_draw_color(Color::RGB(255, 255, 255));

        for (i, &freq) in freqs.iter().enumerate() {
//          let x = i as i32;
//          let y = height_offset as i32;
//          let width = 1;
//          let height = 1 + (scale_factor * freq) as u32;

            let x: i32 = i as i32;
            let y: i32;
            let width: u32 = 1;
            let height: u32 = 1 + (scale_factor * freq.abs()) as u32;

            if freq < 0.0 {
                y = (height_offset as i32) - height as i32;
            } else {
                y = height_offset as i32;
            }

//          println!("freq: {}, x: {}, y: {}, h: {}, w: {}", freq, x, y, height, width);
            let rect = Rect::new(x, y, width, height).unwrap().unwrap();
            self.sdl_renderer.fill_rect(rect);
        }

        self.sdl_renderer.present();
    }
}
