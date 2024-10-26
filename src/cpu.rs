use crate::addressing_modes::*;
use crate::bus::Bus;
use crate::instructions::Instruction;
use crate::registers::{Registers, StatusFlags};
use std::collections::HashMap;

/// Represents a decoded instruction, consisting of an instruction handler, an addressing mode function, and base cycle count.
pub struct DecodedInstruction<B: Bus> {
    pub instruction: Instruction<B>,
    pub addressing_mode: AddressingMode<B>,
    pub cycles: u8, // Base number of cycles for the instruction
}

/// The `CPU` struct represents the 6502 CPU.
/// It is generic over a type `B` that implements the `Bus` trait.
pub struct CPU<B: Bus> {
    pub registers: Registers,
    pub bus: B,
    cycles: u64, // Total cycles elapsed
    instruction_table: HashMap<u8, DecodedInstruction<B>>,
}

impl<B: Bus> CPU<B> {
    /// Creates a new `CPU` instance with the given bus.
    pub fn new(bus: B) -> Self {
        let mut cpu = Self {
            registers: Registers::new(),
            bus,
            cycles: 0,
            instruction_table: HashMap::new(),
        };
        cpu.init_instruction_table();
        cpu
    }

    /// Resets the CPU by setting the program counter to the reset vector.
    pub fn reset(&mut self) {
        let lo = self.bus.read(0xFFFC) as u16;
        let hi = self.bus.read(0xFFFD) as u16;
        self.registers.pc = (hi << 8) | lo;
        self.registers.sp = 0xFD;
        self.registers.status = StatusFlags::new();
        self.cycles = 0;
    }

    /// Executes one CPU cycle (fetch, decode, execute).
    pub fn step(&mut self) {
        // Fetch
        let opcode = self.fetch_byte();

        // Decode
        if let Some(decoded_instruction) = self.instruction_table.get(&opcode) {
            // Copy the function pointers to avoid borrowing issues
            let instruction = decoded_instruction.instruction;
            let addressing_mode = decoded_instruction.addressing_mode;
            let base_cycles = decoded_instruction.cycles;

            // Get the operand address using the addressing mode
            let (addr, addr_additional_cycles) = addressing_mode(self);

            // Execute the instruction with the operand address
            let instr_additional_cycles = instruction(self, addr);

            // Total cycles for this instruction
            let total_cycles = base_cycles + addr_additional_cycles + instr_additional_cycles;

            // Update the CPU cycle count
            self.cycles += total_cycles as u64;
        } else {
            self.unimplemented_instruction(opcode);
        }
    }

    /// Fetches a byte from the current program counter and increments the counter.
    pub fn fetch_byte(&mut self) -> u8 {
        let byte = self.bus.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        byte
    }

    /// Fetches a word (two bytes) from the current program counter.
    pub fn fetch_word(&mut self) -> u16 {
        let lo = self.fetch_byte() as u16;
        let hi = self.fetch_byte() as u16;
        (hi << 8) | lo
    }

    /// Pushes a byte onto the stack.
    pub fn stack_push(&mut self, data: u8) {
        self.bus.write(0x0100 + self.registers.sp as u16, data);
        self.registers.sp = self.registers.sp.wrapping_sub(1);
    }

    /// Pops a byte from the stack.
    pub fn stack_pop(&mut self) -> u8 {
        self.registers.sp = self.registers.sp.wrapping_add(1);
        self.bus.read(0x0100 + self.registers.sp as u16)
    }

    /// Updates the zero and negative flags based on the given result.
    pub fn update_zero_and_negative_flags(&mut self, result: u8) {
        self.registers.status.zero = result == 0;
        self.registers.status.negative = (result & 0x80) != 0;
    }

    /// Executes a branch, returning the number of additional cycles required.
    pub fn branch(&mut self, addr: u16) -> u8 {
        let old_pc = self.registers.pc;
        self.registers.pc = addr;
        // Check if branch crosses a page boundary
        let page_cross = (old_pc & 0xFF00) != (addr & 0xFF00);
        // Branch taken: +1 cycle, +1 more if page crossed
        if page_cross {
            2
        } else {
            1
        }
    }

    /// Executes an interrupt request
    pub fn interrupt(&mut self, nmi: bool) {
        // If IRQs are disabled and the request is not NMI, ignore the IRQ
        if self.registers.status.interrupt_disable && !nmi {
            return;
        }

        // Push the high byte of the PC to the stack
        self.stack_push((self.registers.pc >> 8) as u8);

        // Push the low byte of the PC to the stack
        self.stack_push((self.registers.pc & 0xFF) as u8);

        // Modify the status register to clear the B flag and set the U flag
        let mut status = self.registers.status.to_byte();
        status &= !0x10; // Clear the Break flag (bit 4)
        status |= 0x20; // Set the Unused flag (bit 5)

        // Push the modified status register to the stack
        self.stack_push(status);

        // Set the interrupt disable flag
        self.registers.status.interrupt_disable = true;

        // Load the appropriate interrupt vector into the PC
        let vector_address = if nmi { 0xFFFA } else { 0xFFFE };
        let lo = self.bus.read(vector_address) as u16;
        let hi = self.bus.read(vector_address + 1) as u16;
        self.registers.pc = (hi << 8) | lo;
    }

    /// Executes an IRQ
    pub fn irq(&mut self) {
        self.interrupt(false);
    }

    /// Executes an NMI (Non-Maskable Interrupt)
    pub fn nmi(&mut self) {
        self.interrupt(true);
    }

    /// Unimplemented instruction handler
    pub fn unimplemented_instruction(&mut self, opcode: u8) {
        panic!("Unimplemented opcode {:02X} at PC: {:04X}", opcode, self.registers.pc);
    }

    /// Gets the current cycle count.
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
        self.map_opcode(0x4A, lsr, accumulator, 2); // LSR Accumulator
        self.map_opcode(0x46, lsr, zero_page, 5); // LSR Zero Page
        self.map_opcode(0x56, lsr, zero_page_x, 6); // LSR Zero Page,X
        self.map_opcode(0x4E, lsr, absolute, 6); // LSR Absolute
        self.map_opcode(0x5E, lsr, absolute_x, 7); // LSR Absolute,X

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
        self.map_opcode(0x2A, rol, accumulator, 2); // ROL Accumulator
        self.map_opcode(0x26, rol, zero_page, 5); // ROL Zero Page
        self.map_opcode(0x36, rol, zero_page_x, 6); // ROL Zero Page,X
        self.map_opcode(0x2E, rol, absolute, 6); // ROL Absolute
        self.map_opcode(0x3E, rol, absolute_x, 7); // ROL Absolute,X

        // ROR (Rotate Right) Instructions
        self.map_opcode(0x6A, ror, accumulator, 2); // ROR Accumulator
        self.map_opcode(0x66, ror, zero_page, 5); // ROR Zero Page
        self.map_opcode(0x76, ror, zero_page_x, 6); // ROR Zero Page,X
        self.map_opcode(0x6E, ror, absolute, 6); // ROR Absolute
        self.map_opcode(0x7E, ror, absolute_x, 7); // ROR Absolute,X

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
