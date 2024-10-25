use crate::cpu::CPU;
use crate::bus::Bus;

/// Type alias for an addressing mode function.
/// Returns a tuple of the address and an optional additional cycle count.
pub type AddressingMode<B> = fn(&mut CPU<B>) -> (u16, u8);

/// Accumulator Addressing Mode
pub fn accumulator<B: Bus>(_cpu: &mut CPU<B>) -> (u16, u8) {
    (0, 0)
}

/// Absolute Addressing Mode
pub fn absolute<B: Bus>(cpu: &mut CPU<B>) -> (u16, u8) {
    let addr = cpu.fetch_word();
    (addr, 0)
}

/// Absolute,X Addressing Mode
pub fn absolute_x<B: Bus>(cpu: &mut CPU<B>) -> (u16, u8) {
    let base = cpu.fetch_word();
    let addr = base.wrapping_add(cpu.registers.x as u16);

    // Check for page boundary crossing
    let page_cross = (base & 0xFF00) != (addr & 0xFF00);
    let additional_cycles = if page_cross { 1 } else { 0 };

    (addr, additional_cycles)
}

/// Absolute,Y Addressing Mode
pub fn absolute_y<B: Bus>(cpu: &mut CPU<B>) -> (u16, u8) {
    let base = cpu.fetch_word();
    let addr = base.wrapping_add(cpu.registers.y as u16);

    // Check for page boundary crossing
    let page_cross = (base & 0xFF00) != (addr & 0xFF00);
    let additional_cycles = if page_cross { 1 } else { 0 };

    (addr, additional_cycles)
}

/// Immediate Addressing Mode
pub fn immediate<B: Bus>(cpu: &mut CPU<B>) -> (u16, u8) {
    let addr = cpu.registers.pc;
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    (addr, 0)
}

/// Implied Addressing Mode (used for instructions that do not require an operand)
pub fn implied<B: Bus>(_cpu: &mut CPU<B>) -> (u16, u8) {
    // Implied mode does not use any address or additional cycles
    (0, 0)
}

/// Indirect Addressing Mode
pub fn indirect<B: Bus>(cpu: &mut CPU<B>) -> (u16, u8) {
    let ptr = cpu.fetch_word();

    let lo = cpu.bus.read(ptr) as u16;

    // Simulate the 6502 page boundary bug
    let hi_address = if (ptr & 0x00FF) == 0x00FF {
        ptr & 0xFF00 // Wrap around to the beginning of the same page
    } else {
        ptr + 1 // Normal case, increment to the next byte
    };

    let hi = cpu.bus.read(hi_address) as u16;

    let addr = (hi << 8) | lo;
    (addr, 0)
}

/// Indirect,X Addressing Mode (Indexed Indirect)
pub fn indirect_x<B: Bus>(cpu: &mut CPU<B>) -> (u16, u8) {
    let ptr = cpu.fetch_byte().wrapping_add(cpu.registers.x);
    let lo = cpu.bus.read(ptr as u16) as u16;
    let hi = cpu.bus.read(ptr.wrapping_add(1) as u16) as u16;
    let addr = (hi << 8) | lo;
    (addr, 0)
}

/// Indirect,Y Addressing Mode (Indirect Indexed)
pub fn indirect_y<B: Bus>(cpu: &mut CPU<B>) -> (u16, u8) {
    let ptr = cpu.fetch_byte();
    let lo = cpu.bus.read(ptr as u16) as u16;
    let hi = cpu.bus.read(ptr.wrapping_add(1) as u16) as u16;
    let base_addr = (hi << 8) | lo;
    let addr = base_addr.wrapping_add(cpu.registers.y as u16);

    // Check for page boundary crossing
    let page_cross = (base_addr & 0xFF00) != (addr & 0xFF00);
    let additional_cycles = if page_cross { 1 } else { 0 };

    (addr, additional_cycles)
}

/// Relative Addressing Mode
pub fn relative<B: Bus>(cpu: &mut CPU<B>) -> (u16, u8) {
    let offset = cpu.fetch_byte() as i8; // Fetch the signed 8-bit offset
    let pc = cpu.registers.pc;
    let addr = pc.wrapping_add(offset as u16);
    (addr, 0) // No additional cycles from the addressing mode itself
}

/// Zero Page Addressing Mode
pub fn zero_page<B: Bus>(cpu: &mut CPU<B>) -> (u16, u8) {
    let addr = cpu.fetch_byte() as u16;
    (addr, 0)
}

/// Zero Page,X Addressing Mode
pub fn zero_page_x<B: Bus>(cpu: &mut CPU<B>) -> (u16, u8) {
    let addr = cpu.fetch_byte().wrapping_add(cpu.registers.x) as u16;
    (addr, 0)
}

/// Zero Page,Y Addressing Mode
pub fn zero_page_y<B: Bus>(cpu: &mut CPU<B>) -> (u16, u8) {
    let addr = cpu.fetch_byte().wrapping_add(cpu.registers.y) as u16;
    (addr, 0)
}