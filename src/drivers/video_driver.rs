use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::drivers::configs::defaults::*;
pub struct VideoDriver {
    canvas: Canvas<Window>,
}

impl VideoDriver {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys
            .window(
                "rust-sdl2_gfx: draw line & FPSManager",
                SDL_WIDTH,
                SDL_HEIGHT,
            )
            .position_centered()
            .opengl()
            .build()
            .unwrap();
            
            let mut canvas = window.into_canvas().build().unwrap();

            canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
            canvas.clear();
            canvas.present();
    
            VideoDriver { canvas: canvas }
    }

    pub fn draw(&mut self, pixels: &[[u8; 64]; 32]) {
        for (y, row) in pixels.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x as u32) * SCALE_FACTOR;
                let y = (y as u32) * SCALE_FACTOR;

                self.canvas.set_draw_color(color(col));
                let _ = self.canvas
                    .fill_rect(Rect::new(x as i32, y as i32, SCALE_FACTOR, SCALE_FACTOR));
            }
        }
        self.canvas.present();
    }



}

fn color(value: u8) -> pixels::Color {
    if value == 0 {
        pixels::Color::RGB(0, 0, 0)
    } else {
        pixels::Color::RGB(0, 250, 0)
    }
}