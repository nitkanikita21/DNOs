#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![allow(dead_code)] //

use core::panic::PanicInfo;
mod vga_buffer;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = "GIGACHAD".as_bytes();

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();
    
    loop {}
}