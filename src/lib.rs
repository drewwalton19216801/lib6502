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
use crate::instructions::INSTRUCTION_LIST;

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

    /// The bus that the CPU is connected to
    pub bus: Box<dyn Bus>,

    /// The variant of the CPU
    pub variant: Variant,

    /// Whether illegal instructions are allowed
    pub enable_illegal_opcodes: bool,

    /// The current instruction string
    pub current_instruction_string: String,

    /// Whether debug mode is enabled
    pub debug: bool,
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

/// CPU variants
#[derive(Clone, Copy, PartialEq)]
pub enum Variant {
    /// Modified 65C02 (no ROR bug)
    CMOS,
    /// Modified 2A03 (no decimal mode)
    NES,
    /// Original 6502 (with ROR bug)
    NMOS,
}

/// Variant implementation
#[allow(dead_code)]
impl Variant {
    /// Returns a new Variant from a string
    pub fn from_string(variant: String) -> Self {
        match variant.as_str() {
            "NMOS" => return Self::NMOS,
            "CMOS" => return Self::CMOS,
            "NES" => return Self::NES,
            _ => panic!("Invalid CPU variant"),
        }
    }

    /// Returns a string representation of the variant
    pub fn to_string(&self) -> String {
        match self {
            Self::NMOS => return String::from("NMOS"),
            Self::CMOS => return String::from("CMOS"),
            Self::NES => return String::from("NES"),
        }
    }
}

