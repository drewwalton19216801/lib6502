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

/// Branch if Minus
pub fn bmi<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    if cpu.registers.status.negative {
        cpu.branch(addr)
    } else {
        0 // No additional cycles if branch not taken
    }
}

/// Branch if Not Equal
pub fn bne<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    if !cpu.registers.status.zero {
        cpu.branch(addr)
    } else {
        0 // No additional cycles if branch not taken
    }
}

/// Branch if Positive
pub fn bpl<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    if !cpu.registers.status.negative {
        cpu.branch(addr)
    } else {
        0 // No additional cycles if branch not taken
    }
}

/// Force Interrupt
pub fn brk<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    cpu.irq(); // Fire an interrupt
    0
}

/// Branch if Overflow Clear
pub fn bvc<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    if !cpu.registers.status.overflow {
        cpu.branch(addr)
    } else {
        0 // No additional cycles if branch not taken
    }
}

/// Branch if Overflow Set
pub fn bvs<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    if cpu.registers.status.overflow {
        cpu.branch(addr)
    } else {
        0 // No additional cycles if branch not taken
    }
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

// Clear Interrupt Disable
pub fn cli<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    cpu.registers.status.interrupt_disable = false;
    0
}

// Clear Overflow Flag
pub fn clv<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    cpu.registers.status.overflow = false;
    0
}

/// Compare Accumulator
/// TODO: Implement
pub fn cmp<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}

/// Compare X Register
/// TODO: Implement
pub fn cpx<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}

/// Compare Y Register
/// TODO: Implement
pub fn cpy<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}

/// Decrement Memory
/// TODO: Implement
pub fn dec<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}

/// Decrement X Register
/// TODO: Implement
pub fn dex<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}

/// Decrement Y Register
/// TODO: Implement
pub fn dey<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}

/// Exclusive OR
/// TODO: Implement
pub fn eor<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}

/// Increment Memory
/// TODO: Implement
pub fn inc<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}

/// Increment X Register
/// TODO: Implement
pub fn inx<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}

/// Increment Y Register
/// TODO: Implement
pub fn iny<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}

/// Jump
pub fn jmp<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    cpu.registers.pc = addr;
    0
}

/// Jump to Subroutine
pub fn jsr<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    let pc = cpu.registers.pc.wrapping_sub(1);
    let hi = (pc >> 8) as u8;
    let lo = pc as u8;
    cpu.stack_push(hi);
    cpu.stack_push(lo);
    cpu.registers.pc = addr;
    0
}

/// Load Accumulator
pub fn lda<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    let value = cpu.bus.read(addr);
    cpu.registers.a = value;
    cpu.update_zero_and_negative_flags(cpu.registers.a);
    0 // No additional cycles
}

/// Load X Register
/// TODO: Implement
pub fn ldx<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}

/// Load Y Register
/// TODO: Implement
pub fn ldy<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}

/// Logical Shift Right
/// TODO: Implement
pub fn lsr<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}

/// No-Operation
pub fn nop<B: Bus>(_cpu: &mut CPU<B>, _addr: u16) -> u8 {
    0 // No additional cycles
}

/// OR with Accumulator
/// TODO: Implement
pub fn ora<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}

/// Push Accumulator
/// TODO: Implement
pub fn pha<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}

/// Push Processor Status
/// TODO: Implement
pub fn php<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}

/// Pull Accumulator
/// TODO: Implement
pub fn pla<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}

/// Pull Processor Status
/// TODO: Implement
pub fn plp<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}

/// Rotate Left
/// TODO: Implement
pub fn rol<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}

/// Rotate Right
/// TODO: Implement
pub fn ror<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}

/// Return from Interrupt
pub fn rti<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    let lo = cpu.stack_pop();
    let hi = cpu.stack_pop();
    let pc = (hi as u16) << 8 | lo as u16;
    cpu.registers.pc = pc;
    0
}

/// Return from Subroutine
pub fn rts<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    let lo = cpu.stack_pop();
    let hi = cpu.stack_pop();
    let pc = (hi as u16) << 8 | lo as u16;
    cpu.registers.pc = pc.wrapping_add(1);
    0
}

/// Subtract with Carry
/// TODO: Implement
pub fn sbc<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
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

/// Set Interrupt Disable
pub fn sei<B: Bus>(cpu: &mut CPU<B>, _addr: u16) -> u8 {
    cpu.registers.status.interrupt_disable = true;
    0
}

/// Store Accumulator
/// TODO: Implement
pub fn sta<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}

/// Store X Register
/// TODO: Implement
pub fn stx<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}

/// Store Y Register
/// TODO: Implement
pub fn sty<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}

/// Transfer Accumulator to X
/// TODO: Implement
pub fn tax<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}

/// Transfer Accumulator to Y
/// TODO: Implement
pub fn tay<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}

/// Transfer Stack Pointer to X
/// TODO: Implement
pub fn tsx<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}

/// Transfer X to Accumulator
/// TODO: Implement
pub fn txa<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}

/// Transfer X to Stack Pointer
/// TODO: Implement
pub fn txs<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}

/// Transfer Y to Accumulator
/// TODO: Implement
pub fn tya<B: Bus>(cpu: &mut CPU<B>, addr: u16) -> u8 {
    // Get the opcode at the current PC
    let value = cpu.bus.read(addr);

    cpu.unimplemented_instruction(value);
    0
}