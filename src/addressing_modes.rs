//! The `addressing_modes` module contains implementations of the 6502 CPU addressing modes.

use crate::cpu::CPU;
use crate::bus::Bus;

/// A type alias for an addressing mode function.
/// The function takes a mutable reference to a `CPU` instance as an argument,
/// and returns a tuple containing the address and the number of additional cycles.
///
/// The address is the memory address that the instruction should be executed on.
/// The additional cycles are the number of cycles that the addressing mode adds
/// to the instruction's base cycle count.
pub type AddressingMode<B> = fn(&mut CPU<B>) -> (u16, u8);

/// The Accumulator addressing mode. This mode is used by instructions that
/// only operate on the Accumulator.
///
/// # Returns
///
/// A tuple containing the address (always 0) and the number of additional cycles
/// (always 0).
pub fn accumulator<B: Bus>(_cpu: &mut CPU<B>) -> (u16, u8) {
    (0, 0)
}

/// The Absolute addressing mode. This mode is used by instructions that
/// operate on an absolute memory address.
///
/// # Returns
///
/// A tuple containing the address (the absolute memory address that the
/// instruction should be executed on) and the number of additional cycles (always 0).
pub fn absolute<B: Bus>(cpu: &mut CPU<B>) -> (u16, u8) {
    let addr = cpu.fetch_word();
    (addr, 0)
}

/// The Absolute X addressing mode. This mode is used by instructions that
/// operate on an absolute memory address plus the value of the X register.
///
/// # Returns
///
/// A tuple containing the address (the absolute memory address plus the value
/// of the X register) and the number of additional cycles (always 0 or 1).
pub fn absolute_x<B: Bus>(cpu: &mut CPU<B>) -> (u16, u8) {
    // Fetch the absolute memory address
    let base = cpu.fetch_word();
    // Calculate the address by adding the value of the X register
    let addr = base.wrapping_add(cpu.registers.x as u16);
    // Check if a page boundary was crossed
    let page_cross = (base & 0xFF00) != (addr & 0xFF00);
    // If a page boundary was crossed, add one cycle to the instruction
    let additional_cycles = if page_cross { 1 } else { 0 };
    // Return the address and additional cycles
    (addr, additional_cycles)
}

/// The Absolute Y addressing mode. This mode is used by instructions that
/// operate on an absolute memory address plus the value of the Y register.
///
/// # Returns
///
/// A tuple containing the address (the absolute memory address plus the value
/// of the Y register) and the number of additional cycles (always 0 or 1).
pub fn absolute_y<B: Bus>(cpu: &mut CPU<B>) -> (u16, u8) {
    // Fetch the absolute memory address
    let base = cpu.fetch_word();
    // Calculate the address by adding the value of the Y register
    let addr = base.wrapping_add(cpu.registers.y as u16);
    // Check if a page boundary was crossed
    let page_cross = (base & 0xFF00) != (addr & 0xFF00);
    // If a page boundary was crossed, add one cycle to the instruction
    let additional_cycles = if page_cross { 1 } else { 0 };
    // Return the address and additional cycles
    (addr, additional_cycles)
}

/// The Immediate addressing mode. This mode is used by instructions that
/// operate on an immediate value.
///
/// # Returns
///
/// A tuple containing the address (the current PC) and the number of additional
/// cycles (always 0).
pub fn immediate<B: Bus>(cpu: &mut CPU<B>) -> (u16, u8) {
    // Save the current PC
    let addr = cpu.registers.pc;
    // Increment the PC to the next instruction
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    // Return the address and 0 additional cycles
    (addr, 0)
}

/// The Implied addressing mode. This mode is used by instructions that do not
/// use an operand.
///
/// # Returns
///
/// A tuple containing the address (always 0) and the number of additional cycles
/// (always 0).
pub fn implied<B: Bus>(_cpu: &mut CPU<B>) -> (u16, u8) {
    // The implied addressing mode does not use an operand, so the address is
    // always 0. The instruction also does not add any additional cycles.
    (0, 0)
}

/// The Indirect addressing mode. This mode is used by instructions that operate
/// on a memory address which is stored at another address.
///
/// # Returns
///
/// A tuple containing the address and the number of additional cycles (always 0).
pub fn indirect<B: Bus>(cpu: &mut CPU<B>) -> (u16, u8) {
    // Fetch the address of the memory address to be read
    let ptr = cpu.fetch_word();
    // Read the low byte of the memory address
    let lo = cpu.bus.read(ptr) as u16;
    // Read the high byte of the memory address
    // If the low byte of the pointer is 0xFF, the high byte is stored at the
    // first byte of the page. This is a bug in the original 6502.
    let hi_address = if (ptr & 0x00FF) == 0x00FF {
        ptr & 0xFF00
    } else {
        ptr + 1
    };
    let hi = cpu.bus.read(hi_address) as u16;
    // Calculate the address from the low and high bytes
    let addr = (hi << 8) | lo;
    // Return the address and 0 additional cycles
    (addr, 0)
}

