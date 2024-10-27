//! The `cpu` module contains the implementation of the 6502 CPU emulator.

use crate::addressing_modes::*;
use crate::bus::Bus;
use crate::instructions::Instruction;
use crate::registers::{Registers, StatusFlags};
use std::collections::HashMap;

/// The `DecodedInstruction` struct holds the decoded instruction and its associated metadata.
/// Represents a decoded instruction, consisting of an instruction handler, an addressing mode function, and base cycle count.
///
/// A decoded instruction is created by the CPU during the decoding phase of the instruction execution cycle.
/// It is used to execute the instruction later in the execution phase.
///
/// The `instruction` field holds a function pointer to the instruction handler function.
/// The `addressing_mode` field holds a function pointer to the addressing mode function.
/// The `cycles` field holds the base number of cycles required by the instruction.
/// This may be increased by additional cycles added by the addressing mode.
pub struct DecodedInstruction<B: Bus> {
    /// Instruction handler function
    pub instruction: Instruction<B>,
    /// Addressing mode function
    pub addressing_mode: AddressingMode<B>,
    /// Base number of cycles for the instruction
    pub cycles: u8,
}

/// The `CPU` struct represents the 6502 CPU emulator.
///
/// It contains the current state of the CPU, including the registers, the bus, and the instruction table.
/// The instruction table is used to decode instructions and execute them.
pub struct CPU<B: Bus> {
    /// The current state of the CPU registers.
    pub registers: Registers,

    /// The bus used by the CPU to access memory and I/O.
    pub bus: B,

    /// The total number of cycles elapsed since the CPU was reset.
    /// This is used to track the CPU's progress and to handle certain instructions that depend on the cycle count.
    cycles: u64,

    /// The instruction table is a mapping of opcodes to their associated instruction handlers and addressing modes.
    /// The instruction table is used to decode instructions and execute them.
    instruction_table: HashMap<u8, DecodedInstruction<B>>,
}

impl<B: Bus> CPU<B> {
    /// Creates a new instance of the `CPU` with the given bus.
    ///
    /// # Arguments
    ///
    /// * `bus` - The bus to be used by the CPU for memory and I/O operations.
    ///
    /// # Returns
    ///
    /// A new `CPU` instance with initialized registers and instruction table.
    pub fn new(bus: B) -> Self {
        // Initialize the CPU with default register values and the provided bus
        let mut cpu = Self {
            registers: Registers::new(), // Create new registers with default values
            bus,                         // Use the provided bus for memory operations
            cycles: 0,                   // Initialize cycle count to zero
            instruction_table: HashMap::new(), // Create an empty instruction table
        };
        cpu.init_instruction_table(); // Initialize the instruction table with opcodes
        cpu // Return the initialized CPU instance
    }

    /// Resets the CPU to its initial state.
    ///
    /// This method is used to initialize the CPU at the start of a program.
    /// It sets the program counter to the reset vector address, initializes the stack pointer to 0xFD, and clears the
    /// status flags.
    pub fn reset(&mut self) {
        // Read the reset vector from the bus
        let lo = self.bus.read(0xFFFC) as u16;
        let hi = self.bus.read(0xFFFD) as u16;

        // Set the program counter to the reset vector address
        self.registers.pc = (hi << 8) | lo;

        // Initialize the stack pointer to 0xFD
        self.registers.sp = 0xFD;

        // Clear the status flags
        self.registers.status = StatusFlags::new();

        // Reset the cycle count to zero
        self.cycles = 0;
    }

    /// Executes one instruction cycle.
    ///
    /// This method fetches the current opcode from memory, decodes the instruction, and executes it.
    /// If the instruction is not implemented, it will call the `unimplemented_instruction` method.
    pub fn step(&mut self) {
        let opcode = self.fetch_byte();
        // Get the instruction from the instruction table
        if let Some(decoded_instruction) = self.instruction_table.get(&opcode) {
            // Get the instruction and addressing mode from the instruction table
            let instruction = decoded_instruction.instruction;
            let addressing_mode = decoded_instruction.addressing_mode;
            let base_cycles = decoded_instruction.cycles;

            // Get the address and additional cycles from the addressing mode
            let (addr, addr_additional_cycles) = addressing_mode(self);

            // Execute the instruction
            let instr_additional_cycles = instruction(self, addr);

            // Calculate the total cycles for this instruction
            let total_cycles = base_cycles + addr_additional_cycles + instr_additional_cycles;

            // Increment the CPU cycle count by the total cycles
            self.cycles += total_cycles as u64;
        } else {
            // If the instruction is not implemented, call the unimplemented_instruction method
            self.unimplemented_instruction(opcode);
        }
    }

