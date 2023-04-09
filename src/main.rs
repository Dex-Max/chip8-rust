mod chip8;
mod display;

use std::env;
use std::fs;
use std::time::Instant;
use chip8::Chip8;
use chip8::State;
use display::Display;

fn main() {
    let args: Vec<String> = env::args().collect();

    let bytes = match fs::read(&args[1]) {
        Ok(t) => { t }
        Err(_) => { panic!("File not found!") }
    };

    let mut display = Display::new();
    let mut processor = Chip8::new();
    let mut num_cycles = 0;

    processor.load_program(bytes);

    let mut tick_time = Instant::now();
    loop {
        if num_cycles > 16 {
            num_cycles = 0;
            if processor.dt > 0 { processor.dt -= 1 }
            if processor.st > 0 { processor.st -= 1 }

            while tick_time.elapsed().as_nanos() < 16666 {}

            tick_time = Instant::now();
        }

        let state = processor.cycle();
        num_cycles += 1;

        match state {
            State::Run => { continue }
            State::Quit => { break }
            State:: Draw => {
                display.draw(&processor.display);
            }
        }
    }
}
