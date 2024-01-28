// disabling the standard library
#![no_std]
// telling the compiler to not use the normal entry point chain
#![no_main]

use core::panic::PanicInfo;

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Creating an entry point. Also tells the compiler to use the C calling convention, rather than the rust convention.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
