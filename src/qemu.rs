/// Enumerates the different exit codes for QEMU. We use this for our test runner, because we want
/// QEMU to automatically exit after running our tests. The exact values are simply values that are
/// not used by QEMU otherwise.
/// An example given in the guide was that using 0 as the success code was a bad idea, because of
/// the way QEMU transforms exit codes, which is (x << 1) | 1, where x is the exit code we pass to
/// QEMU. When x = 0, this transformation would result in 1, which is the exit code QEMU uses to
/// denote a failed run making it impossible to distinguish between our tests failing and QEMU
/// failing.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

/// Exits QEMU with the exit_code.
/// Used for our test_runner, because we want QEMU to exit after running our tests and reporting
/// the status of our tests with an exit_code. This exit code is written to the 0xf4 port on the
/// x86's IO bus, because that is a port that's usually unused. This port is then mapped to QEMU's
/// isa-debug-exit device with a port size of 4 bytes. This mapping is defined in the test-args in
/// the Cargo.toml, which defines the arguments passed to QEMU when running cargo test.
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
