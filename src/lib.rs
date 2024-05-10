#![warn(missing_docs)]
//! A 6502 emulator library written in Rust.
//!
//! Aims to provide a simple, easy-to-use interface for emulating the 6502 CPU.
//! The CPU connects to a bus, and your emulator can define any number of devices
//! on the bus. The CPU can then read and write to these devices.

mod addresses;
/// The bus module
pub mod bus;
mod addressing;
mod instructions;

use bitflags::bitflags;
use crate::addresses::RESET_VECTOR;
use crate::addressing::AddressingMode;
use crate::bus::Bus;

/// The emulated 6502 CPU
pub struct Cpu {
    /// The 8-bit accumulator
    pub a: u8,
    /// The 8-bit x index register
    pub x: u8,
    /// The 8-bit y index register
    pub y: u8,
    /// The 8-bit stack pointer
    pub sp: u8,
    /// The 16-bit program counter
    pub pc: u16,
    /// The status flags
    pub status: u8,
    /// The number of cycles remaining
    pub cycles: u8,

    addr_abs: u16,
    addr_rel: u16,
    addr_mode: AddressingMode,
    opcode: u8,
    fetched_data: u8,

    /// The memory of the CPU
    // TODO: Make this a bus that the CPU can connect to
    pub bus: Box<dyn Bus>,
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
    pub fn new(bus: Box<dyn Bus>) -> Cpu {
        Cpu {
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            pc: 0,
            status: StatusFlags::None.bits(),
            cycles: 0,

            addr_abs: 0,
            addr_rel: 0,
            addr_mode: AddressingMode::None,
            opcode: 0,
            fetched_data: 0,
            bus
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
        self.bus.read(address)
    }

    /// Read a 16-bit word from memory
    pub fn read_u16(&self, address: u16) -> u16 {
        let lo = self.read(address) as u16;
        let hi = self.read(address + 1) as u16;
        (hi << 8) | lo
    }

    /// Write a byte to memory
    pub fn write(&mut self, address: u16, value: u8) {
        self.bus.write(address, value)
    }
}

#[cfg(test)]
mod cpu_tests {
    struct TestBus {
        ram: Vec<u8>,
    }

    impl TestBus {
        fn new() -> TestBus {
            TestBus {
                ram: vec![0; 0x10000],
            }
        }
    }

    impl Bus for TestBus {
        fn read(&self, address: u16) -> u8 {
            self.ram[address as usize]
        }
        fn write(&mut self, address: u16, value: u8) {
            self.ram[address as usize] = value;
        }
    }

    use super::*;
    #[test]
    fn test_reset() {
        let bus = Box::new(TestBus::new());
        let mut cpu = Cpu::new(bus);
        cpu.write(RESET_VECTOR, 0x00);
        cpu.write(RESET_VECTOR + 1, 0x80);
        cpu.reset();

        assert_eq!(cpu.pc, 0x8000);
        assert_eq!(cpu.status, StatusFlags::None.bits() | StatusFlags::Unused.bits() | StatusFlags::InterruptDisable.bits());
        assert_eq!(cpu.sp, 0xfd);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
    }

    #[test]
    fn test_read_u16() {
        let bus = Box::new(TestBus::new());
        let mut cpu = Cpu::new(bus);
        cpu.write(0x10, 0x20);
        cpu.write(0x11, 0x30);

        assert_eq!(cpu.read_u16(0x10), 0x3020);
    }

    #[test]
    fn test_read_write() {
        let bus = Box::new(TestBus::new());
        let mut cpu = Cpu::new(bus);

        cpu.write(0x10, 0x20);

        assert_eq!(cpu.read(0x10), 0x20);
    }
}