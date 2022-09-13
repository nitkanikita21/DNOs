use core::arch::asm;

pub fn get_video_mode() -> u8 {
    let mut mode: u8 = 2;
    unsafe {
        asm!(
            ".code16gcc",
            "mov ax, 0x0001",
            "int 0x10",
            "mov ax, 0x0f00",
            "int 0x10",
            "mov al, ah",
            out("al") mode
        );
    }
    mode
}
