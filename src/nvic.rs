use crate::_start;

#[no_mangle]
#[link_section = ".text"]
pub unsafe fn reset_handler() -> ! {
    _start();
}

#[allow(dead_code)]
#[link_section = ".isr_table"]
pub static NVIC: [Option<unsafe fn() -> !>; 1] = [
    Some(reset_handler),
];