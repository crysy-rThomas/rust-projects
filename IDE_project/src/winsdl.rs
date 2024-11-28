use sdl2::{Sdl, video::Window, EventPump};

#[allow(dead_code)]
pub struct Winsdl {
    pub sdl: Sdl,
    pub window: Window,
    pub event_pump: EventPump,

}

impl Winsdl{
    pub fn new(width: usize, height: usize) -> Result<Self, &'static str> {
        let sdl:  Sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let window = video_subsystem
            .window("My window", width as u32, height as u32)
            .build()
            .unwrap();

        let event_pump: sdl2::EventPump = sdl.event_pump().unwrap();

        Ok(Winsdl {sdl, window, event_pump})
    }
}