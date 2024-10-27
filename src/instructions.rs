//! The `instructions` module contains the implementation of the 6502 CPU instructions.
//! 
//! Unimplemented instructions are intentionally left undocumented.

use crate::cpu::CPU;
use crate::bus::Bus;

/// A type alias for an instruction function.
///
/// The function takes a mutable reference to a `CPU` instance and a memory address as arguments,
/// and returns the number of additional cycles that the instruction adds to the instruction's
/// base cycle count.
pub type Instruction<B> = fn(&mut CPU<B>, u16) -> u8;

/// ADC - Add with Carry
///
/// The ADC instruction adds the value of the memory at the given address to the
/// accumulator, taking into account the carry flag.
///
/// If the decimal mode flag is set, the instruction adds the values as BCD
/// values. Otherwise it adds the values as binary values.
///
/// # Returns
///
/// The number of additional cycles that the instruction adds to the instruction's
/// base cycle count.
pub fn adc<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    let value = cpu.bus.read(addr);
    let a = cpu.registers.a;
    let carry_in = if cpu.registers.status.carry { 1 } else { 0 };
    let mut additional_cycles = 0;

    if cpu.registers.status.decimal_mode {
        // Add the values as BCD values
        let mut al = (a & 0x0F) + (value & 0x0F) + carry_in;
        let mut ah = (a >> 4) + (value >> 4);

        // If the lower nibble is greater than 9, add 6 to carry the value
        // to the next digit. This is done because the range of the lower nibble
        // is 0-9, not 0-F.
        if al > 9 {
            al += 6;
        }

        // If the lower nibble is greater than 0xF, add 1 to the higher nibble
        // and mask the lower nibble to 0-9.
        if al > 0x0F {
            ah += 1;
            al &= 0x0F;
        }

        // If the higher nibble is greater than 9, add 6 to carry the value
        // to the next digit. This is done because the range of the higher nibble
        // is 0-9, not 0-F.
        if ah > 9 {
            ah += 6;
        }

        let result = ((ah << 4) | (al & 0x0F)) as u8;
        cpu.registers.status.carry = ah > 0x0F;
        cpu.registers.status.zero = result == 0;
        cpu.registers.status.negative = (result & 0x80) != 0;
        // Note: The overflow flag in decimal mode is undefined on the 6502 and can be ignored
        cpu.registers.a = result;
        additional_cycles = 1;
    } else {
        // Add the values as binary values
        let sum = (a as u16) + (value as u16) + (carry_in as u16);
        let result = sum as u8;

        cpu.registers.status.carry = sum > 0xFF;
        cpu.registers.status.zero = result == 0;
        cpu.registers.status.negative = (result & 0x80) != 0;
        cpu.registers.status.overflow = ((!(a ^ value) & (a ^ result)) & 0x80) != 0;
        cpu.registers.a = result;
    }

    additional_cycles
}

/// AND - Logical AND
///
/// Performs a logical AND on the accumulator and the value at the given
/// address, storing the result in the accumulator.
///
/// # Returns
///
/// The number of additional cycles that the instruction adds to the
/// instruction's base cycle count.
pub fn and<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    let value = cpu.bus.read(addr);
    cpu.registers.a &= value;
    cpu.update_zero_and_negative_flags(cpu.registers.a);
    0
}

/// ASL - Arithmetic Shift Left
///
/// This instruction shifts the bits in the memory location at the given
/// address one position to the left. The bit that was shifted out is stored
/// in the carry flag, and the result is stored back into the memory location.
///
/// The zero and negative flags are updated based on the result.
///
/// # Returns
///
/// The number of additional cycles that the instruction adds to the
/// instruction's base cycle count.
pub fn asl<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Read the value from the specified address
    let value = cpu.bus.read(addr);
    // Shift the value left by one bit
    let result = value << 1;
    // Write the result back to the specified address
    cpu.bus.write(addr, result);
    // Set the carry flag if the high bit of the original value was set
    cpu.registers.status.carry = (value & 0x80) != 0;
    // Update the zero and negative flags based on the result
    cpu.update_zero_and_negative_flags(result);
    // Return the additional cycle count (0 in this case)
    0
}

