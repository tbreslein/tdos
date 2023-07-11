// We cannot depend on OS features, since we are writing an OS
#![no_std]
// Without std, we don't have access to the Rust runtime, which is usually responsible for calling
// the entry point (i.e. the main function). Thus, we have to create our own entry point, aka the
// function marked with `start`, and disable all Rust-level entry points.
#![no_main]
// We need a custom test framework, because the standard testing framework depends on std
#![feature(custom_test_frameworks)]
// Redefine which function is used as the test run
#![test_runner(tdos::test_runner::test_runner)]
// Redefine what the test harness is called. This is needed, because we have no main, but a main
// function is exactly what the custom_test_frameworks feature calls the function that calls the
// test_runner. Thus, we need to rename that function, and then we can call it in our _start.
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use tdos::println;

mod qemu;
mod serial;
#[cfg(test)]
mod test_runner;

/// core does not provide its own panic handler, as its defined in std. Since we have a #![no_std]
/// environment, we have to write our own panic_handler. The #[panic_handler] attribute lets the
/// compiler now that this is the panic handler it needs to use.
///
/// NOTE: The ! is the "never" type because this function is supposed to never return.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// Seperate panic handler when running tests. This writes to our SERIAL1 device which is then
/// rerouted to the VM host's stdio. This way we can see panics when running tests in our console,
/// because QEMU can print it to said console.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use test_runner::test_panic_handler;
    test_panic_handler(info)
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
    println!("Welcome to tdos!");
    println!("Unfortunately, this little kernel\nisn't interactive yet... <.<");

    tdos::init();
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    // draw_heart();
    println!("It didn't crash though, that exception was planned!");
    loop {}
}

#[allow(dead_code)]
fn draw_heart() {
    println!("   *******     *******   ");
    println!("  *       *   *       *  ");
    println!(" *         ***         * ");
    println!("*  ======       ======  *");
    println!("*    II     +   II      *");
    println!("*    II    +++  ======  *");
    println!("*    II     +       II  *");
    println!(" *   II         ====== * ");
    println!("  *                   *  ");
    println!("   *                 *   ");
    println!("    *               *    ");
    println!("     *             *     ");
    println!("      *           *      ");
    println!("       *         *       ");
    println!("        *       *        ");
    println!("         *     *         ");
    println!("          *   *          ");
    println!("           * *           ");
    println!("            *            ");
}

/// Tests the test runner, basically
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
