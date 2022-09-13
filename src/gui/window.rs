extern crate alloc;

use alloc::vec::Vec;

pub struct Window {
    width: u8,
    height: u8,
    elements: Vec<dyn UIElement>
}

trait UIElement {
    fn draw(&self);
}