#![no_std]
#![no_main]

use core::ptr::{read_volatile, write_volatile};

mod nvic;
mod panic_handler;

const RCC: u32 = 0x4002_1000;
const RCC_APB2ENR: u32 = RCC + 0x18;

const PORTC: u32 = 0x4001_1000;
const PORTC_CRL: u32 = PORTC + 4 * 0;
const PORTC_CRH: u32 = PORTC + 4 * 1;
const PORTC_IDR: u32 = PORTC + 4 * 1;
const PORTC_ODR: u32 = PORTC + 4 * 1;


#[no_mangle]
pub fn _start() -> ! {
    unsafe {
        write_volatile(RCC_APB2ENR as *mut u32, read_volatile(RCC_APB2ENR as *const u32) | (1 << 4));
        write_volatile(PORTC_CRH as *mut u32, (read_volatile(PORTC_CRH as *const u32) & !(0b1111 << 20)) | (0b11 << 20) | (0b00 << 20) );
    }

    loop {}
}