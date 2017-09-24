use chip8;
use phf;

use sdl2;
use sdl2::event::Event;

use time;

const WINDOW_TITLE: &str = "CHIP-8 Emulator";

const PIXEL_WIDTH: u32 = 16;
const PIXEL_HEIGHT: u32 = 16;

const TICK_FREQUENCY: u64 = 60;

static KEY_MAPPING: phf::Map<&'static str, u8> = phf_map! {
    "1" => 0x1,
    "2" => 0x2,
    "3" => 0x3,
    "4" => 0xC,
    "Q" => 0x4,
    "W" => 0x5,
    "E" => 0x6,
    "R" => 0xD,
    "A" => 0x7,
    "S" => 0x8,
    "D" => 0x9,
    "F" => 0xE,
    "Z" => 0xA,
    "Y" => 0xA,
    "X" => 0x0,
    "C" => 0xB,
    "V" => 0xF
};

pub struct Emulator {
    canvas: sdl2::render::WindowCanvas,
    event_pump: sdl2::EventPump,
    cpu: chip8::cpu::CPU
}

fn time_now_ms() -> u64 {
    let ts = time::get_time();
    (ts.sec + ts.nsec as i64 / 1000000) as u64
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
            .present_vsync()
            .build().expect("Failed to create canvas for window");

        let event_pump = sdl.event_pump().expect("Failed to initialise SDL2 event subsystem");

        Emulator { canvas, event_pump, cpu: chip8::cpu::CPU::new() }
    }

    fn draw_screen(&mut self) {
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        self.canvas.clear();

        let w = self.cpu.env.display_width as u32;
        let h = self.cpu.env.display_height as u32;
        let disp = &self.cpu.env.display;

        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));

        for x in 0..(w-1) {
            for y in 0..(h-1) {
                if disp[(y * w + x) as usize] {
                    let rect = sdl2::rect::Rect::new(
                        (x * PIXEL_WIDTH) as i32,
                        (y * PIXEL_HEIGHT) as i32,
                        PIXEL_WIDTH,
                        PIXEL_HEIGHT
                    );

                    self.canvas.fill_rect(rect).expect("Failed to draw rectangle");
                }
            }
        }
    }

    pub fn start(&mut self, program: &[u8]) {
        self.cpu.mem.load_program(program);

        let mut last_time = time_now_ms();
        let mut tick_timer = 0;

        'main_loop: loop {
            let now = time_now_ms();
            let dt = if last_time > now { now } else { now - last_time };
            last_time = now;

            tick_timer += dt;

            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => break 'main_loop,
                    Event::KeyDown { keycode, .. } => {
                        match keycode.and_then(|k| KEY_MAPPING.get(k.name().as_str())) {
                            Some(key) => {
                                self.cpu.env.keyboard[*key as usize] = true;
                                self.cpu.press_key(*key)
                            },
                            None => {}
                        }
                    },
                    Event::KeyUp { keycode, .. } => {
                        match keycode.and_then(|k| KEY_MAPPING.get(k.name().as_str())) {
                            Some(key) => self.cpu.env.keyboard[*key as usize] = false,
                            None => {}
                        }
                    }
                    _ => {}
                }
            }

            if tick_timer >= 1000 / TICK_FREQUENCY {
                self.cpu.tick();
                tick_timer = 0;
            }

            self.cpu.step();
            self.draw_screen();

            self.canvas.present();
        }
    }
}