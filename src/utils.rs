use sdl2::{event::Event, keyboard::Keycode};

pub fn keyboard_pressed(event: &Event, key: Keycode) -> bool {
    matches!(event, Event::KeyDown { keycode: Some(k), .. } if *k == key)
}
