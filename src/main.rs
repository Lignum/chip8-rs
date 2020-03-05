#![windows_subsystem = "windows"]

extern crate chip8;
extern crate sdl2;
extern crate phf;

use std::fs::File;
use std::io::Read;

mod emu;

fn main() {
    let program = {
        let mut file = File::open("game.ch8").expect("Failed to open ROM!");
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).expect("Failed to read ROM to end");
        buf
    };

    let mut emulator = emu::Emulator::new();
    emulator.start(&program);
}