use core::panic::PanicInfo;

use crate::qemu::{exit_qemu, QemuExitCode};
use crate::{serial_print, serial_println};

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        // Prints the type name, because for functions the function name IS the type name, so this
        // way we get the name of function we are testing in our test output.
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

/// Custom test runner. Simply taskes the list of test functions collected, prints how many tests
/// its running, and then calls all tests sequentially.
#[allow(dead_code)] // this code is only really used in tests, so cargo complains about dead code
                    // for non-test binaries
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
