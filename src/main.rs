
#![feature(custom_test_frameworks)] // Custom tests
#![test_runner(crate::test_runner)] // Tells, that we want to use test_runner_func as test runner. (Genius)

#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![reexport_test_harness_main = "test_main"]



mod vga_buffer;
mod serial_buffer;

mod exit_device;
mod ports;

use core::panic::PanicInfo;
use crate::serial_buffer::SERIAL;

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_device::ExitDevice::new(0xF4).exit(0x10); // Close QEMU after tests (success)
}

//qemu-system-x86_64 -drive format=raw,file=/home/artur1214/Desktop/os/target/x86_64-rust_os/debug/bootimage-rust_os.bin
/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    println!("Hello world!");
    serial_print!("Hello {} serial port!", "from");
    serial_println!();
    loop {
        let x = SERIAL.lock().receive();
        serial_print!("{}", x as char);
    }

    loop {}
}


#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}