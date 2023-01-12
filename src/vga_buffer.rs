use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

// Public static interface for interacting with the VGA buffer. This is defined as a lazy static,
// because Rust must initialise regular statics at compile time, but it cannot initialise
// references at compile time. The lazy static initialises itself when it is used for the first
// time during run time, so that's when the reference to the VGA buffer will also be initialised.
//
// We also wrap the writer in a spinlock mutex. An immutable writer would be useless, but in order
// for writing to the buffer to be potentially async and safe, we need a locking mechanism.
// Spinlocks are a primitive mutex that, when locked, just "spins" a tight loop till the lock is
// released.
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

/// our own print! macro, because we have to use a custom _print function that interacts with our
/// WRITER.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

/// see print!
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// custom _print function that uses our WRITER. The docs are hidden because this function is an
/// implementation detail for our print macros, because our print macros are put at the crate root
/// namespace in order to be available outside of this module. So, in order to make sure that the
/// macros can expand into this function, it needs to be publically available throughout the crate.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

/// Enum to represent the 4 bits declaring the color of a code page 437 character used in the VGA
/// text buffer. If Rust supported u4, that's what this would be representing it, but instead we
/// have to use u8.
/// Generally, those 4 bits consist of 3 bits for the base color + 1 bit for whether it's bright or
/// not. For example, the binary 0000 stands for black, and 0001 for blue, and then 1000 is "bright
/// black" (or dark gray) and 1001 is light blue.
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

/// Repesents the full color code (foreground + background). It is transparently represented by a
/// u8, but we can give it new methods and stuff like that (kind of like distinct types in nim and
/// odin).
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> Self {
        // shift the background bits into the leftmost bits of the u8, and keep the foreground
        // color in rightmost bits; the bitwise or | "adds" the foreground bits to the bits of the
        // byte left over after the left shift.
        return ColorCode((background as u8) << 4 | (foreground as u8));
    }
}

/// Represents a character in the VGA text buffer, consisting of a code page 437 character and its
/// color code.
/// In order to make sure that the layout is exactly as we define it here, we add the #[repr(C)] to
/// enforce C style field ordering, instead of Rust style ordering (which may be switched around by
/// the compiler).
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
struct ScreenChar {
    character: u8,
    color_code: ColorCode,
}

/// Number of rows in the VGA buffer
const BUFFER_HEIGHT: usize = 25;

/// Number of columns in the VGA buffer
const BUFFER_WIDTH: usize = 80;

/// The VGA buffer, which is basically just an array of an array of ScreenChar, representing the
/// matrix of characters being stored in the VGA buffer.
/// In order for the representation to match the array of an array (of essentially 2 u8), we tell
/// the compiler again to make the representation transparent.
/// The ScreenChar is wrapped in a Volatile to make sure that this array will never be optimised
/// away, even if it isn't used (directly).
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// Public facing object responsible for writing to the VGA buffer. The way it is going to write to
/// is to write to the bottom line, and when that line is full or it hits a line break, all lines
/// are shifted one row up, with the top most row being lost.
/// While writing to a row, it keeps track of the column it would be writing to next as well as the
/// current color code.
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    // Note that the life time for this reference is static, because the VGA buffer is supposed to
    // live for the full run time of program (aka the kernel)
    buffer: &'static mut Buffer,
}

impl Writer {
    /// writes a single byte to the last row at self.column_position, and advance column_position.
    /// In case the line is full, or the byte is a newline, we write a new line first.
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    character: byte,
                    color_code,
                });
                self.column_position += 1;
            },
        }
    }

    /// Write a string into the buffer, which just means we write each byte of the string byte by
    /// byte.
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // code page 437 character => write that byte
                0x20..=0x7e | b'\n' => self.write_byte(byte),

                // byte outside of the code page 437 range, for example characters with an umlaut
                //  => write the block character
                _ => self.write_byte(0xfe),
            };
        }
    }

    /// Take every row, starting at the second from the top, and write to the row above it, thus
    /// shifting the content one row upwards
    fn new_line(&mut self) {
        // start at row 1 instead of row 0, because row 0 is being overwritten by row 1
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                // take the character the current position [row][col], and write it to the same
                // column in the row above it.
                self.buffer.chars[row - 1][col].write(self.buffer.chars[row][col].read());
            }
        }

        // empty the bottom most row and put the cursor in the leftmost position
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    /// Overwrite the characters in a given row with the blank character
    fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(ScreenChar {
                character: b' ',
                color_code: self.color_code,
            });
        }
    }
}

/// This trait impl gives us the ability to use the write! and writeln! macros
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        return Ok(());
    }
}

// // Function to demonstate using a Writer
// pub fn print_hello_world() {
//     use core::fmt::Write;
//
//     let mut writer = Writer {
//         column_position: 0,
//         color_code: ColorCode::new(Color::Yellow, Color::Black),
//         buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
//     };
//
//     // Just to demonstrate we can write bytes as well as strings.
//     writer.write_byte(b'H');
//     writer.write_string("ello ");
//
//     // The guide also demonstates what happens when you write an o umlaut instead of the o. Since
//     // the o umlaut as a UTF-8 character consistent of two bytes, and both are outside of the code
//     // page 437 range, that letter is going to be written as two block characters.
//     writer.write_string("World! ");
//
//     write!(writer, "Some numbers: {} and {}", 42, 1.0 / 3.0).unwrap();
// }

// Just run the println! macro and check that it does not panic
#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

// Same as above but for a number of println statements
#[test_case]
fn test_println_many() {
    for _ in 0..300 {
        println!("test_println_simple output");
    }
}

// Test that a line of text printed to the VGA buffer has actually been written to that buffer
#[test_case]
fn test_println_output() {
    // our test string
    let s = "foo bar baz";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        // read the buffer and check, character for character, that it actually equals the
        // character in our test string
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.character), c);
    }
}