/// BCC - Branch if Carry Clear
///
/// This function checks if the carry flag is clear (i.e., false) and branches 
/// to the specified address if it is. If the carry flag is set, it does not 
/// branch and returns 0 additional cycles.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - The address to potentially branch to.
///
/// # Returns
///
/// The number of additional cycles incurred by the branch operation (1 or 2
/// if a branch is taken and a page boundary is crossed, otherwise 0).
pub fn bcc<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Check if the carry flag is clear
    if !cpu.registers.status.carry {
        // Branch to the specified address
        cpu.branch(addr)
    } else {
        // No branch taken, return 0 additional cycles
        0
    }
}

/// BCS - Branch if Carry Set
///
/// This function checks if the carry flag is set and branches to the specified
/// address if it is. If the carry flag is clear, it does not branch and returns
/// 0 additional cycles.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - The address to potentially branch to.
///
/// # Returns
///
/// The number of additional cycles incurred by the branch operation (1 or 2
/// if a branch is taken and a page boundary is crossed, otherwise 0).
pub fn bcs<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Check if the carry flag is set
    if cpu.registers.status.carry {
        // Branch to the specified address
        cpu.branch(addr)
    } else {
        // No branch taken, return 0 additional cycles
        0
    }
}

/// BEQ - Branch if Equal
///
/// This function checks if the zero flag is set and branches to the specified
/// address if it is. If the zero flag is clear, it does not branch and returns
/// 0 additional cycles.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - The address to potentially branch to.
///
/// # Returns
///
/// The number of additional cycles incurred by the branch operation (1 or 2
/// if a branch is taken and a page boundary is crossed, otherwise 0).
pub fn beq<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Check if the zero flag is set
    if cpu.registers.status.zero {
        // Branch to the specified address
        cpu.branch(addr)
    } else {
        // No branch taken, return 0 additional cycles
        0
    }
}

/// BIT - Bit Test
///
/// This instruction performs a logical AND on the accumulator and the value
/// at the given address. The result is discarded, and the zero and negative
/// flags are set based on the result. The overflow flag is set based on the
/// value at the given address.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - The address to read from.
///
/// # Returns
///
/// The number of additional cycles that the instruction adds to the
/// instruction's base cycle count.
pub fn bit<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    let value = cpu.bus.read(addr);
    let result = cpu.registers.a & value;
    cpu.registers.status.zero = result == 0;
    cpu.registers.status.overflow = (value & 0x40) != 0;
    cpu.registers.status.negative = (value & 0x80) != 0;
    0
}

/// BMI - Branch if Negative
///
/// This function checks if the negative flag is set and branches to the specified
/// address if it is. If the negative flag is clear, it does not branch and returns
/// 0 additional cycles.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - The address to potentially branch to.
///
/// # Returns
///
/// The number of additional cycles incurred by the branch operation (1 or 2
/// if a branch is taken and a page boundary is crossed, otherwise 0).
pub fn bmi<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Check if the negative flag is set
    if cpu.registers.status.negative {
        // Branch to the specified address
        cpu.branch(addr)
    } else {
        // No branch taken, return 0 additional cycles
        0
    }
}

/// BNE - Branch if Not Equal
///
/// This function checks if the zero flag is clear and branches to the specified
/// address if it is. If the zero flag is set, it does not branch and returns
/// 0 additional cycles.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - The address to potentially branch to.
///
/// # Returns
///
/// The number of additional cycles incurred by the branch operation (1 or 2
/// if a branch is taken and a page boundary is crossed, otherwise 0).
pub fn bne<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Check if the zero flag is clear
    if !cpu.registers.status.zero {
        // Branch to the specified address
        cpu.branch(addr)
    } else {
        // No branch taken, return 0 additional cycles
        0
    }
}

/// BPL - Branch if Positive
///
/// This function checks if the negative flag is clear and branches to the specified
/// address if it is. If the negative flag is set, it does not branch and returns
/// 0 additional cycles.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - The address to potentially branch to.
///
/// # Returns
///
/// The number of additional cycles incurred by the branch operation (1 or 2
/// if a branch is taken and a page boundary is crossed, otherwise 0).
pub fn bpl<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Check if the negative flag is clear
    if !cpu.registers.status.negative {
        // Branch to the specified address
        cpu.branch(addr)
    } else {
        // No branch taken, return 0 additional cycles
        0
    }
}

