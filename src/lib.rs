#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt, custom_test_frameworks)]
#![test_runner(crate::test_runner::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
use core::panic::PanicInfo;

pub mod gdt;
pub mod interrupts;
pub mod qemu;
#[macro_use]
pub mod serial;
pub mod test_runner;
pub mod vga_buffer;

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}

/// Central function for anything that needs to initialised
pub fn init() {
    gdt::init();
    interrupts::init_dt();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use test_runner::test_panic_handler;

    test_panic_handler(info)
}