impl Cpu {
    /// Create a new CPU
    pub fn new(bus: Box<dyn Bus>, debug: bool) -> Cpu {
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
            bus,

            variant: Variant::CMOS,
            enable_illegal_opcodes: false,
            current_instruction_string: String::new(),
            debug
        }
    }

    /// Reset the CPU
    pub fn reset(&mut self) {
        if self.debug {
            println!("CPU: Reset");
        }
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xfd;
        self.pc = self.read_u16(RESET_VECTOR);
        self.status = StatusFlags::None.bits() | StatusFlags::Unused.bits() | StatusFlags::InterruptDisable.bits();
        self.cycles = 8;
        self.current_instruction_string = "RESET".to_string();
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

    /// Write a 16-bit word to memory
    pub fn write_u16(&mut self, address: u16, value: u16) {
        self.write(address, (value & 0xff) as u8);
        self.write(address + 1, ((value >> 8) & 0xff) as u8);
    }

    fn set_flag(&mut self, flag: StatusFlags, value: bool) {
        if value {
            self.status |= flag.bits();
        } else {
            self.status &= !flag.bits();
        }
    }

    fn get_flag(&self, flag: StatusFlags) -> bool {
        (self.status & flag.bits()) != 0
    }

    fn increment_sp(&mut self) {
        self.sp = self.sp.wrapping_add(1);
    }

    fn decrement_sp(&mut self) {
        self.sp = self.sp.wrapping_sub(1);
    }

    /// Set the CPU variant
    pub fn change_variant(&mut self, variant: Variant) {
        self.variant = variant;
    }

    /// Set the enable_illegal_opcodes flag
    pub fn set_illegal_opcodes(&mut self, value: bool) {
        self.enable_illegal_opcodes = value;
    }

    fn get_status_string(&self) -> String {
        let mut status = String::new();
        status.push_str("STATUS: ");
        status.push_str(if self.get_flag(StatusFlags::Negative) {
            "N"
        } else {
            "n"
        });
        status.push_str(if self.get_flag(StatusFlags::Overflow) {
            "V"
        } else {
            "v"
        });
        status.push_str("-");
        status.push_str(if self.get_flag(StatusFlags::Break) {
            "B"
        } else {
            "b"
        });
        status.push_str(if self.get_flag(StatusFlags::Decimal) {
            "D"
        } else {
            "d"
        });
        status.push_str(if self.get_flag(StatusFlags::InterruptDisable) {
            "I"
        } else {
            "i"
        });
        status.push_str(if self.get_flag(StatusFlags::Zero) { "Z" } else { "z" });
        status.push_str(if self.get_flag(StatusFlags::Carry) {
            "C"
        } else {
            "c"
        });
        status
    }

    fn fetch(&mut self) -> u8 {
        if self.addr_mode != AddressingMode::Implied {
            self.fetched_data = self.read(self.addr_abs);
        }
        self.fetched_data
    }

    fn push(&mut self, value: u8) {
        self.write(0x100 + self.sp as u16, value);
        self.decrement_sp();
    }

    fn push_word(&mut self, value: u16) {
        self.push(((value >> 8) & 0xff) as u8);
        self.push((value & 0xff) as u8);
    }

    fn pop(&mut self) -> u8 {
        self.increment_sp();
        self.read(0x100 + self.sp as u16)
    }

    fn pop_word(&mut self) -> u16 {
        let lo = self.pop() as u16;
        let hi = self.pop() as u16;
        (hi << 8) | lo
    }

    fn execute_instruction(&mut self, opcode: u8) -> u8 {
        let instruction = &INSTRUCTION_LIST[opcode as usize];
        (instruction.function)(self)
    }

    fn get_operand_string(&mut self, mode: AddressingMode, address: u16) -> String {
        match mode {
            AddressingMode::None => return String::from(""),
            AddressingMode::Implied => return String::from(""),
            AddressingMode::Immediate => return format!("#${:02X}", self.read(address)),
            AddressingMode::ZeroPage => return format!("${:02X}", self.read(address)),
            AddressingMode::ZeroPageX => return format!("${:02X},X", self.read(address)),
            AddressingMode::ZeroPageY => return format!("${:02X},Y", self.read(address)),
            AddressingMode::Relative => return format!("${:02X}", self.read(address)),
            AddressingMode::Absolute => return format!("${:04X}", self.read_u16(address)),
            AddressingMode::AbsoluteX => return format!("${:04X},X", self.read_u16(address)),
            AddressingMode::AbsoluteY => return format!("${:04X},Y", self.read_u16(address)),
            AddressingMode::Indirect => return format!("(${:04X})", self.read_u16(address)),
            AddressingMode::IndexedIndirect => return format!("(${:02X},X)", self.read(address)),
            AddressingMode::IndirectIndexed => return format!("(${:02X}),Y", self.read(address)),
        }
    }

    fn disassemble_instruction_at(&mut self, from_pc: u16) -> String {
        let opcode = self.read(from_pc);
        let instruction = &INSTRUCTION_LIST[opcode as usize];
        let addr_mode = instructions::get_addr_mode(opcode);
        let addr_str = self.get_operand_string(addr_mode, from_pc + 1);
        format!("{} {}", instruction.name, addr_str)
    }

    fn execute_addr_mode(&mut self, mode: AddressingMode) -> u8 {
        self.addr_mode = mode;
        let extra_cycle = mode.execute(self);
        if extra_cycle {
            return 1;
        }
        0
    }

    /// Get the number of cycles for an instruction
    pub fn get_cycles(&self, opcode: u8) -> u8 {
        return instructions::get_cycles(opcode);
    }

    fn do_interrupt(&mut self, vector: u16) {
        self.push_word(self.pc);
        self.set_flag(StatusFlags::Break, false);
        self.set_flag(StatusFlags::Unused, true);
        self.set_flag(StatusFlags::Break, true);
        self.set_flag(StatusFlags::InterruptDisable, true);
        self.push(self.status);
        self.set_flag(StatusFlags::InterruptDisable, false);
        self.pc = self.read_u16(vector);
        self.cycles = 7;
    }

    /// Interrupt request
    #[allow(dead_code)]
    pub fn irq(&mut self) {
        if !self.get_flag(StatusFlags::InterruptDisable) {
            self.do_interrupt(addresses::IRQ_VECTOR);
        }
    }

    /// Non-maskable interrupt request
    #[allow(dead_code)]
    pub fn nmi(&mut self) {
        self.do_interrupt(addresses::NMI_VECTOR);
    }

    fn get_register(&self, register: &str) -> u8 {
        match register {
            "A" => self.a,
            "X" => self.x,
            "Y" => self.y,
            "SP" => self.sp,
            _ => panic!("Invalid register: {}", register),
        }
    }

    fn set_zn_flags(&mut self, register: &str) {
        self.set_flag(StatusFlags::Zero, self.get_register(register) == 0);
        self.set_flag(StatusFlags::Negative, self.get_register(register) & 0x80 != 0);
    }

    /// Clock the CPU
    pub fn clock(&mut self) {
        if self.cycles == 0 {
            self.current_instruction_string = self.disassemble_instruction_at(self.pc);
            if self.debug {
                println!("CPU insn: {}", self.current_instruction_string);
                println!("CPU pre-op: {}", self.get_state());
            }
            self.opcode = self.read(self.pc);
            self.pc += 1;
            self.cycles = self.get_cycles(self.opcode);
            self.addr_mode = instructions::get_addr_mode(self.opcode);
            let cycles_addr = self.execute_addr_mode(self.addr_mode);
            let cycles_instruction = self.execute_instruction(self.opcode);
            self.cycles += cycles_addr + cycles_instruction;
            if self.debug {
                println!("CPU post-op: {}", self.get_state());
            }
        }
        self.cycles -= 1;
    }

    /// Get the state of the CPU
    pub fn get_state(&self) -> String {
        format!(
            "A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} PC:{:04X}",
            self.a, self.x, self.y, self.status, self.sp, self.pc
        )
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
        let mut cpu = Cpu::new(bus, false);
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
        let mut cpu = Cpu::new(bus, false);
        cpu.write(0x10, 0x20);
        cpu.write(0x11, 0x30);

        assert_eq!(cpu.read_u16(0x10), 0x3020);
    }

    #[test]
    fn test_write_u16() {
        let bus = Box::new(TestBus::new());
        let mut cpu = Cpu::new(bus, false);
        cpu.write_u16(0x10, 0x3020);

        assert_eq!(cpu.read(0x10), 0x20);
        assert_eq!(cpu.read(0x11), 0x30);
    }

    #[test]
    fn test_read_write() {
        let bus = Box::new(TestBus::new());
        let mut cpu = Cpu::new(bus, false);

        cpu.write(0x10, 0x20);

        assert_eq!(cpu.read(0x10), 0x20);
    }
}