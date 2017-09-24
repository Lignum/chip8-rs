pub trait IOInterface {
    fn clear_screen(&mut self) -> ();

    fn is_key_pressed(&self, key: u8) -> bool;

    fn set_pixel(&mut self, x: u32, y: u32, white: bool) -> ();

    fn pixel(&self, x: u32, y: u32) -> bool;
}

pub struct HeadlessInterface {}

impl IOInterface for HeadlessInterface {
    fn clear_screen(&mut self) {}

    fn is_key_pressed(&self, _: u8) -> bool { false }

    fn set_pixel(&mut self, _: u32, _: u32, _: bool) -> () {}

    fn pixel(&self, _: u32, _: u32) -> bool { false }
}
