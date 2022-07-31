use core::arch::asm;
use core::fmt;
use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;

#[macro_export]
macro_rules! wait_for {
    ($cond:expr) => {
        while !$cond {
            core::hint::spin_loop()
        }
    };
}

pub trait PortRead {
    unsafe fn read_from_port(port: u16) -> Self;
}

pub trait PortWrite {
    unsafe fn write_to_port(port: u16, value: Self);
}

impl PortRead for u8 {
    #[inline]
    unsafe fn read_from_port(port: u16) -> u8 {
        let value: u8;

        asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack, preserves_flags));

        value
    }
}

impl PortRead for u16 {
    #[inline]
    unsafe fn read_from_port(port: u16) -> u16 {
        let value: u16;

        asm!("in ax, dx", out("ax") value, in("dx") port, options(nomem, nostack, preserves_flags));

        value
    }
}

impl PortRead for u32 {
    #[inline]
    unsafe fn read_from_port(port: u16) -> u32 {
        let value: u32;
        asm!("in eax, dx", out("eax") value, in("dx") port, options(nomem, nostack, preserves_flags));
        value
    }
}

impl PortWrite for u8 {
    #[inline]
    unsafe fn write_to_port(port: u16, value: u8) {
        asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
    }
}

impl PortWrite for u16 {
    #[inline]
    unsafe fn write_to_port(port: u16, value: u16) {
        asm!("out dx, ax", in("dx") port, in("ax") value, options(nomem, nostack, preserves_flags));
    }
}

impl PortWrite for u32 {
    #[inline]
    unsafe fn write_to_port(port: u16, value: u32) {
        asm!("out dx, eax", in("dx") port, in("eax") value, options(nomem, nostack, preserves_flags));
    }
}

mod sealed {
    pub trait Access {
        const DEBUG_NAME: &'static str;
    }
}

/// A marker trait for access types which allow reading port values.
pub trait PortReadAccess: sealed::Access {}

/// A marker trait for access types which allow writing port values.
pub trait PortWriteAccess: sealed::Access {}

/// An access marker type indicating that a port is only allowed to read values.
#[derive(Debug)]
pub struct ReadOnlyAccess(());

impl sealed::Access for ReadOnlyAccess {
    const DEBUG_NAME: &'static str = "ReadOnly";
}
impl PortReadAccess for ReadOnlyAccess {}

/// An access marker type indicating that a port is only allowed to write values.
#[derive(Debug)]
pub struct WriteOnlyAccess(());

impl sealed::Access for WriteOnlyAccess {
    const DEBUG_NAME: &'static str = "WriteOnly";
}
impl PortWriteAccess for WriteOnlyAccess {}

/// An access marker type indicating that a port is allowed to read or write values.
#[derive(Debug)]
pub struct ReadWriteAccess(());

impl sealed::Access for ReadWriteAccess {
    const DEBUG_NAME: &'static str = "ReadWrite";
}
impl PortReadAccess for ReadWriteAccess {}
impl PortWriteAccess for ReadWriteAccess {}

pub struct PortGeneric<T, A> {
    port: u16,
    phantom: PhantomData<(T, A)>,
}

impl<T: PortRead, A: PortReadAccess> PortGeneric<T, A> {
    #[inline]
    pub unsafe fn read(&mut self) -> T {
        T::read_from_port(self.port)
    }
}

impl<T: PortWrite, A: PortWriteAccess> PortGeneric<T, A> {
    #[inline]
    pub unsafe fn write(&mut self, value: T) {
        T::write_to_port(self.port, value)
    }
}
impl<T, A> PortGeneric<T, A> {
    #[inline]
    pub const fn new(port: u16) -> PortGeneric<T, A> {
        PortGeneric {
            port,
            phantom: PhantomData,
        }
    }
}

impl<T, A: sealed::Access> Debug for PortGeneric<T, A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("PortGeneric")
            .field("port", &self.port)
            .field("size", &core::mem::size_of::<T>())
            .field("access", &format_args!("{}", A::DEBUG_NAME))
            .finish()
    }
}

pub type Port<T> = PortGeneric<T, ReadWriteAccess>;
pub type WriteOnlyPort<T> = PortGeneric<T, WriteOnlyAccess>;
pub type ReadOnlyPort<T> = PortGeneric<T, ReadOnlyAccess>;

