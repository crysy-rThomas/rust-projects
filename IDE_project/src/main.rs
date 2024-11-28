mod winsdl;
use crate::winsdl::Winsdl;
use sdl2::event::Event;

fn main() {
    let mut winsdl = Winsdl::new(800, 600).unwrap();
    'running: loop {
        for event in winsdl.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'running,
                _ => {}
            }
        }
    }
}