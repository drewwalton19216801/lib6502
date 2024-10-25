use crate::cpu::CPU;
use crate::bus::Bus;

/// Type alias for an instruction handler function.
/// Returns any additional cycles the instruction may add.
pub type Instruction<B> = fn(&mut CPU<B>, u16) -> u8;

/// Add with Carry
pub fn adc<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    let value = cpu.bus.read(addr);
    let a = cpu.registers.a;
    let carry_in = if cpu.registers.status.carry { 1 } else { 0 };
    let mut additional_cycles = 0;

    if cpu.registers.status.decimal_mode {
        // BCD (Binary Coded Decimal) Addition
        let mut al = (a & 0x0F) + (value & 0x0F) + carry_in;
        let mut ah = (a >> 4) + (value >> 4);

        if al > 9 {
            al += 6;
        }

        if al > 0x0F {
            ah += 1;
            al &= 0x0F;
        }

        if ah > 9 {
            ah += 6;
        }

        let result = ((ah << 4) | (al & 0x0F)) as u8;
        cpu.registers.status.carry = ah > 0x0F;
        cpu.registers.status.zero = result == 0;
        cpu.registers.status.negative = (result & 0x80) != 0;
        // Note: The overflow flag in decimal mode is undefined on the 6502 and can be ignored
        cpu.registers.a = result;
        additional_cycles = 1; // Decimal mode adds an extra cycle
    } else {
        // Binary Addition
        let sum = (a as u16) + (value as u16) + (carry_in as u16);
        let result = sum as u8;

        // Update flags
        cpu.registers.status.carry = sum > 0xFF;
        cpu.registers.status.zero = result == 0;
        cpu.registers.status.negative = (result & 0x80) != 0;
        cpu.registers.status.overflow = ((!(a ^ value) & (a ^ result)) & 0x80) != 0;
        cpu.registers.a = result;
    }

    additional_cycles
}

/// AND with Accumulator
pub fn and<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    let value = cpu.bus.read(addr);
    cpu.registers.a &= value;
    cpu.update_zero_and_negative_flags(cpu.registers.a);
    0 // No additional cycles
}

/// Arithmetic Shift Left
pub fn asl<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    let value = cpu.bus.read(addr);
    let result = value << 1;
    cpu.bus.write(addr, result);
    cpu.registers.status.carry = (value & 0x80) != 0;
    cpu.update_zero_and_negative_flags(result);
    0 // No additional cycles
}

/// Branch if Carry Clear
pub fn bcc<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    if !cpu.registers.status.carry {
        cpu.branch(addr)
    } else {
        0 // No additional cycles if branch not taken
    }
}

/// Branch if Carry Set
pub fn bcs<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    if cpu.registers.status.carry {
        cpu.branch(addr)
    } else {
        0 // No additional cycles if branch not taken
    }
}

/// Branch if Equal
pub fn beq<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    if cpu.registers.status.zero {
        cpu.branch(addr)
    } else {
        0 // No additional cycles if branch not taken
    }
}

/// Bit Test
pub fn bit<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    let value = cpu.bus.read(addr);
    let result = cpu.registers.a & value;
    cpu.registers.status.zero = result == 0;
    cpu.registers.status.overflow = (value & 0x40) != 0;
    cpu.registers.status.negative = (value & 0x80) != 0;
    0 // No additional cycles
}

/// Clear Carry Flag
pub fn clc<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    cpu.registers.status.carry = false;
    0
}

/// Clear Decimal Flag
pub fn cld<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    cpu.registers.status.decimal_mode = false;
    0
}

/// Load Accumulator
pub fn lda<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    let value = cpu.bus.read(addr);
    cpu.registers.a = value;
    cpu.update_zero_and_negative_flags(cpu.registers.a);
    0 // No additional cycles
}

/// Set Carry Flag
pub fn sec<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    cpu.registers.status.carry = true;
    0
}

/// Set Decimal Flag
pub fn sed<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    cpu.registers.status.decimal_mode = true;
    0
}

// Implement other instructions similarly...
