use sdl2;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const SCALE: u32 = 10;
const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;

pub struct Display {
    canvas: Canvas<Window> 
}

impl Display {
    pub fn new() -> Display {
        let video = sdl2::init().unwrap().video().unwrap();
        let window = video.window("CHIP-8", WIDTH * SCALE, HEIGHT * SCALE).opengl().build().unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();


        Display { canvas }
    }

    pub fn draw(&mut self, pixels: &[bool]) {
        let mut rect = Rect::new(0, 0, SCALE, SCALE);
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if pixels[(y * WIDTH + x) as usize] {
                    rect.set_x((x * SCALE) as i32);
                    rect.set_y((y * SCALE) as i32);

                    self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                    self.canvas.fill_rect(rect);
                    self.canvas.draw_rect(rect);
                }
            }
        }

        self.canvas.present();
    }
}
