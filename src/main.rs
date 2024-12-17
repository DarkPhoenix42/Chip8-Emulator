mod emulator;

fn main() {
    let mut emulator = emulator::Emulator::new();
    emulator.load_rom("src/roms/INVADERS");
    emulator.run();
}
