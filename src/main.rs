mod chip8;
mod display;

use std::env;
use std::fs;
use chip8::Chip8;
use chip8::State;
use display::Display;

fn main() {
    let args: Vec<String> = env::args().collect();

    let bytes = match fs::read(&args[1]) {
        Ok(t) => { t }
        Err(_) => { panic!("File not found!") }
    };

    let mut processor = Chip8::new();
    processor.load_program(bytes);

    let mut display = Display::new();

    loop {
        let state = processor.cycle();

        match state {
            State::Run => { continue }
            State::Quit => { break }
            State:: Draw => {
                display.draw(&processor.display);
            }
        }
    }
}
