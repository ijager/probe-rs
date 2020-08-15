use crate::error;

pub trait MemoryInterface {
    /// Read a 32bit word of at `address`.
    ///
    /// The address where the read should be performed at has to be word aligned.
    /// Returns `AccessPortError::MemoryNotAligned` if this does not hold true.
    fn read_word_32(&mut self, address: u32) -> Result<u32, error::Error>;

    /// Read an 8bit word of at `address`.
    fn read_word_8(&mut self, address: u32) -> Result<u8, error::Error>;

    /// Read a block of 32bit words at `address`.
    ///
    /// The number of words read is `data.len()`.
    /// The address where the read should be performed at has to be word aligned.
    /// Returns `AccessPortError::MemoryNotAligned` if this does not hold true.
    fn read_32(&mut self, address: u32, data: &mut [u32]) -> Result<(), error::Error>;

    /// Read a block of 8bit words at `address`.
    fn read_8(&mut self, address: u32, data: &mut [u8]) -> Result<(), error::Error>;

    /// Write a 32bit word at `address`.
    ///
    /// The address where the write should be performed at has to be word aligned.
    /// Returns `AccessPortError::MemoryNotAligned` if this does not hold true.
    fn write_word_32(&mut self, address: u32, data: u32) -> Result<(), error::Error>;

    /// Write an 8bit word at `address`.
    fn write_word_8(&mut self, address: u32, data: u8) -> Result<(), error::Error>;

    /// Write a block of 32bit words at `address`.
    ///
    /// The number of words written is `data.len()`.
    /// The address where the write should be performed at has to be word aligned.
    /// Returns `AccessPortError::MemoryNotAligned` if this does not hold true.
    fn write_32(&mut self, address: u32, data: &[u32]) -> Result<(), error::Error>;

    /// Write a block of 8bit words at `address`.
    fn write_8(&mut self, address: u32, data: &[u8]) -> Result<(), error::Error>;

    /// Flush any outstanding operations.
    ///
    /// For performance, debug probe implementations may choose to batch writes;
    /// to assure that any such batched writes have in fact been issued, `flush`
    /// can be called.  Takes no arguments, but may return failure if a batched
    /// operation fails.
    fn flush(&mut self) -> Result<(), error::Error>;
}

impl<T> MemoryInterface for &mut T
where
    T: MemoryInterface,
{
    fn read_word_32(&mut self, address: u32) -> Result<u32, error::Error> {
        (*self).read_word_32(address)
    }

    fn read_word_8(&mut self, address: u32) -> Result<u8, error::Error> {
        (*self).read_word_8(address)
    }

    fn read_32(&mut self, address: u32, data: &mut [u32]) -> Result<(), error::Error> {
        (*self).read_32(address, data)
    }

    fn read_8(&mut self, address: u32, data: &mut [u8]) -> Result<(), error::Error> {
        (*self).read_8(address, data)
    }

    fn write_word_32(&mut self, addr: u32, data: u32) -> Result<(), error::Error> {
        (*self).write_word_32(addr, data)
    }

    fn write_word_8(&mut self, addr: u32, data: u8) -> Result<(), error::Error> {
        (*self).write_word_8(addr, data)
    }

    fn write_32(&mut self, addr: u32, data: &[u32]) -> Result<(), error::Error> {
        (*self).write_32(addr, data)
    }

    fn write_8(&mut self, addr: u32, data: &[u8]) -> Result<(), error::Error> {
        (*self).write_8(addr, data)
    }

    fn flush(&mut self) -> Result<(), error::Error> {
        (*self).flush()
    }
}
