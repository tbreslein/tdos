#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(tdos::test_runner::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::format_args;
use core::panic::PanicInfo;
use tdos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    tdos::test_runner::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("testing println output");
}
