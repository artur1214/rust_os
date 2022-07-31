
use crate::ports::{Port};

pub struct ExitDevice {
    data: Port<u16>
}

impl ExitDevice {
    pub fn new(address: u16) -> Self {
        Self { data: Port::new(address) }
    }

    pub fn exit(&mut self, exit_code: u8){
        unsafe { self.data.write(exit_code as u16) };
    }
}

pub fn exit_success(){
    ExitDevice::new(0xf4).exit(0x10);
}

pub fn exit_error(){
    ExitDevice::new(0xf4).exit(0x1);
}