/// BRK - Force Interrupt
///
/// This instruction simulates an interrupt request. It increments the program
/// counter, pushes the program counter and status register onto the stack,
/// disables further interrupts, and jumps to the interrupt vector address.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `_addr` - This argument is unused in the BRK instruction.
///
/// # Returns
///
/// The number of additional cycles that the instruction adds to the instruction's
/// base cycle count (always 0 for BRK).
pub fn brk<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    // Increment the program counter
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    
    // Push the program counter onto the stack (high byte first)
    cpu.stack_push((cpu.registers.pc >> 8) as u8);
    cpu.stack_push((cpu.registers.pc & 0xFF) as u8);
    
    // Prepare the status register with the B and U flags set
    let mut status = cpu.registers.status.to_byte();
    status |= 0x10; // B flag
    status |= 0x20; // U flag
    cpu.stack_push(status);
    
    // Disable interrupts
    cpu.registers.status.interrupt_disable = true;
    
    // Jump to the interrupt vector address
    let lo = cpu.bus.read(0xFFFE) as u16;
    let hi = cpu.bus.read(0xFFFF) as u16;
    cpu.registers.pc = (hi << 8) | lo;
    
    // Return 0 additional cycles
    0
}

/// BVC - Branch if Overflow Clear
///
/// This function checks if the overflow flag is clear and branches to the
/// specified address if it is. If the overflow flag is set, it does not branch
/// and returns 0 additional cycles.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - The address to potentially branch to.
///
/// # Returns
///
/// The number of additional cycles incurred by the branch operation (1 or 2
/// if a branch is taken and a page boundary is crossed, otherwise 0).
pub fn bvc<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    if !cpu.registers.status.overflow {
        // Branch to the specified address
        cpu.branch(addr)
    } else {
        // No branch taken, return 0 additional cycles
        0
    }
}

/// BVS - Branch if Overflow Set
///
/// This function checks if the overflow flag is set and branches to the
/// specified address if it is. If the overflow flag is clear, it does not branch
/// and returns 0 additional cycles.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - The address to potentially branch to.
///
/// # Returns
///
/// The number of additional cycles incurred by the branch operation (1 or 2
/// if a branch is taken and a page boundary is crossed, otherwise 0).
pub fn bvs<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    if cpu.registers.status.overflow {
        // Branch to the specified address
        cpu.branch(addr)
    } else {
        // No branch taken, return 0 additional cycles
        0
    }
}

/// CLC - Clear Carry Flag
///
/// This instruction clears the carry flag.
///
/// # Returns
///
/// The number of additional cycles incurred by this instruction (0).
pub fn clc<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    // Clear the carry flag
    cpu.registers.status.carry = false;
    // Return 0 additional cycles
    0
}

/// CLD - Clear Decimal Mode
///
/// This instruction clears the decimal mode flag.
///
/// # Returns
///
/// The number of additional cycles incurred by this instruction (0).
pub fn cld<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    // Clear the decimal mode flag
    cpu.registers.status.decimal_mode = false;
    // Return 0 additional cycles
    0
}

/// CLI - Clear Interrupt Disable
///
/// This instruction clears the interrupt disable flag.
///
/// # Returns
///
/// The number of additional cycles incurred by this instruction (0).
pub fn cli<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    // Clear the interrupt disable flag
    cpu.registers.status.interrupt_disable = false;
    // Return 0 additional cycles
    0
}

/// CLV - Clear Overflow Flag
///
/// This instruction clears the overflow flag.
///
/// # Returns
///
/// The number of additional cycles incurred by this instruction (0).
pub fn clv<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    // Clear the overflow flag
    cpu.registers.status.overflow = false;
    // Return 0 additional cycles
    0
}

/// CMP - Compare Accumulator
///
/// This instruction compares the value in the accumulator to the value at the
/// given address and sets the carry, zero, and negative flags based on the
/// result.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - The address to read from.
///
/// # Returns
///
/// The number of additional cycles incurred by this instruction (0).
pub fn cmp<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Read the value from the given address
    let m = cpu.bus.read(addr);
    // Calculate the result of the comparison
    let result = cpu.registers.a.wrapping_sub(m);
    // Set the carry flag if a > m
    cpu.registers.status.carry = cpu.registers.a >= m;
    // Set the zero flag if a == m
    cpu.registers.status.zero = cpu.registers.a == m;
    // Set the negative flag if the result is negative
    cpu.registers.status.negative = (result & 0x80) != 0;
    // Return 0 additional cycles
    0
}

