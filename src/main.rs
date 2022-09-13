#![feature(abi_x86_interrupt)]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![allow(dead_code)] //

mod tests;
pub mod cpu_interrupts;
pub mod graphics;

use core::{panic::PanicInfo};
use crate::graphics::video::get_video_mode;



/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print!("Error: {_info}");
    loop {}
}

static HELLO: &[u8] = "GIGACHAD".as_bytes();

#[no_mangle]
pub extern "C" fn _start() -> ! {

    cpu_interrupts::init();
    print!("Video mode: {}", get_video_mode());
    loop {}
}
