#![no_std]
#![no_main]

use core::{arch::global_asm, panic::PanicInfo};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

global_asm!(include_str!("entry.asm"));

#[no_mangle]
fn main() {
    putstr("Hello, world!\n");
}

#[inline(never)]
fn putc(c: u8) {
    unsafe {
        (0x10000000 as *mut u8).write_volatile(c);
    }
}

fn putstr(s: &str) {
    for c in s.bytes() {
        putc(c);
    }
}
