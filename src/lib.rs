#![warn(missing_docs)]
//! A 6502 emulator library written in Rust.
//!
//! Aims to provide a simple, easy-to-use interface for emulating the 6502 CPU.
//! The CPU connects to a bus, and your emulator can define any number of devices
//! on the bus. The CPU can then read and write to these devices.

mod addresses;

use bitflags::bitflags;
use crate::addresses::RESET_VECTOR;

/// The emulated 6502 CPU
pub struct Cpu {
    a: u8,
    x: u8,
    y: u8,
    sp: u8,
    pc: u16,
    status: u8,

    /// The memory of the CPU
    /// TODO: Make this a bus that the CPU can connect to
    pub memory: Box<[u8]>,
}

bitflags! {
    /// A set of flags that can be set in the status register
    pub struct StatusFlags: u8 {
        /// No flags
        const None = 0b0000_0000;

        /// Carry flag
        const Carry = 0b0000_0001;

        /// Zero flag
        const Zero = 0b0000_0010;

        /// Interrupt disable flag
        const InterruptDisable = 0b0000_0100;

        /// Decimal mode flag
        const Decimal = 0b0000_1000;

        /// Break flag
        const Break = 0b0001_0000;

        /// Unused flag
        const Unused = 0b0010_0000;

        /// Overflow flag
        const Overflow = 0b0100_0000;

        /// Negative flag
        const Negative = 0b1000_0000;
    }
}

impl Cpu {
    /// Create a new CPU
    pub fn new() -> Cpu {
        Cpu {
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            pc: 0,
            status: StatusFlags::None.bits(),
            memory: Box::new([0; 0x10000]),
        }
    }

    /// Reset the CPU
    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xfd;
        self.pc = self.read_u16(RESET_VECTOR);
        self.status = StatusFlags::None.bits() | StatusFlags::Unused.bits() | StatusFlags::InterruptDisable.bits();
    }

    /// Read a byte from memory
    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    /// Read a 16-bit word from memory
    pub fn read_u16(&self, address: u16) -> u16 {
        let lo = self.read(address) as u16;
        let hi = self.read(address + 1) as u16;
        (hi << 8) | lo
    }

    /// Write a byte to memory
    pub fn write(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reset() {
        let mut cpu = Cpu::new();
        cpu.reset();
        let calculated_pc = cpu.read_u16(RESET_VECTOR);
        assert_eq!(calculated_pc, cpu.pc);
        assert_eq!(cpu.status, StatusFlags::None.bits() | StatusFlags::Unused.bits() | StatusFlags::InterruptDisable.bits());
        assert_eq!(cpu.sp, 0xfd);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
    }
}