use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

// Our primary serial port is a UART 16550, which is a serial device model supported by all common
// UARTS (a UART simply being a chip implementing a serial interface).
// Like our VGA text buffer, this serial port is wrapped in a mutex to make sure that only ever one
// process is writing to this port. The port address for the serial port is 0x3F8, which is the
// standard port for the first serial interface.
// Unlike the VGA text buffer, this is obviously port IO though; the VGA text buffer was memory IO.
lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

/// Writes formatted args to the SERIAL1 device.
/// NOTE: uart_16550::SerialPort already implements fmt::Write, so we can call write_fmt on it
#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
}

/// Prints to the host using the first serial interface.
/// Similar to our print implementation, but instead we use the _print function in this module to
/// write to SERIAL1.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ($crate::serial::_print(format_args!($($arg)*)));
}

/// Prints to the host using the first serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => {
        $crate::serial_print!("\n")
    };
    ($fmt:expr) => {
        $crate::serial_print!(concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}
