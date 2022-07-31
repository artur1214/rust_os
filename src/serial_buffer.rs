use core::fmt;
use core::fmt::Write;
use lazy_static::lazy_static;
use spin::Mutex;
use crate::ports::{Port, ReadOnlyPort, WriteOnlyPort};
use crate::wait_for;

pub struct SerialPort {
    data: Port<u8>,
    int_en: WriteOnlyPort<u8>,
    fifo_ctrl: WriteOnlyPort<u8>,
    line_ctrl: Port<u8>,
    modem_ctrl: Port<u8>,
    line_status: ReadOnlyPort<u8>
}

impl SerialPort {
    pub fn new(port_address: u16) -> Self {
        let mut port = Self {
            data: Port::new(port_address),
            int_en: WriteOnlyPort::new(port_address + 1 ),
            fifo_ctrl: WriteOnlyPort::new(port_address + 2),
            line_ctrl: Port::new(port_address + 3),
            modem_ctrl: Port::new(port_address + 4),
            line_status: ReadOnlyPort::new(port_address + 5)
        };
        port.init();
        port
    }
    pub fn init(&mut self) {
        unsafe {
            // Disable interrupts
            self.int_en.write(0x00);

            // Enable DLAB
            self.line_ctrl.write(0x80);

            // Set maximum speed to 38400 bps by configuring DLL and DLM
            self.data.write(0x03);
            self.int_en.write(0x00);

            // Disable DLAB and set data word length to 8 bits
            self.line_ctrl.write(0x03);

            // Enable FIFO, clear TX/RX queues and
            // set interrupt watermark at 14 bytes
            self.fifo_ctrl.write(0xC7);

            // Mark data terminal ready, signal request to send
            // and enable auxilliary output #2 (used as interrupt line for CPU)
            self.modem_ctrl.write(0x0B);

            // Enable interrupts
            self.int_en.write(0x01);
        }
    }

    fn line_sts(&mut self) -> u8 {
        unsafe { self.line_status.read() }
    }

    fn line_empty(&mut self) -> bool {
        self.line_sts() & (1 << 5) != 0 // mb try line_sts() == 32
    }
    fn line_input_full(&mut self) -> bool {
        self.line_sts() & 1 != 0 // mb try line_sts() == 1
    }

    /// Sends a byte on the serial port.
    pub fn send(&mut self, data: u8) {
        unsafe {
            match data {
                8 | 0x7F => {
                    wait_for!(self.line_empty());
                    self.data.write(8);
                    wait_for!(self.line_empty());
                    self.data.write(b' ');
                    wait_for!(self.line_empty());
                    self.data.write(8)
                }
                _ => {
                    wait_for!(self.line_empty());
                    self.data.write(data);
                }
            }
        }
    }

    pub fn send_raw(&mut self, data: u8) {
        unsafe {
            wait_for!(self.line_empty());
            self.data.write(data);
        }
    }

    pub fn receive(&mut self) -> u8 {
        unsafe {
            wait_for!(self.line_input_full());
            self.data.read()
        }
    }
}

impl Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.send(byte);
        }
        Ok(())
    }
}

lazy_static!{
    pub static ref SERIAL: Mutex<SerialPort> = Mutex::new(SerialPort::new(0x3f8));
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ($crate::serial_buffer::_serial_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _serial_print(args: fmt::Arguments) {
    SERIAL.lock().write_fmt(args).unwrap();
}