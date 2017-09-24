pub trait IOInterface {
    fn clear_screen(&mut self) -> ();
}

pub struct HeadlessInterface {}

impl IOInterface for HeadlessInterface {
    fn clear_screen(&mut self) {}
}
