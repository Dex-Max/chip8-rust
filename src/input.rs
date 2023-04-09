use sdl2::Sdl;
use sdl2::EventPump;

pub struct Input {
    event_pump: EventPump
}

impl Input {
    pub fn new(sdl: &Sdl) -> Input {
        let event_pump = sdl.event_pump().unwrap();
    }
}