/// The Indirect X addressing mode. This mode is used by instructions that
/// operate on a memory address which is stored at the address in the X
/// register plus the value of the X register.
///
/// # Returns
///
/// A tuple containing the address and the number of additional cycles (always 0).
pub fn indirect_x<B: Bus>(cpu: &mut CPU<B>) -> (u16, u8) {
    // Fetch the address of the memory address to be read
    let ptr = cpu.fetch_byte().wrapping_add(cpu.registers.x);
    // Read the low byte of the memory address
    let lo = cpu.bus.read(ptr as u16) as u16;
    // Read the high byte of the memory address
    let hi = cpu.bus.read(ptr.wrapping_add(1) as u16) as u16;
    // Calculate the address from the low and high bytes
    let addr = (hi << 8) | lo;
    // Return the address and 0 additional cycles
    (addr, 0)
}

/// The Indirect Y addressing mode. This mode is used by instructions that
/// operate on a memory address which is stored at the address in the Y
/// register plus the value of the Y register.
///
/// # Returns
///
/// A tuple containing the address and the number of additional cycles. If a
/// page boundary was crossed, one additional cycle is added.
pub fn indirect_y<B: Bus>(cpu: &mut CPU<B>) -> (u16, u8) {
    // Fetch the address of the memory address to be read
    let ptr = cpu.fetch_byte();
    // Read the low byte of the memory address
    let lo = cpu.bus.read(ptr as u16) as u16;
    // Read the high byte of the memory address
    let hi = cpu.bus.read(ptr.wrapping_add(1) as u16) as u16;
    // Calculate the base address from the low and high bytes
    let base_addr = (hi << 8) | lo;
    // Calculate the address by adding the value of the Y register
    let addr = base_addr.wrapping_add(cpu.registers.y as u16);
    // Check if a page boundary was crossed
    let page_cross = (base_addr & 0xFF00) != (addr & 0xFF00);
    // If a page boundary was crossed, add one additional cycle
    let additional_cycles = if page_cross { 1 } else { 0 };
    // Return the address and additional cycles
    (addr, additional_cycles)
}

/// The Relative addressing mode. This mode is used by branch instructions to
/// jump to an address relative to the current program counter.
///
/// # Returns
///
/// A tuple containing the address (the current PC plus the signed offset) and
/// the number of additional cycles (always 0).
pub fn relative<B: Bus>(cpu: &mut CPU<B>) -> (u16, u8) {
    // Fetch the signed offset byte
    let offset = cpu.fetch_byte() as i8;
    // Calculate the address by adding the signed offset to the current PC
    let pc = cpu.registers.pc;
    let addr = pc.wrapping_add(offset as u16);
    // Return the address and 0 additional cycles
    (addr, 0)
}

/// The Zero Page addressing mode. This mode is used by instructions that
/// operate on a memory address within the first 256 bytes of memory.
///
/// # Returns
///
/// A tuple containing the address (the zero page address) and the number of
/// additional cycles (always 0).
pub fn zero_page<B: Bus>(cpu: &mut CPU<B>) -> (u16, u8) {
    // Fetch the zero page address from the next byte in memory
    let addr = cpu.fetch_byte() as u16;
    // Return the zero page address and 0 additional cycles
    (addr, 0)
}

/// The Zero Page X addressing mode. This mode is used by instructions that
/// operate on a memory address within the first 256 bytes of memory, with the
/// X register being added to the zero page address.
///
/// # Returns
///
/// A tuple containing the address (the zero page address plus the X register)
/// and the number of additional cycles (always 0).
pub fn zero_page_x<B: Bus>(cpu: &mut CPU<B>) -> (u16, u8) {
    // Fetch the zero page address from the next byte in memory
    let addr = cpu.fetch_byte().wrapping_add(cpu.registers.x) as u16;
    // Return the zero page address plus the X register and 0 additional cycles
    (addr, 0)
}

/// The Zero Page Y addressing mode. This mode is used by instructions that
/// operate on a memory address within the first 256 bytes of memory, with the
/// Y register being added to the zero page address.
///
/// # Returns
///
/// A tuple containing the address (the zero page address plus the Y register)
/// and the number of additional cycles (always 0).
pub fn zero_page_y<B: Bus>(cpu: &mut CPU<B>) -> (u16, u8) {
    // Fetch the zero page address from the next byte in memory
    let addr = cpu.fetch_byte().wrapping_add(cpu.registers.y) as u16;
    // Return the zero page address plus the Y register and 0 additional cycles
    (addr, 0)
}
