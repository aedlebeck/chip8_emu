use sdl2;
use sdl2::event::{EventPollIterator};

pub struct InputDriver {
    events: sdl2::EventPump,
}

impl InputDriver {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        InputDriver {
            events: sdl_context.event_pump().unwrap(),
        }
    }

    pub fn poll(&mut self) -> EventPollIterator {
        return self.events.poll_iter();
    }

}
