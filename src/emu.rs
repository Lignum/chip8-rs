use chip8;

use sdl2;
use sdl2::event::Event;

const WINDOW_TITLE: &str = "CHIP-8 Emulator";

const PIXEL_WIDTH: u32 = 16;
const PIXEL_HEIGHT: u32 = 16;

pub struct Emulator {
    sdl: sdl2::Sdl,
    canvas: sdl2::render::WindowCanvas,
    event_pump: sdl2::EventPump,
    cpu: chip8::cpu::CPU
}

impl Emulator {
    pub fn new() -> Emulator {
        let sdl = sdl2::init().expect("Failed to initialise SDL2");
        let video = sdl.video().expect("Failed to initialise SDL2 video subsystem");

        let window = video.window(WINDOW_TITLE, 64 * PIXEL_WIDTH, 32 * PIXEL_HEIGHT)
            .position_centered()
            .build()
            .expect("Failed to create window");

        let canvas: sdl2::render::WindowCanvas = window.into_canvas()
            .build().expect("Failed to create canvas for window");

        let event_pump = sdl.event_pump().expect("Failed to initialise SDL2 event subsystem");

        Emulator { sdl, canvas, event_pump, cpu: chip8::cpu::CPU::new(64, 32) }
    }

    pub fn start_loop(&mut self) {
        self.cpu.mem.load_program(&[
            0xF0, 0x0A, // 0200 - LD Vx, K
            0x12, 0x00, // 0202 - JP 0x0200
        ]);

        'main_loop: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => break 'main_loop,
                    Event::KeyDown { keycode, .. } => self.cpu.press_key(0),
                    _ => {}
                }
            }

            self.cpu.step();
        }
    }
}