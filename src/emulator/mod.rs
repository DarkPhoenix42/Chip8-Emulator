pub mod consts;
pub mod display;
pub mod input;
pub mod processor;

pub struct Emulator {
    processor: processor::Processor,
    display: display::Display,
    input: input::Input,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            processor: processor::Processor::new(),
            display: display::Display::new(),
            input: input::Input::new(),
        }
    }

    pub fn load_rom(&mut self, rom_path: &str) {
        match self.processor.load_rom(rom_path) {
            Ok(_) => (),
            Err(err) => println!("{}", err),
        }
    }

    pub fn run(&mut self) {
        loop {
            self.processor.emulate_cycle().unwrap();
            self.display.draw_texture(&self.processor.display);
            self.input.handle_keypress(&mut self.processor.keys_pressed);
        }
    }
}