    /// Fetches the next byte from the memory bus and increments the program counter.
    ///
    /// This method is used to fetch the next opcode or operand from memory.
    /// It increments the program counter after fetching the byte.
    pub fn fetch_byte(&mut self) -> u8 {
        let byte = self.bus.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        byte
    }

    /// Fetches the next word from the memory bus and increments the program counter.
    ///
    /// This method is used to fetch the next opcode or operand from memory.
    /// It increments the program counter after fetching the word.
    pub fn fetch_word(&mut self) -> u16 {
        // Fetch the low byte
        let lo = self.fetch_byte() as u16;
        // Fetch the high byte
        let hi = self.fetch_byte() as u16;
        // Combine the bytes into a 16-bit word
        (hi << 8) | lo
    }

    /// Pushes a byte onto the stack.
    ///
    /// This method writes a byte to the current stack location in memory
    /// and then decrements the stack pointer.
    ///
    /// # Arguments
    ///
    /// * `data` - The byte to be pushed onto the stack.
    pub fn stack_push(&mut self, data: u8) {
        // Write the byte to the stack memory address
        self.bus.write(0x0100 + self.registers.sp as u16, data);
        // Decrement the stack pointer
        self.registers.sp = self.registers.sp.wrapping_sub(1);
    }

    /// Pops a byte from the stack.
    ///
    /// This method reads a byte from the current stack location in memory
    /// and then increments the stack pointer.
    ///
    /// # Returns
    ///
    /// The byte popped from the stack.
    pub fn stack_pop(&mut self) -> u8 {
        // Increment the stack pointer
        self.registers.sp = self.registers.sp.wrapping_add(1);
        // Read the byte from the stack memory address
        self.bus.read(0x0100 + self.registers.sp as u16)
    }

    /// Updates the zero and negative flags based on the result.
    ///
    /// This method sets the zero flag if the result is zero and the negative flag
    /// if the most significant bit (bit 7) of the result is set.
    ///
    /// # Arguments
    ///
    /// * `result` - The result to check for zero and negative flags.
    pub fn update_zero_and_negative_flags(&mut self, result: u8) {
        // Set the zero flag if the result is zero
        self.registers.status.zero = result == 0;
        // Set the negative flag if bit 7 of the result is set
        self.registers.status.negative = (result & 0x80) != 0;
    }

    /// Branches to the specified address and returns the cycle penalty.
    ///
    /// This method updates the program counter to the given address and checks
    /// if a page boundary was crossed during the branch. If a page boundary
    /// is crossed, an additional cycle penalty is incurred.
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to branch to.
    ///
    /// # Returns
    ///
    /// The cycle penalty incurred by the branch operation (1 or 2).
    pub fn branch(&mut self, addr: u16) -> u8 {
        // Store the current program counter
        let old_pc = self.registers.pc;
        // Update the program counter to the new address
        self.registers.pc = addr;
        // Determine if a page boundary was crossed
        let page_cross = (old_pc & 0xFF00) != (addr & 0xFF00);
        // Return the cycle penalty based on page crossing
        if page_cross {
            2
        } else {
            1
        }
    }

    /// Handles an interrupt (IRQ or NMI).
    ///
    /// This method will not trigger an interrupt if the Interrupt Disable flag is set
    /// and the interrupt is not an NMI.
    ///
    /// # Arguments
    ///
    /// * `nmi` - Whether the interrupt is an NMI (true) or an IRQ (false).
    fn interrupt(&mut self, nmi: bool) {
        if self.registers.status.interrupt_disable && !nmi {
            return;
        }
        // Push the current program counter onto the stack
        self.stack_push((self.registers.pc >> 8) as u8);
        self.stack_push((self.registers.pc & 0xFF) as u8);
        // Push the current status register onto the stack
        // Clear the B flag and set the U flag
        let mut status = self.registers.status.to_byte();
        status &= !0x10;
        status |= 0x20;
        self.stack_push(status);
        // Set the Interrupt Disable flag
        self.registers.status.interrupt_disable = true;
        // Read the interrupt vector address from memory
        let vector_address = if nmi { 0xFFFA } else { 0xFFFE };
        let lo = self.bus.read(vector_address) as u16;
        let hi = self.bus.read(vector_address + 1) as u16;
        // Set the program counter to the vector address
        self.registers.pc = (hi << 8) | lo;
    }

