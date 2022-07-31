
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

#![feature(custom_test_frameworks)] // Custom tests
#![test_runner(rust_os::test_runner)] // Tells, that we want to use test_runner_func as test runner. (Genius)


#![reexport_test_harness_main = "test_main"]


pub mod vga_buffer;
pub mod serial_buffer;

pub mod exit_device;
pub mod ports;


use core::panic::PanicInfo;
use crate::serial_buffer::SERIAL;

/// This function is called on panic.
#[panic_handler]
#[cfg(not(test))] // don't use in test mode
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
//Use this instead in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();
    println!("Hello world!");
    serial_print!("Hello {} serial port!", "from");
    //serial_println!();
    loop {
        let x = SERIAL.lock().receive();
        serial_print!("{}", x as char);
        if 0 == 1 {
            break
        }
    }
    loop {}
}
