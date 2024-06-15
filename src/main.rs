#![no_std]
#![no_main]

mod nvic;
mod panic_handler;

#[no_mangle]
pub fn _start() -> ! {
    loop {}
}