/// CPX - Compare X Register
///
/// This instruction compares the value in the X register to the value at the
/// given address and sets the carry, zero, and negative flags based on the
/// result.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - The address to read from.
///
/// # Returns
///
/// The number of additional cycles incurred by this instruction (0).
pub fn cpx<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Read the value from the given address
    let m = cpu.bus.read(addr);
    // Calculate the result of the comparison
    let result = cpu.registers.x.wrapping_sub(m);
    // Set the carry flag if x >= m
    cpu.registers.status.carry = cpu.registers.x >= m;
    // Set the zero flag if x == m
    cpu.registers.status.zero = cpu.registers.x == m;
    // Set the negative flag if the result has the high bit set
    cpu.registers.status.negative = (result & 0x80) != 0;
    // Return 0 additional cycles
    0
}

/// CPY - Compare Y Register
///
/// This instruction compares the value in the Y register to the value at the
/// given address and sets the carry, zero, and negative flags based on the
/// result.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - The address to read from.
///
/// # Returns
///
/// The number of additional cycles incurred by this instruction (0).
pub fn cpy<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Read the value from the given address
    let m = cpu.bus.read(addr);
    // Calculate the result of the comparison
    let result = cpu.registers.y.wrapping_sub(m);
    // Set the carry flag if y >= m
    cpu.registers.status.carry = cpu.registers.y >= m;
    // Set the zero flag if y == m
    cpu.registers.status.zero = cpu.registers.y == m;
    // Set the negative flag if the result has the high bit set
    cpu.registers.status.negative = (result & 0x80) != 0;
    // Return 0 additional cycles
    0
}

/// DEC - Decrement Memory
///
/// This instruction decrements the value at the given address.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - The address to decrement.
///
/// # Returns
///
/// The number of additional cycles incurred by this instruction (0).
pub fn dec<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Read the value from the given address
    let m = cpu.bus.read(addr);
    // Decrement the value
    let result = m.wrapping_sub(1);
    // Write the result back to the given address
    cpu.bus.write(addr, result);
    // Update the zero and negative flags
    cpu.update_zero_and_negative_flags(result);
    // Return 0 additional cycles
    0
}

/// DEX - Decrement X Register
///
/// This instruction decrements the value in the X register by one. The zero
/// and negative flags are updated based on the result.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - This argument is unused for this instruction.
///
/// # Returns
///
/// The number of additional cycles incurred by the instruction (always 0).
pub fn dex<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    // Subtract 1 from the X register
    cpu.registers.x = cpu.registers.x.wrapping_sub(1);
    // Update the zero and negative flags based on the X register's value
    cpu.update_zero_and_negative_flags(cpu.registers.x);
    // Return 0 additional cycles
    0
}

/// DEY - Decrement Y Register
///
/// This instruction decrements the value in the Y register by one. The zero
/// and negative flags are updated based on the result.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - This argument is unused for this instruction.
///
/// # Returns
///
/// The number of additional cycles incurred by the instruction (always 0).
pub fn dey<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    // Subtract 1 from the Y register
    cpu.registers.y = cpu.registers.y.wrapping_sub(1);
    // Update the zero and negative flags based on the Y register's value
    cpu.update_zero_and_negative_flags(cpu.registers.y);
    // Return 0 additional cycles
    0
}

/// EOR - Exclusive OR
///
/// This instruction performs an exclusive OR between the accumulator and the 
/// value at the given address, storing the result in the accumulator. The zero 
/// and negative flags are updated based on the result.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - The address to read from.
///
/// # Returns
///
/// The number of additional cycles incurred by this instruction (always 0).
pub fn eor<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Read the value from the specified address
    let m = cpu.bus.read(addr);
    // Perform XOR operation with the accumulator
    let result = cpu.registers.a ^ m;
    // Store the result back into the accumulator
    cpu.registers.a = result;
    // Update the zero and negative flags based on the result
    cpu.update_zero_and_negative_flags(result);
    // Return 0 additional cycles
    0
}

/// INC - Increment Memory
///
/// This instruction increments the value at the given address by one.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - The address to increment.
///
/// # Returns
///
/// The number of additional cycles incurred by this instruction (always 0).
pub fn inc<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Read the value from the given address
    let m = cpu.bus.read(addr);
    // Increment the value
    let result = m.wrapping_add(1);
    // Write the result back to the given address
    cpu.bus.write(addr, result);
    // Update the zero and negative flags based on the result
    cpu.update_zero_and_negative_flags(result);
    // Return 0 additional cycles
    0
}

/// INX - Increment X Register
///
/// This instruction increments the value in the X register by one. The zero
/// and negative flags are updated based on the result.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - This argument is unused for this instruction.
///
/// # Returns
///
/// The number of additional cycles incurred by the instruction (always 0).
pub fn inx<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    // Increment the X register
    cpu.registers.x = cpu.registers.x.wrapping_add(1);
    // Update the zero and negative flags based on the X register's value
    cpu.update_zero_and_negative_flags(cpu.registers.x);
    // Return 0 additional cycles
    0
}

