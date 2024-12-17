use crate::emulator::consts;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Input {
    event_pump: sdl2::EventPump,
}

impl Input {
    pub fn new() -> Input {
        let sdl_context = sdl2::init().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        Input { event_pump }
    }

    pub fn handle_keypress(&mut self, is_key_pressed: &mut [bool; consts::KEYPAD_SIZE]) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { timestamp: _ } => {
                    std::process::exit(0);
                }

                Event::KeyDown {
                    timestamp: _,
                    window_id: _,
                    keycode,
                    scancode: _,
                    keymod: _,
                    repeat: _,
                } => match keycode.unwrap() {
                    Keycode::Num1 => is_key_pressed[0x1] = true,
                    Keycode::Num2 => is_key_pressed[0x2] = true,
                    Keycode::Num3 => is_key_pressed[0x3] = true,
                    Keycode::Num4 => is_key_pressed[0xC] = true,
                    Keycode::Q => is_key_pressed[0x4] = true,
                    Keycode::W => is_key_pressed[0x5] = true,
                    Keycode::E => is_key_pressed[0x6] = true,
                    Keycode::R => is_key_pressed[0xD] = true,
                    Keycode::A => is_key_pressed[0x7] = true,
                    Keycode::S => is_key_pressed[0x8] = true,
                    Keycode::D => is_key_pressed[0x9] = true,
                    Keycode::F => is_key_pressed[0xE] = true,
                    Keycode::Z => is_key_pressed[0xA] = true,
                    Keycode::X => is_key_pressed[0x0] = true,
                    Keycode::C => is_key_pressed[0xB] = true,
                    Keycode::V => is_key_pressed[0xF] = true,
                    Keycode::Escape => std::process::exit(0),
                    _ => {}
                },

                Event::KeyUp {
                    timestamp: _,
                    window_id: _,
                    keycode,
                    scancode: _,
                    keymod: _,
                    repeat: _,
                } => match keycode.unwrap() {
                    Keycode::Num1 => is_key_pressed[0x1] = false,
                    Keycode::Num2 => is_key_pressed[0x2] = false,
                    Keycode::Num3 => is_key_pressed[0x3] = false,
                    Keycode::Num4 => is_key_pressed[0xC] = false,
                    Keycode::Q => is_key_pressed[0x4] = false,
                    Keycode::W => is_key_pressed[0x5] = false,
                    Keycode::E => is_key_pressed[0x6] = false,
                    Keycode::R => is_key_pressed[0xD] = false,
                    Keycode::A => is_key_pressed[0x7] = false,
                    Keycode::S => is_key_pressed[0x8] = false,
                    Keycode::D => is_key_pressed[0x9] = false,
                    Keycode::F => is_key_pressed[0xE] = false,
                    Keycode::Z => is_key_pressed[0xA] = false,
                    Keycode::X => is_key_pressed[0x0] = false,
                    Keycode::C => is_key_pressed[0xB] = false,
                    Keycode::V => is_key_pressed[0xF] = false,
                    _ => {}
                },

                _ => {}
            }
        }
    }
}
