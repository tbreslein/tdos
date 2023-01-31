#![no_std]
#![no_main]

use core::panic::PanicInfo;
use tdos::{
    qemu::{exit_qemu, QemuExitCode},
    serial_print, serial_println,
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn should_fail() {
    // NOTE: We don't use the Testable trait, so we need to spell out the module and fn name
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}