/// INY - Increment Y Register
///
/// This instruction increments the value in the Y register by one. The zero
/// and negative flags are updated based on the result.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - This argument is unused for this instruction.
///
/// # Returns
///
/// The number of additional cycles incurred by the instruction (always 0).
pub fn iny<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    // Increment the Y register
    cpu.registers.y = cpu.registers.y.wrapping_add(1);
    // Update the zero and negative flags based on the Y register's value
    cpu.update_zero_and_negative_flags(cpu.registers.y);
    // Return 0 additional cycles
    0
}

/// JMP - Jump
///
/// The JMP instruction sets the program counter to the given address.
///
/// # Returns
///
/// The number of additional cycles that the instruction adds to the instruction's
/// base cycle count (always 0).
pub fn jmp<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Set the program counter to the given address
    cpu.registers.pc = addr;
    // Return 0 additional cycles
    0
}

/// JSR - Jump to Subroutine
///
/// The JSR instruction pushes the current program counter onto the stack and
/// sets the program counter to the given address.
///
/// # Returns
///
/// The number of additional cycles that the instruction adds to the instruction's
/// base cycle count (always 0).
pub fn jsr<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Push the current program counter onto the stack
    let pc = cpu.registers.pc.wrapping_sub(1);
    let hi = (pc >> 8) as u8;
    let lo = pc as u8;
    cpu.stack_push(hi);
    cpu.stack_push(lo);
    // Set the program counter to the given address
    cpu.registers.pc = addr;
    // Return 0 additional cycles
    0
}

/// LDA - Load Accumulator
///
/// This instruction loads a byte from the specified address into the
/// accumulator (A) register.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - The address to read from.
///
/// # Returns
///
/// The number of additional cycles incurred by this instruction (always 0).
pub fn lda<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Read the value from the specified address
    let value = cpu.bus.read(addr);
    // Load the value into the accumulator
    cpu.registers.a = value;
    // Update the zero and negative flags based on the accumulator's value
    cpu.update_zero_and_negative_flags(cpu.registers.a);
    // Return 0 additional cycles
    0
}

/// LDX - Load X Register
///
/// This instruction loads a byte from the specified address into the
/// X register.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - The address to read from.
///
/// # Returns
///
/// The number of additional cycles incurred by this instruction (always 0).
pub fn ldx<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Read the value from the specified address
    let value = cpu.bus.read(addr);
    // Load the value into the X register
    cpu.registers.x = value;
    // Update the zero and negative flags based on the X register's value
    cpu.update_zero_and_negative_flags(value);
    // Return 0 additional cycles
    0
}

/// LDY - Load Y Register
///
/// This instruction loads a byte from the specified address into the
/// Y register.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - The address to read from.
///
/// # Returns
///
/// The number of additional cycles incurred by this instruction (always 0).
pub fn ldy<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Read the value from the specified address
    let value = cpu.bus.read(addr);
    // Load the value into the Y register
    cpu.registers.y = value;
    // Update the zero and negative flags based on the Y register's value
    cpu.update_zero_and_negative_flags(value);
    // Return 0 additional cycles
    0
}

/// LSR - Logical Shift Right
///
/// This instruction shifts the bits in the accumulator one position to the
/// right. The bit that was shifted out is stored in the carry flag, and the
/// result is stored back into the accumulator.
///
/// The zero and negative flags are updated based on the result.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - The address to read from (unused for this instruction).
///
/// # Returns
///
/// The number of additional cycles incurred by this instruction (always 0).
pub fn lsr_accumulator<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    // Read the value from the accumulator
    let m = cpu.registers.a;
    // Shift the value to the right by one bit
    let result = m >> 1;
    // Set the carry flag if the least significant bit of the original value was set
    cpu.registers.status.carry = (m & 0x01) != 0;
    // Write the result back to the accumulator
    cpu.registers.a = result;
    // Update the zero and negative flags based on the result
    cpu.update_zero_and_negative_flags(result);
    // The Negative flag will always be cleared because the result's bit 7 is 0 after shift
    // But update_zero_and_negative handles that
    // Return 0 additional cycles
    0
}

