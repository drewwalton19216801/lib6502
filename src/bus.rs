//! The `bus` module defines the `Bus` trait for memory and I/O operations.

/// The `Bus` trait represents the system bus for memory and I/O operations.
pub trait Bus {
    /// Reads a byte from the given address.
    ///
    /// # Arguments
    ///
    /// * `addr` - The memory address to read from.
    ///
    /// # Returns
    ///
    /// The byte read from memory.
    fn read(&mut self, addr: u16) -> u8;

    /// Writes a byte to the given address.
    ///
    /// # Arguments
    ///
    /// * `addr` - The memory address to write to.
    /// * `data` - The byte to write to memory.
    fn write(&mut self, addr: u16, data: u8);
}
