use crate::bus::Bus;
use crate::instructions::Instruction;
use crate::addressing_modes::*;
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

    /// Gets the current cycle count.
    pub fn cycles(&self) -> u64 {
        self.cycles
    }

    /// Initializes the instruction dispatch table.
    fn init_instruction_table(&mut self) {
        use crate::instructions::*;
        use crate::addressing_modes::*;

        // Map opcodes to instruction handlers and addressing modes with cycle counts

        // ADC Instructions
        self.map_opcode(0x69, adc, immediate, 2);     // ADC Immediate
        self.map_opcode(0x65, adc, zero_page, 3);     // ADC Zero Page
        self.map_opcode(0x75, adc, zero_page_x, 4);   // ADC Zero Page,X
        self.map_opcode(0x6D, adc, absolute, 4);      // ADC Absolute
        self.map_opcode(0x7D, adc, absolute_x, 4);    // ADC Absolute,X (+1 if page crossed)
        self.map_opcode(0x79, adc, absolute_y, 4);    // ADC Absolute,Y (+1 if page crossed)
        self.map_opcode(0x61, adc, indirect_x, 6);    // ADC Indirect,X
        self.map_opcode(0x71, adc, indirect_y, 5);    // ADC Indirect,Y (+1 if page crossed)

        // Branch Instructions
        self.map_opcode(0x90, bcc, absolute, 2); // BCC Absolute
        self.map_opcode(0x90, bcc, relative, 2); // BCC Relative
        self.map_opcode(0xB0, bcs, absolute, 2); // BCS Absolute
        self.map_opcode(0xB0, bcs, relative, 2); // BCS Relative
        // self.map_opcode(0xF0, beq, absolute, 2); // BEQ Absolute
        // self.map_opcode(0xF0, beq, relative, 2); // BEQ Relative
        // self.map_opcode(0x30, bmi, absolute, 2); // BMI Absolute
        // self.map_opcode(0x30, bmi, relative, 2); // BMI Relative
        // self.map_opcode(0xD0, bne, absolute, 2); // BNE Absolute
        // self.map_opcode(0xD0, bne, relative, 2); // BNE Relative
        // self.map_opcode(0x10, bpl, absolute, 2); // BPL Absolute
        // self.map_opcode(0x10, bpl, relative, 2); // BPL Relative
        // self.map_opcode(0x50, bvc, absolute, 2); // BVC Absolute
        // self.map_opcode(0x50, bvc, relative, 2); // BVC Relative
        // self.map_opcode(0x70, bvs, absolute, 2); // BVS Absolute
        // self.map_opcode(0x70, bvs, relative, 2); // BVS Relative
        
        // Clear Instructions
        self.map_opcode(0x18, clc, implied, 2); // CLC Implied
        self.map_opcode(0xD8, cld, implied, 2); // CLD Implied

        // LDA instructions
        self.map_opcode(0xA9, lda, immediate, 2);     // LDA Immediate
        self.map_opcode(0xA5, lda, zero_page, 3);     // LDA Zero Page
        self.map_opcode(0xB5, lda, zero_page_x, 4);   // LDA Zero Page,X
        self.map_opcode(0xAD, lda, absolute, 4);      // LDA Absolute
        self.map_opcode(0xBD, lda, absolute_x, 4);    // LDA Absolute,X (+1 if page crossed)
        self.map_opcode(0xB9, lda, absolute_y, 4);    // LDA Absolute,Y (+1 if page crossed)
        self.map_opcode(0xA1, lda, indirect_x, 6);    // LDA Indirect,X
        self.map_opcode(0xB1, lda, indirect_y, 5);    // LDA Indirect,Y (+1 if page crossed)

        // Set Status Instructions
        self.map_opcode(0x38, sec, implied, 2); // SEC Implied
        self.map_opcode(0xF8, sed, implied, 2); // SED Implied

        // Add more instruction mappings here...
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

    /// Default handler for unimplemented instructions.
    fn unimplemented_instruction(&self, opcode: u8) {
        panic!("Unimplemented opcode: {:#X}", opcode);
    }
}