/// LSR - Logical Shift Right (Memory)
///
/// This instruction shifts the bits in the memory location at the given
/// address one position to the right. The bit that was shifted out is stored
/// in the carry flag, and the result is stored back into the memory location.
///
/// The zero and negative flags are updated based on the result.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - The address of the memory location to shift.
///
/// # Returns
///
/// The number of additional cycles incurred by this instruction (always 0).
pub fn lsr_memory<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Read the value from the specified address
    let m = cpu.bus.read(addr);
    // Shift the value to the right by one bit
    let result = m >> 1;
    // Set the carry flag if the least significant bit of the original value was set
    cpu.registers.status.carry = (m & 0x01) != 0;
    // Write the result back to the specified address
    cpu.bus.write(addr, result);
    // Update the zero and negative flags based on the result
    cpu.update_zero_and_negative_flags(result);
    // Return 0 additional cycles
    0
}

/// NOP - No Operation
///
/// This instruction performs no operation and is used to introduce a small delay.
///
/// # Returns
///
/// The number of additional cycles incurred by this instruction (always 0).
pub fn nop<B: Bus>(_cpu: &mut CPU<B>, _addr: u16) -> u8 {
    // No operation is performed
    0
}

pub fn ora<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    let value = cpu.bus.read(addr);
    cpu.unimplemented_instruction(value);
    0
}

pub fn pha<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    let value = cpu.bus.read(addr);
    cpu.unimplemented_instruction(value);
    0
}

pub fn php<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    let value = cpu.bus.read(addr);
    cpu.unimplemented_instruction(value);
    0
}

pub fn pla<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    let value = cpu.bus.read(addr);
    cpu.unimplemented_instruction(value);
    0
}

pub fn plp<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    let value = cpu.bus.read(addr);
    cpu.unimplemented_instruction(value);
    0
}

pub fn rol<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    let value = cpu.bus.read(addr);
    cpu.unimplemented_instruction(value);
    0
}

/// ROR - Rotate Right
///
/// Rotate the contents of the accumulator one position to the right. 
/// The carry flag is shifted into bit 7 of the result, and bit 0 of 
/// the result is shifted into the carry flag.
///
/// # Returns
///
/// The number of additional cycles that the instruction adds to the
/// instruction's base cycle count.
pub fn ror_accumulator<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    // Read the accumulator
    let m = cpu.registers.a;
    // Save the carry flag
    let old_carry = if cpu.registers.status.carry { 1 } else { 0 };
    // Set the carry flag to the value of the low bit of the accumulator
    cpu.registers.status.carry = (m & 0x01) != 0;
    // Rotate the accumulator one position to the right, shifting the carry
    // flag into bit 7 of the result, and shifting bit 0 of the result into
    // the carry flag.
    let result = (m >> 1) | (old_carry << 7);
    // Store the result back into the accumulator
    cpu.registers.a = result;
    // Update the zero and negative flags
    cpu.update_zero_and_negative_flags(result);
    // Return the additional cycle count
    0
}

/// ROR - Rotate Right (Memory)
///
/// This instruction rotates the bits in the memory location at the given
/// address one position to the right. The carry flag is shifted into bit 7 
/// of the result, and bit 0 of the original value is shifted into the carry flag.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - The address of the memory location to rotate.
///
/// # Returns
///
/// The number of additional cycles incurred by this instruction (always 0).
pub fn ror_memory<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Read the value from the specified address
    let m = cpu.bus.read(addr);
    // Save the current carry flag as a bit value
    let old_carry = if cpu.registers.status.carry { 1 } else { 0 };
    // Set the carry flag to the value of the least significant bit of the original value
    cpu.registers.status.carry = (m & 0x01) != 0;
    // Rotate the value one position to the right, inserting the old carry as the new high bit
    let result = (m >> 1) | (old_carry << 7);
    // Write the result back to the specified address
    cpu.bus.write(addr, result);
    // Update the zero and negative flags based on the result
    cpu.update_zero_and_negative_flags(result);
    // Return 0 additional cycles
    0
}

/// RTI - Return from Interrupt
///
/// This instruction is used to return from an interrupt handler. It restores
/// the program counter and the status register from the stack.
///
/// # Returns
///
/// The number of additional cycles that the instruction adds to the
/// instruction's base cycle count (always 0).
pub fn rti<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    // Pop the status register from the stack
    let status = cpu.stack_pop();
    // Restore the status flags from the popped value
    cpu.registers.status.from_byte(status);

    // Pop the low and high bytes of the program counter from the stack
    let lo = cpu.stack_pop();
    let hi = cpu.stack_pop();
    // Combine the low and high bytes to form the program counter
    let pc = (hi as u16) << 8 | (lo as u16);
    // Set the program counter to the restored value
    cpu.registers.pc = pc;
    
    // Return 0 additional cycles
    0
}

