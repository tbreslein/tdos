// We cannot depend on OS features, since we are writing an OS
#![no_std]
// Without std, we don't have access to the Rust runtime, which is usually responsible for calling
// the entry point (i.e. the main function). Thus, we have to create our own entry point, aka the
// function marked with `start`, and disable all Rust-level entry points.
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

/// core does not provide its own panic handler, as its defined in std. Since we have a #![no_std]
/// environment, we have to write our own panic_handler. The #[panic_handler] attribute lets the
/// compiler now that this is the panic handler it needs to use.
///
/// NOTE: The ! is the "never" type because this function is supposed to never return.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// The custom entry point for the binary.
///
/// This function needs #[no_mangle] so that this function is actually going to be called _start,
/// instead of some cryptic identifier.
/// This is important, because calling the entry point _start is the regular default calling
/// convention for such a function for most systems.
/// This function also is not allowed to return ever, because the function is called by the
/// bootloader directly, instead of a function inside of the code base.
/// Eventually, we will want to call something like the exit system call.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello {}", "World!");
    panic!("foobar");
    loop {}
}
