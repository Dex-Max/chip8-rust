mod chip8;
mod display;

use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::KeyboardState;
use std::env;
use std::fs;
use std::time::Instant;
use chip8::Chip8;
use chip8::State;
use display::Display;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return eprintln!("Usage: cargo run <PATH-TO-ROM>");
    }
    let bytes = match fs::read(&args[1]) {
        Ok(t) => { t }
        Err(_) => { return eprintln!("File {} not found!", &args[1]) }
    };

    let sdl = sdl2::init().unwrap();
    let mut display = Display::new(&sdl);
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

        let mut event_pump = sdl.event_pump().unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { println!("Quit"); }
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Q => {  }
                        Keycode::W => { println!("W"); }
                        Keycode::E => { println!("E"); }
                        Keycode::R => { println!("R"); }
                        _ => {}
                    }
                }

                _ => {}

            }
        }
        let mut keyboard = [false; 16];
        let keyboard_state = KeyboardState::new(&mut event_pump);
        keyboard[0] = keyboard_state.is_scancode_pressed(Scancode::X);
        keyboard[1] = keyboard_state.is_scancode_pressed(Scancode::Num1);
        keyboard[2] = keyboard_state.is_scancode_pressed(Scancode::Num2);
        keyboard[3] = keyboard_state.is_scancode_pressed(Scancode::Num3);
        keyboard[4] = keyboard_state.is_scancode_pressed(Scancode::Q);
        keyboard[5] = keyboard_state.is_scancode_pressed(Scancode::W);
        keyboard[6] = keyboard_state.is_scancode_pressed(Scancode::E);
        keyboard[7] = keyboard_state.is_scancode_pressed(Scancode::A);
        keyboard[8] = keyboard_state.is_scancode_pressed(Scancode::S);
        keyboard[9] = keyboard_state.is_scancode_pressed(Scancode::D);
        keyboard[0xA] = keyboard_state.is_scancode_pressed(Scancode::Z);
        keyboard[0xB] = keyboard_state.is_scancode_pressed(Scancode::C);
        keyboard[0xC] = keyboard_state.is_scancode_pressed(Scancode::Num4);
        keyboard[0xD] = keyboard_state.is_scancode_pressed(Scancode::R);
        keyboard[0xE] = keyboard_state.is_scancode_pressed(Scancode::F);
        keyboard[0xF] = keyboard_state.is_scancode_pressed(Scancode::V);

        let state = processor.cycle(keyboard);
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