/// RTS - Return from Subroutine
///
/// This instruction is used to return from a subroutine. It increments the
/// program counter by one and returns the program counter to the value on the
/// stack.
///
/// # Returns
///
/// The number of additional cycles that the instruction adds to the
/// instruction's base cycle count (always 0).
pub fn rts<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    // Pop the low and high bytes of the program counter from the stack
    let lo = cpu.stack_pop();
    let hi = cpu.stack_pop();
    // Combine the low and high bytes to form the program counter
    let pc = (hi as u16) << 8 | lo as u16;
    // Increment the program counter by one
    cpu.registers.pc = pc.wrapping_add(1);
    
    // Return 0 additional cycles
    0
}

/// SBC - Subtract with Carry
///
/// This instruction subtracts the value of the memory at the given address
/// from the accumulator, taking into account the carry flag. If the decimal
/// mode flag is set, the instruction subtracts the values as BCD values.
/// Otherwise it subtracts the values as binary values.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - The address of the memory to subtract from the accumulator.
///
/// # Returns
///
/// The number of additional cycles that the instruction adds to the
/// instruction's base cycle count (always 0).
pub fn sbc<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    let m = cpu.bus.read(addr);
    let value = m;
    let carry = if cpu.registers.status.carry { 1 } else { 0 };
    let a = cpu.registers.a;
    if cpu.registers.status.decimal_mode {
        let mut temp = a as i16 - value as i16 - (1 - carry) as i16;
        // Set the carry flag if the result is negative
        cpu.registers.status.carry = temp >= 0;
        // Set the zero flag if the result is zero
        cpu.registers.status.zero = (temp & 0xFF) == 0;
        // Set the negative flag if the result has the high bit set
        cpu.registers.status.negative = (temp & 0x80) != 0;
        // Set the overflow flag if the result is negative and the carry flag was set
        cpu.registers.status.overflow = ((a ^ temp as u8) & (a ^ value) & 0x80) != 0;
        // If the lower nibble of A is less than the lower nibble of M plus the carry
        // flag, subtract 6 from the result
        if (a & 0x0F) < ((value & 0x0F) + (1 - carry as u8)) {
            temp -= 6;
        }
        // If the result is negative, subtract 0x60 from the result
        if temp < 0 {
            temp -= 0x60;
        }
        // Store the result in A
        cpu.registers.a = (temp & 0xFF) as u8;
    } else {
        let temp = a as u16 - value as u16 - (1 - carry) as u16;
        // Store the result in A
        cpu.registers.a = temp as u8;
        // Set the carry flag if the result is positive (no borrow)
        cpu.registers.status.carry = temp < 0x100;
        // Set the zero flag if the result is zero
        cpu.registers.status.zero = cpu.registers.a == 0;
        // Set the negative flag if the result has the high bit set
        cpu.registers.status.negative = (cpu.registers.a & 0x80) != 0;
        // Set the overflow flag if the result has the high bit set and the carry
        // flag was set
        cpu.registers.status.overflow = ((a ^ cpu.registers.a) & (a ^ value) & 0x80) != 0;
    }
    0
}

/// SEC - Set Carry Flag
///
/// This instruction sets the carry flag to true.
///
/// # Returns
///
/// The number of additional cycles that the instruction adds to the
/// instruction's base cycle count (always 0).
pub fn sec<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    // Set the carry flag to true
    cpu.registers.status.carry = true;
    // Return 0 additional cycles
    0
}

/// SED - Set Decimal Mode
///
/// This instruction sets the decimal mode flag to true. In decimal mode, the
/// 6502 CPU performs arithmetic operations in BCD (Binary Coded Decimal)
/// format.
///
/// # Returns
///
/// The number of additional cycles that the instruction adds to the
/// instruction's base cycle count (always 0).
pub fn sed<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    // Set the decimal mode flag to true
    cpu.registers.status.decimal_mode = true;
    // Return 0 additional cycles
    0
}

/// SEI - Set Interrupt Disable
///
/// This instruction sets the Interrupt Disable flag to true, which prevents
/// the CPU from responding to interrupts until the flag is cleared.
///
/// # Returns
///
/// The number of additional cycles that the instruction adds to the
/// instruction's base cycle count (always 0).
pub fn sei<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    // Set the Interrupt Disable flag to true
    cpu.registers.status.interrupt_disable = true;
    // Return 0 additional cycles
    0
}

