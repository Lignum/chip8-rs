extern crate chip8;
extern crate sdl2;

mod emu;

fn main() {
    emu::Emulator::new().start_loop();
}