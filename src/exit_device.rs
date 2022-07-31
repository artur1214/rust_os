
use crate::ports::{Port};

pub struct ExitDevice {
    data: Port<u16>
}
const EXIT_DEVICE_ADDRESS: u16 = 0xf4;

#[repr(u8)]
enum ExitStatus {
    SUCCESS = 0x10,
    FAIL = 0x1
}

impl ExitDevice {
    pub fn new(address: u16) -> Self {
        Self { data: Port::new(address) }
    }

    pub fn exit(&mut self, exit_code: u8) {
        unsafe { self.data.write(exit_code as u16) };
    }
}

pub fn exit_success(){
    ExitDevice::new(EXIT_DEVICE_ADDRESS).exit(ExitStatus::SUCCESS as u8);
}

pub fn exit_error(){
    ExitDevice::new(EXIT_DEVICE_ADDRESS).exit(ExitStatus::FAIL as u8);
}