/// STA - Store Accumulator
///
/// This instruction stores the value of the accumulator (A) register at the
/// given address.
///
/// # Returns
///
/// The number of additional cycles that the instruction adds to the
/// instruction's base cycle count (always 0).
pub fn sta<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Store the value of the accumulator at the given address
    cpu.bus.write(addr, cpu.registers.a);
    // Return 0 additional cycles
    0
}

/// STA - Store Accumulator
///
/// This instruction stores the value of the accumulator (A) register at the
/// given address.
///
/// # Returns
///
/// The number of additional cycles that the instruction adds to the
/// instruction's base cycle count (always 0).
pub fn stx<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Store the value of the X register at the given address
    cpu.bus.write(addr, cpu.registers.x);
    // Return 0 additional cycles
    0
}

/// STY - Store Y Register
///
/// This instruction stores the value of the Y register at the given address.
///
/// # Returns
///
/// The number of additional cycles that the instruction adds to the
/// instruction's base cycle count (always 0).
pub fn sty<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Store the value of the Y register at the given address
    cpu.bus.write(addr, cpu.registers.y);
    // Return 0 additional cycles
    0
}

/// TAX - Transfer Accumulator to X
///
/// This instruction copies the value of the accumulator (A) register to the X
/// register.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - Unused argument.
///
/// # Returns
///
/// The number of additional cycles that the instruction adds to the
/// instruction's base cycle count (always 0).
pub fn tax<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    // Copy the value of the accumulator to the X register
    cpu.registers.x = cpu.registers.a;
    // Update the zero and negative flags based on the X register's value
    cpu.update_zero_and_negative_flags(cpu.registers.x);
    // Return 0 additional cycles
    0
}

/// TAY - Transfer Accumulator to Y
///
/// This instruction copies the value of the accumulator (A) register to the Y
/// register.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - Unused argument.
///
/// # Returns
///
/// The number of additional cycles that the instruction adds to the
/// instruction's base cycle count (always 0).
pub fn tay<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    // Copy the value of the accumulator to the Y register
    cpu.registers.y = cpu.registers.a;
    // Update the zero and negative flags based on the Y register's value
    cpu.update_zero_and_negative_flags(cpu.registers.y);
    // Return 0 additional cycles
    0
}

/// TSX - Transfer Stack Pointer to X
///
/// This instruction copies the value of the stack pointer register to the X
/// register.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - Unused argument.
///
/// # Returns
///
/// The number of additional cycles that the instruction adds to the
/// instruction's base cycle count (always 0).
pub fn tsx<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    // Copy the value of the stack pointer to the X register
    cpu.registers.x = cpu.registers.sp;
    // Update the zero and negative flags based on the X register's value
    cpu.update_zero_and_negative_flags(cpu.registers.x);
    // Return 0 additional cycles
    0
}

/// TXA - Transfer X to Accumulator
///
/// This instruction copies the value of the X register to the accumulator.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - Unused argument.
///
/// # Returns
///
/// The number of additional cycles that the instruction adds to the
/// instruction's base cycle count (always 0).
pub fn txa<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    // Copy the value of the X register to the accumulator
    cpu.registers.a = cpu.registers.x;
    // Update the zero and negative flags based on the accumulator's value
    cpu.update_zero_and_negative_flags(cpu.registers.a);
    // Return 0 additional cycles
    0
}

/// TXS - Transfer X to Stack Pointer
///
/// This instruction copies the value of the X register to the stack pointer.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - Unused argument.
///
/// # Returns
///
/// The number of additional cycles that the instruction adds to the
/// instruction's base cycle count (always 0).
pub fn txs<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    // Copy the value of the X register to the stack pointer
    cpu.registers.sp = cpu.registers.x;
    // Return 0 additional cycles
    0
}

/// TYA - Transfer Y to Accumulator
///
/// This instruction copies the value of the Y register to the accumulator.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the CPU instance.
/// * `addr` - Unused argument.
///
/// # Returns
///
/// The number of additional cycles that the instruction adds to the
/// instruction's base cycle count (always 0).
pub fn tya<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    // Copy the value of the Y register to the accumulator
    cpu.registers.a = cpu.registers.y;
    // Update the zero and negative flags based on the accumulator's value
    cpu.update_zero_and_negative_flags(cpu.registers.a);
    // Return 0 additional cycles
    0
}