    /// Handles an interrupt request (IRQ).
    ///
    /// This method will not trigger an interrupt if the Interrupt Disable flag is set.
    pub fn irq(&mut self) {
        self.interrupt(false);
    }

    /// Handles a non-maskable interrupt (NMI).
    ///
    /// This method will trigger an interrupt regardless of the Interrupt Disable flag.
    pub fn nmi(&mut self) {
        self.interrupt(true);
    }

    /// Panics when an unimplemented opcode is encountered.
    ///
    /// This method is called when the emulator encounters an opcode that is not implemented.
    /// It will panic with an error message indicating the opcode and the current program counter.
    ///
    /// # Arguments
    ///
    /// * `opcode` - The opcode that was not implemented.
    pub fn unimplemented_instruction(&mut self, opcode: u8) {
        panic!("Unimplemented opcode {:02X} at PC: {:04X}", opcode, self.registers.pc);
    }

    /// Returns the current cycle count.
    ///
    /// # Returns
    ///
    /// The current cycle count of the CPU.
    pub fn cycles(&self) -> u64 {
        self.cycles
    }

    /// Initializes the instruction dispatch table.
    fn init_instruction_table(&mut self) {
        use crate::addressing_modes::*;
        use crate::instructions::*;

        // Map opcodes to instruction handlers and addressing modes with cycle counts

        // ADC Instructions
        self.map_opcode(0x69, adc, immediate, 2); // ADC Immediate
        self.map_opcode(0x65, adc, zero_page, 3); // ADC Zero Page
        self.map_opcode(0x75, adc, zero_page_x, 4); // ADC Zero Page,X
        self.map_opcode(0x6D, adc, absolute, 4); // ADC Absolute
        self.map_opcode(0x7D, adc, absolute_x, 4); // ADC Absolute,X (+1 if page crossed)
        self.map_opcode(0x79, adc, absolute_y, 4); // ADC Absolute,Y (+1 if page crossed)
        self.map_opcode(0x61, adc, indirect_x, 6); // ADC Indirect,X
        self.map_opcode(0x71, adc, indirect_y, 5); // ADC Indirect,Y (+1 if page crossed)

        // AND Instructions
        self.map_opcode(0x29, and, immediate, 2); // AND Immediate
        self.map_opcode(0x25, and, zero_page, 3); // AND Zero Page
        self.map_opcode(0x35, and, zero_page_x, 4); // AND Zero Page,X
        self.map_opcode(0x2D, and, absolute, 4); // AND Absolute
        self.map_opcode(0x3D, and, absolute_x, 4); // AND Absolute,X (+1 if page crossed)
        self.map_opcode(0x39, and, absolute_y, 4); // AND Absolute,Y (+1 if page crossed)
        self.map_opcode(0x21, and, indirect_x, 6); // AND Indirect,X
        self.map_opcode(0x31, and, indirect_y, 5); // AND Indirect,Y (+1 if page crossed)

        // ASL Instructions
        self.map_opcode(0x0A, asl, accumulator, 2); // ASL Accumulator
        self.map_opcode(0x06, asl, zero_page, 5); // ASL Zero Page
        self.map_opcode(0x16, asl, zero_page_x, 6); // ASL Zero Page,X
        self.map_opcode(0x0E, asl, absolute, 6); // ASL Absolute
        self.map_opcode(0x1E, asl, absolute_x, 7); // ASL Absolute,X (+1 if page crossed)

        // Branch Instructions
        self.map_opcode(0x90, bcc, relative, 2); // BCC Relative
        self.map_opcode(0xB0, bcs, relative, 2); // BCS Relative
        self.map_opcode(0xF0, beq, relative, 2); // BEQ Relative

        // Bit Instructions
        self.map_opcode(0x24, bit, zero_page, 3); // BIT Zero Page
        self.map_opcode(0x2C, bit, absolute, 4); // BIT Absolute

        // Branch Instructions (continued)
        self.map_opcode(0x30, bmi, relative, 2); // BMI Relative
        self.map_opcode(0xD0, bne, relative, 2); // BNE Relative
        self.map_opcode(0x10, bpl, relative, 2); // BPL Relative

        // Break Instruction
        self.map_opcode(0x00, brk, implied, 7); // BRK Implied

        // Branch Instructions (continued)
        self.map_opcode(0x50, bvc, relative, 2); // BVC Relative
        self.map_opcode(0x70, bvs, relative, 2); // BVS Relative

        // Clear Instructions
        self.map_opcode(0x18, clc, implied, 2); // CLC Implied
        self.map_opcode(0xD8, cld, implied, 2); // CLD Implied
        self.map_opcode(0x58, cli, implied, 2); // CLI Implied
        self.map_opcode(0xB8, clv, implied, 2); // CLV Implied

        // Comparison Instructions
        self.map_opcode(0xC9, cmp, immediate, 2); // CMP Immediate
        self.map_opcode(0xC5, cmp, zero_page, 3); // CMP Zero Page
        self.map_opcode(0xD5, cmp, zero_page_x, 4); // CMP Zero Page,X
        self.map_opcode(0xCD, cmp, absolute, 4); // CMP Absolute
        self.map_opcode(0xDD, cmp, absolute_x, 4); // CMP Absolute,X (+1 if page crossed)
        self.map_opcode(0xD9, cmp, absolute_y, 4); // CMP Absolute,Y (+1 if page crossed)
        self.map_opcode(0xC1, cmp, indirect_x, 6); // CMP Indirect,X
        self.map_opcode(0xD1, cmp, indirect_y, 5); // CMP Indirect,Y (+1 if page crossed)

        // Compare X Instructions
        self.map_opcode(0xE0, cpx, immediate, 2); // CPX Immediate
        self.map_opcode(0xE4, cpx, zero_page, 3); // CPX Zero Page
        self.map_opcode(0xEC, cpx, absolute, 4); // CPX Absolute

        // Compare Y Instructions
        self.map_opcode(0xC0, cpy, immediate, 2); // CPY Immediate
        self.map_opcode(0xC4, cpy, zero_page, 3); // CPY Zero Page
        self.map_opcode(0xCC, cpy, absolute, 4); // CPY Absolute

        // Decrement Instructions
        self.map_opcode(0xC6, dec, zero_page, 5); // DEC Zero Page
        self.map_opcode(0xD6, dec, zero_page_x, 6); // DEC Zero Page,X
        self.map_opcode(0xCE, dec, absolute, 6); // DEC Absolute
        self.map_opcode(0xDE, dec, absolute_x, 7); // DEC Absolute,X (+1 if page crossed)

        // Decrement X Instruction
        self.map_opcode(0xCA, dex, implied, 2); // DEX Implied

        // Decrement Y Instruction
        self.map_opcode(0x88, dey, implied, 2); // DEY Implied

        // Exclusive OR Instructions
        self.map_opcode(0x49, eor, immediate, 2); // EOR Immediate
        self.map_opcode(0x45, eor, zero_page, 3); // EOR Zero Page
        self.map_opcode(0x55, eor, zero_page_x, 4); // EOR Zero Page,X
        self.map_opcode(0x4D, eor, absolute, 4); // EOR Absolute
        self.map_opcode(0x5D, eor, absolute_x, 4); // EOR Absolute,X (+1 if page crossed)
        self.map_opcode(0x59, eor, absolute_y, 4); // EOR Absolute,Y (+1 if page crossed)
        self.map_opcode(0x41, eor, indirect_x, 6); // EOR Indirect,X
        self.map_opcode(0x51, eor, indirect_y, 5); // EOR Indirect,Y (+1 if page crossed)

        // Increment Instructions
        self.map_opcode(0xE6, inc, zero_page, 5); // INC Zero Page
        self.map_opcode(0xF6, inc, zero_page_x, 6); // INC Zero Page,X
        self.map_opcode(0xEE, inc, absolute, 6); // INC Absolute
        self.map_opcode(0xFE, inc, absolute_x, 7); // INC Absolute,X (+1 if page crossed)

        // Increment X Instruction
        self.map_opcode(0xE8, inx, implied, 2); // INX Implied

        // Increment Y Instruction
        self.map_opcode(0xC8, iny, implied, 2); // INY Implied

        // Jump Instructions
        self.map_opcode(0x4C, jmp, absolute, 3); // JMP Absolute
        self.map_opcode(0x6C, jmp, indirect, 5); // JMP Indirect

        // Jump Subroutine Instruction
        self.map_opcode(0x20, jsr, absolute, 6); // JSR Absolute

        // LDA Instructions
        self.map_opcode(0xA9, lda, immediate, 2); // LDA Immediate
        self.map_opcode(0xA5, lda, zero_page, 3); // LDA Zero Page
        self.map_opcode(0xB5, lda, zero_page_x, 4); // LDA Zero Page,X
        self.map_opcode(0xAD, lda, absolute, 4); // LDA Absolute
        self.map_opcode(0xBD, lda, absolute_x, 4); // LDA Absolute,X (+1 if page crossed)
        self.map_opcode(0xB9, lda, absolute_y, 4); // LDA Absolute,Y (+1 if page crossed)
        self.map_opcode(0xA1, lda, indirect_x, 6); // LDA Indirect,X
        self.map_opcode(0xB1, lda, indirect_y, 5); // LDA Indirect,Y (+1 if page crossed)

        // LDX Instructions
        self.map_opcode(0xA2, ldx, immediate, 2); // LDX Immediate
        self.map_opcode(0xA6, ldx, zero_page, 3); // LDX Zero Page
        self.map_opcode(0xB6, ldx, zero_page_y, 4); // LDX Zero Page,Y
        self.map_opcode(0xAE, ldx, absolute, 4); // LDX Absolute
        self.map_opcode(0xBE, ldx, absolute_y, 4); // LDX Absolute,Y (+1 if page crossed)

        // LDY Instructions
        self.map_opcode(0xA0, ldy, immediate, 2); // LDY Immediate
        self.map_opcode(0xA4, ldy, zero_page, 3); // LDY Zero Page
        self.map_opcode(0xB4, ldy, zero_page_x, 4); // LDY Zero Page,X
        self.map_opcode(0xAC, ldy, absolute, 4); // LDY Absolute
        self.map_opcode(0xBC, ldy, absolute_x, 4); // LDY Absolute,X (+1 if page crossed)

        // LSR (Logical Shift Right) Instructions
        self.map_opcode(0x4A, lsr_accumulator, accumulator, 2); // LSR Accumulator
        self.map_opcode(0x46, lsr_memory, zero_page, 5); // LSR Zero Page
        self.map_opcode(0x56, lsr_memory, zero_page_x, 6); // LSR Zero Page,X
        self.map_opcode(0x4E, lsr_memory, absolute, 6); // LSR Absolute
        self.map_opcode(0x5E, lsr_memory, absolute_x, 7); // LSR Absolute,X

        // No-op Instructions
        self.map_opcode(0xEA, nop, implied, 2); // NOP Implied

        // ORA Instructions
        self.map_opcode(0x09, ora, immediate, 2); // ORA Immediate
        self.map_opcode(0x05, ora, zero_page, 3); // ORA Zero Page
        self.map_opcode(0x15, ora, zero_page_x, 4); // ORA Zero Page,X
        self.map_opcode(0x0D, ora, absolute, 4); // ORA Absolute
        self.map_opcode(0x1D, ora, absolute_x, 4); // ORA Absolute,X (+1 if page crossed)
        self.map_opcode(0x19, ora, absolute_y, 4); // ORA Absolute,Y (+1 if page crossed)
        self.map_opcode(0x01, ora, indirect_x, 6); // ORA Indirect,X
        self.map_opcode(0x11, ora, indirect_y, 5); // ORA Indirect,Y (+1 if page crossed)

        // Stack Operations
        self.map_opcode(0x48, pha, implied, 3); // PHA Implied
        self.map_opcode(0x08, php, implied, 3); // PHP Implied
        self.map_opcode(0x68, pla, implied, 4); // PLA Implied
        self.map_opcode(0x28, plp, implied, 4); // PLP Implied

        // ROL (Rotate Left) Instructions
        self.map_opcode(0x2A, rol_accumulator, accumulator, 2); // ROL Accumulator
        self.map_opcode(0x26, rol_memory, zero_page, 5); // ROL Zero Page
        self.map_opcode(0x36, rol_memory, zero_page_x, 6); // ROL Zero Page,X
        self.map_opcode(0x2E, rol_memory, absolute, 6); // ROL Absolute
        self.map_opcode(0x3E, rol_memory, absolute_x, 7); // ROL Absolute,X

        // ROR (Rotate Right) Instructions
        self.map_opcode(0x6A, ror_accumulator, accumulator, 2); // ROR Accumulator
        self.map_opcode(0x66, ror_memory, zero_page, 5); // ROR Zero Page
        self.map_opcode(0x76, ror_memory, zero_page_x, 6); // ROR Zero Page,X
        self.map_opcode(0x6E, ror_memory, absolute, 6); // ROR Absolute
        self.map_opcode(0x7E, ror_memory, absolute_x, 7); // ROR Absolute,X

        // Return Instructions
        self.map_opcode(0x40, rti, implied, 6); // RTI Implied
        self.map_opcode(0x60, rts, implied, 6); // RTS Implied

        // SBC (Subtract with Carry) Instructions
        self.map_opcode(0xE9, sbc, immediate, 2); // SBC Immediate
        self.map_opcode(0xE5, sbc, zero_page, 3); // SBC Zero Page
        self.map_opcode(0xF5, sbc, zero_page_x, 4); // SBC Zero Page,X
        self.map_opcode(0xED, sbc, absolute, 4); // SBC Absolute
        self.map_opcode(0xFD, sbc, absolute_x, 4); // SBC Absolute,X (+1 if page crossed)
        self.map_opcode(0xF9, sbc, absolute_y, 4); // SBC Absolute,Y (+1 if page crossed)
        self.map_opcode(0xE1, sbc, indirect_x, 6); // SBC Indirect,X
        self.map_opcode(0xF1, sbc, indirect_y, 5); // SBC Indirect,Y (+1 if page crossed)

        // Set Status Instructions
        self.map_opcode(0x38, sec, implied, 2); // SEC Implied
        self.map_opcode(0xF8, sed, implied, 2); // SED Implied
        self.map_opcode(0x78, sei, implied, 2); // SEI Implied

        // STA (Store Accumulator) Instructions
        self.map_opcode(0x85, sta, zero_page, 3); // STA Zero Page
        self.map_opcode(0x95, sta, zero_page_x, 4); // STA Zero Page,X
        self.map_opcode(0x8D, sta, absolute, 4); // STA Absolute
        self.map_opcode(0x9D, sta, absolute_x, 5); // STA Absolute,X
        self.map_opcode(0x99, sta, absolute_y, 5); // STA Absolute,Y
        self.map_opcode(0x81, sta, indirect_x, 6); // STA Indirect,X
        self.map_opcode(0x91, sta, indirect_y, 6); // STA Indirect,Y

        // STX (Store X Register) Instructions
        self.map_opcode(0x86, stx, zero_page, 3); // STX Zero Page
        self.map_opcode(0x96, stx, zero_page_y, 4); // STX Zero Page,Y
        self.map_opcode(0x8E, stx, absolute, 4); // STX Absolute

        // STY (Store Y Register) Instructions
        self.map_opcode(0x84, sty, zero_page, 3); // STY Zero Page
        self.map_opcode(0x94, sty, zero_page_x, 4); // STY Zero Page,X
        self.map_opcode(0x8C, sty, absolute, 4); // STY Absolute

        // Transfer Operations
        self.map_opcode(0xAA, tax, implied, 2); // TAX Implied
        self.map_opcode(0xA8, tay, implied, 2); // TAY Implied
        self.map_opcode(0xBA, tsx, implied, 2); // TSX Implied
        self.map_opcode(0x8A, txa, implied, 2); // TXA Implied
        self.map_opcode(0x9A, txs, implied, 2); // TXS Implied
        self.map_opcode(0x98, tya, implied, 2); // TYA Implied
    }

    /// Helper function to map an opcode to an instruction and addressing mode.
    fn map_opcode(
        &mut self,
        opcode: u8,
        instruction: Instruction<B>,
        addressing_mode: AddressingMode<B>,
        cycles: u8,
    ) {
        self.instruction_table.insert(
            opcode,
            DecodedInstruction {
                instruction,
                addressing_mode,
                cycles,
            },
        );
    }
}
