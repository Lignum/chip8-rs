#![feature(plugin)]
#![plugin(phf_macros)]

extern crate chip8;
extern crate sdl2;
extern crate phf;

mod emu;

fn main() {
    emu::Emulator::new().start_loop();
}