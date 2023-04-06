mod chip8;

use std::env;
use chip8::Chip8;

fn main() {
    let args: Vec<String> = env::args().collect();

    let processor = Chip8::new();

}
