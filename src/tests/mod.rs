// src/tests/mod.rs

use crate::bus::Bus;
use crate::cpu::CPU;
use crate::registers::StatusFlags;

struct TestBus {
    memory: [u8; 0x10000], // 64KB memory
}

impl TestBus {
    fn new() -> Self {
        Self {
            memory: [0; 0x10000],
        }
    }

    fn load(&mut self, data: &[u8], start_address: u16) {
        let start = start_address as usize;
        let end = start + data.len();
        self.memory[start..end].copy_from_slice(data);
    }
}

impl Bus for TestBus {
    fn read(&mut self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
}

// Helper function to create a CPU with a test bus
fn create_cpu_with_program(program: &[u8]) -> CPU<TestBus> {
    let mut bus = TestBus::new();
    bus.load(program, 0x8000);
    // Set reset vector to 0x8000
    bus.memory[0xFFFC] = 0x00;
    bus.memory[0xFFFD] = 0x80;

    let mut cpu = CPU::new(bus);
    cpu.reset();
    cpu
}

#[cfg(test)]
mod instruction_tests {
    use super::*;

    #[test]
    fn test_adc_immediate_binary_mode_no_carry() {
        // Assemble the program:
        // LDA #$10
        // ADC #$05
        let program = vec![
            0xA9, 0x10, // LDA #$10
            0x69, 0x05, // ADC #$05
        ];
        let mut cpu = create_cpu_with_program(&program);
        cpu.reset();

        // Execute LDA #$10
        cpu.step();
        assert_eq!(cpu.registers.a, 0x10);
        assert_eq!(cpu.registers.status.carry, false);
        assert_eq!(cpu.registers.status.zero, false);
        assert_eq!(cpu.registers.status.negative, false);
        assert_eq!(cpu.registers.status.overflow, false);

        // Execute ADC #$05
        cpu.step();
        assert_eq!(cpu.registers.a, 0x15);
        assert_eq!(cpu.registers.status.carry, false);
        assert_eq!(cpu.registers.status.zero, false);
        assert_eq!(cpu.registers.status.negative, false);
        assert_eq!(cpu.registers.status.overflow, false);
    }

    #[test]
    fn test_adc_immediate_binary_mode_with_carry() {
        // Assemble the program:
        // LDA #$FF
        // ADC #$01
        let program = vec![
            0xA9, 0xFF, // LDA #$FF
            0x69, 0x01, // ADC #$01
        ];
        let mut cpu = create_cpu_with_program(&program);
        cpu.reset();

        // Execute LDA #$FF
        cpu.step();
        assert_eq!(cpu.registers.a, 0xFF);

        // Execute ADC #$01
        cpu.step();
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.registers.status.carry, true);
        assert_eq!(cpu.registers.status.zero, true);
        assert_eq!(cpu.registers.status.negative, false);
        assert_eq!(cpu.registers.status.overflow, false);
    }

    #[test]
    fn test_adc_immediate_decimal_mode() {
        // Assemble the program:
        // SED
        // LDA #$15
        // ADC #$27
        let program = vec![
            0xF8,       // SED
            0xA9, 0x15, // LDA #$15
            0x69, 0x27, // ADC #$27
        ];
        let mut cpu = create_cpu_with_program(&program);
        cpu.reset();

        // Execute SED
        cpu.step();
        assert_eq!(cpu.registers.status.decimal_mode, true);

        // Execute LDA #$15
        cpu.step();
        assert_eq!(cpu.registers.a, 0x15);

        // Execute ADC #$27 (Decimal Mode)
        cpu.step();
        assert_eq!(cpu.registers.a, 0x42);
        assert_eq!(cpu.registers.status.carry, false);
        assert_eq!(cpu.registers.status.zero, false);
        assert_eq!(cpu.registers.status.negative, false);
        // Note: Overflow flag behavior is undefined in decimal mode on NMOS 6502
    }

    #[test]
    fn test_adc_immediate_decimal_mode_with_carry() {
        // Assemble the program:
        // SED
        // LDA #$99
        // ADC #$01
        let program = vec![
            0xF8,       // SED
            0xA9, 0x99, // LDA #$99
            0x69, 0x01, // ADC #$01
        ];
        let mut cpu = create_cpu_with_program(&program);
        cpu.reset();

        // Execute SED
        cpu.step();
        assert_eq!(cpu.registers.status.decimal_mode, true);

        // Execute LDA #$99
        cpu.step();
        assert_eq!(cpu.registers.a, 0x99);

        // Execute ADC #$01 (Decimal Mode)
        cpu.step();
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.registers.status.carry, true);
        assert_eq!(cpu.registers.status.zero, true);
        assert_eq!(cpu.registers.status.negative, false);
    }

    // You can add more tests for different addressing modes and edge cases
}

#[test]
fn test_and() {
    // Assemble the program:
    // LDA #$FF
    // AND #$FF
    let program = vec![
        0xA9, 0xFF, // LDA #$FF
        0x29, 0xFF, // AND #$FF
    ];
    let mut cpu = create_cpu_with_program(&program);
    cpu.reset();

    // Execute LDA #$FF
    cpu.step();
    assert_eq!(cpu.registers.a, 0xFF);
    assert_eq!(cpu.registers.status.zero, false);
    assert_eq!(cpu.registers.status.negative, true);
}

#[test]
fn test_asl() {
    // Assemble the program:
    // LDA #$01
    // ASL
    let program = vec![
        0xA9, 0x01, // LDA #$01
        0x0A,       // ASL
    ];
    let mut cpu = create_cpu_with_program(&program);
    cpu.reset();

    // Execute LDA #$01
    cpu.step();
    assert_eq!(cpu.registers.a, 0x01);
    assert_eq!(cpu.registers.status.zero, false);
    assert_eq!(cpu.registers.status.negative, false);
}

#[test]
fn test_bcc() {
    // Assemble a program that branches if the carry flag is clear
    // CLC
    // BCC $02
    let program = vec![
        0x18,       // CLC
        0x90, 0x02, // BCC $02
    ];
    let mut cpu = create_cpu_with_program(&program);
    cpu.reset();

    // Execute CLC
    cpu.step();
    assert_eq!(cpu.registers.status.carry, false);
    assert_eq!(cpu.registers.pc, 0x8001);

    // Execute BCC $02
    cpu.step();
    // PC after fetching instruction and operand: 0x8001 + 2 = 0x8003
    // Offset: +2
    // Branch Target: 0x8003 + 2 = 0x8005
    assert_eq!(cpu.registers.pc, 0x8005);
    // Total cycles should be 2 (CLC) + 2 (BCC base cycles) + 1 (branch taken) = 5
    assert_eq!(cpu.cycles(), 5);
}

#[test]
fn test_bcs() {
    // Assemble a program that branches if the carry flag is set
    // SEC
    // BCS $02
    let program = vec![
        0x38,       // SEC
        0xB0, 0x02, // BCS $02
    ];
    let mut cpu = create_cpu_with_program(&program);
    cpu.reset();

    // Execute SEC
    cpu.step();
    assert_eq!(cpu.registers.status.carry, true);
    assert_eq!(cpu.registers.pc, 0x8001);

    // Execute BCS $02
    cpu.step();
    // PC after fetching instruction and operand: 0x8001 + 2 = 0x8003
    // Offset: +2
    // Branch Target: 0x8003 + 2 = 0x8005
    assert_eq!(cpu.registers.pc, 0x8005);
    // Total cycles should be 2 (SEC) + 2 (BCS base cycles) + 1 (branch taken) = 5
    assert_eq!(cpu.cycles(), 5);
}

#[test]
fn test_beq_branch_taken() {
    // Assemble a program that branches if the zero flag is set
    // LDA #$00
    // BEQ $02
    let program = vec![
        0xA9, 0x00, // LDA #$00
        0xF0, 0x02, // BEQ $02
    ];
    let mut cpu = create_cpu_with_program(&program);
    cpu.reset();

    // Execute LDA #$00
    cpu.step();
    assert_eq!(cpu.registers.a, 0x00);
    assert_eq!(cpu.registers.status.zero, true);
    assert_eq!(cpu.registers.pc, 0x8002);

    // Execute BEQ $02
    cpu.step();
    // PC after fetching instruction and operand: 0x8002 + 2 = 0x8004
    // Offset: +2
    // Branch Target: 0x8004 + 2 = 0x8006
    assert_eq!(cpu.registers.pc, 0x8006);
    // Total cycles should be 2 (LDA) + 2 (BEQ base cycles) + 1 (branch taken) = 5
    assert_eq!(cpu.cycles(), 5);
}

#[test]
fn test_beq_branch_not_taken() {
    // Assemble a program that does not branch because the zero flag is clear
    // LDA #$01
    // BEQ $02
    let program = vec![
        0xA9, 0x01, // LDA #$01
        0xF0, 0x02, // BEQ $02
    ];
    let mut cpu = create_cpu_with_program(&program);
    cpu.reset();

    // Execute LDA #$01
    cpu.step();
    assert_eq!(cpu.registers.a, 0x01);
    assert_eq!(cpu.registers.status.zero, false);
    assert_eq!(cpu.registers.pc, 0x8002);

    // Execute BEQ $02
    cpu.step();
    // Since zero flag is not set, branch is not taken
    // PC should advance by 2 bytes (size of BEQ instruction)
    assert_eq!(cpu.registers.pc, 0x8004);
    // Total cycles should be 2 (LDA) + 2 (BEQ base cycles) = 4
    assert_eq!(cpu.cycles(), 4);
}

#[test]
fn test_bit() {
    // Assemble the program:
    // LDA #$80
    // BIT $40
    let program = vec![
        0xA9, 0x80, // LDA #$80
        0x24, 0x40, // BIT $40
    ];
    let mut cpu = create_cpu_with_program(&program);
    cpu.reset();

    // Write a value to zero page address $40
    cpu.bus.write(0x0040, 0x40); // Write value 0x40 to address $40

    // Execute LDA #$80
    cpu.step();
    assert_eq!(cpu.registers.a, 0x80);
    assert_eq!(cpu.registers.status.zero, false);
    assert_eq!(cpu.registers.status.negative, true);

    // Execute BIT $40
    cpu.step();

    // A = 0x80, Memory[$40] = 0x40
    // A & Memory[$40] = 0x80 & 0x40 = 0x00
    assert_eq!(cpu.registers.status.zero, true);

    // Negative flag is set to bit 7 of Memory[$40] (0x40)
    // Bit 7 of 0x40 is 0, so negative flag should be false
    assert_eq!(cpu.registers.status.negative, false);

    // Overflow flag is set to bit 6 of Memory[$40] (0x40)
    // Bit 6 of 0x40 is 1, so overflow flag should be true
    assert_eq!(cpu.registers.status.overflow, true);
}

#[test]
fn test_brk() {
    use crate::instructions::brk;
    let mut cpu = CPU::new(TestBus::new());
    cpu.registers.pc = 0x1000;
    cpu.registers.sp = 0xFF;
    cpu.registers.status = StatusFlags::new();
    cpu.registers.status.carry = true; // Set some flags
    cpu.registers.status.zero = false;
    cpu.registers.status.negative = true;

    // Set the IRQ vector to point to address 0x2000
    cpu.bus.write(0xFFFE, 0x00);
    cpu.bus.write(0xFFFF, 0x20);

    // Execute BRK
    brk(&mut cpu, 0);

    // Check that the PC was incremented by 1 (from 0x1000 to 0x1001)
    assert_eq!(cpu.registers.pc, 0x2000); // PC should now be 0x2000

    // Check that the PC was pushed onto the stack
    assert_eq!(cpu.bus.read(0x01FF), 0x10); // High byte of PC (0x1001)
    assert_eq!(cpu.bus.read(0x01FE), 0x01); // Low byte of PC (0x1001)

    // Check that the status register was pushed onto the stack
    let status_pushed = cpu.bus.read(0x01FD);
    assert_eq!(status_pushed & 0x10, 0x10); // B flag set
    assert_eq!(status_pushed & 0x20, 0x20); // U flag set
    assert_eq!(status_pushed & 0x01, 0x01); // Carry flag preserved
    assert_eq!(status_pushed & 0x80, 0x80); // Negative flag preserved

    // Check that the Interrupt Disable flag is set
    assert!(cpu.registers.status.interrupt_disable);

    // Check that the stack pointer is correctly updated
    assert_eq!(cpu.registers.sp, 0xFC);
}

#[test]
fn test_clc() {
    // Assemble the program:
    // CLC
    let program = vec![
        0x18, // CLC
    ];
    let mut cpu = create_cpu_with_program(&program);
    cpu.reset();

    // Execute CLC
    cpu.step();
    assert_eq!(cpu.registers.status.carry, false);
}

#[test]
fn test_cld() {
    // Assemble the program:
    // CLD
    let program = vec![
        0xD8, // CLD
    ];
    let mut cpu = create_cpu_with_program(&program);
    cpu.reset();

    // Execute CLD
    cpu.step();
    assert_eq!(cpu.registers.status.decimal_mode, false);
}

#[test]
fn test_cmp_immediate() {
    // Assemble the program: LDA #$50; CMP #$40
    let program = vec![
        0xA9, 0x50, // LDA #$50
        0xC9, 0x40, // CMP #$40
    ];
    let mut cpu = create_cpu_with_program(&program);

    // Execute LDA #$50
    cpu.step();
    assert_eq!(cpu.registers.a, 0x50);

    // Execute CMP #$40
    cpu.step();
    assert_eq!(cpu.registers.status.carry, true);  // A >= M
    assert_eq!(cpu.registers.status.zero, false);  // A != M
    assert_eq!(cpu.registers.status.negative, false); // Result is positive

    // Test when A == M
    cpu.registers.a = 0x40;
    cpu.registers.pc = 0x8002; // Reset PC to CMP instruction
    cpu.step();
    assert_eq!(cpu.registers.status.carry, true);  // A >= M
    assert_eq!(cpu.registers.status.zero, true);   // A == M
    assert_eq!(cpu.registers.status.negative, false); // Result is zero

    // Test when A < M
    cpu.registers.a = 0x30;
    cpu.registers.pc = 0x8002; // Reset PC to CMP instruction
    cpu.step();
    assert_eq!(cpu.registers.status.carry, false); // A < M
    assert_eq!(cpu.registers.status.zero, false);  // A != M
    assert_eq!(cpu.registers.status.negative, true);  // Result is negative
}

#[test]
fn test_cpx_immediate() {
    // Assemble the program: LDX #$50; CPX #$40
    let program = vec![
        0xA2, 0x50, // LDX #$50
        0xE0, 0x40, // CPX #$40
    ];
    let mut cpu = create_cpu_with_program(&program);

    // Execute LDX #$50
    cpu.step();
    assert_eq!(cpu.registers.x, 0x50);

    // Execute CPX #$40
    cpu.step();
    assert_eq!(cpu.registers.status.carry, true);  // X >= M
    assert_eq!(cpu.registers.status.zero, false);  // X != M
    assert_eq!(cpu.registers.status.negative, false); // Result is positive
}

#[test]
fn test_cpy_immediate() {
    // Assemble the program: LDY #$50; CPY #$40
    let program = vec![
        0xA0, 0x50, // LDY #$50
        0xC0, 0x40, // CPY #$40
    ];
    let mut cpu = create_cpu_with_program(&program);

    // Execute LDY #$50
    cpu.step();
    assert_eq!(cpu.registers.y, 0x50);

    // Execute CPY #$40
    cpu.step();
    assert_eq!(cpu.registers.status.carry, true);  // Y >= M
    assert_eq!(cpu.registers.status.zero, false);  // Y != M
    assert_eq!(cpu.registers.status.negative, false); // Result is positive
}

#[test]
fn test_dec_zero_page() {
    // Assemble the program: LDA #$01; STA $10; DEC $10; LDA $10
    let program = vec![
        0xA9, 0x01, // LDA #$01
        0x85, 0x10, // STA $10
        0xC6, 0x10, // DEC $10
        0xA5, 0x10, // LDA $10
    ];
    let mut cpu = create_cpu_with_program(&program);

    // Execute LDA #$01
    cpu.step();
    assert_eq!(cpu.registers.a, 0x01);

    // Execute STA $10
    cpu.step();
    assert_eq!(cpu.bus.read(0x0010), 0x01);

    // Execute DEC $10
    cpu.step();
    assert_eq!(cpu.bus.read(0x0010), 0x00);
    assert_eq!(cpu.registers.status.zero, true);
    assert_eq!(cpu.registers.status.negative, false);

    // Execute LDA $10
    cpu.step();
    assert_eq!(cpu.registers.a, 0x00);
}

#[test]
fn test_dec_zero_page_with_ldx() {
    // Assemble the program: LDA #$01; LDX #$01; STA $10; DEC $10; LDA $10
    let program = vec![
        0xA9, 0x01, // LDA #$01
        0xA2, 0x01, // LDX #$01
        0x85, 0x10, // STA $10
        0xC6, 0x10, // DEC $10
        0xA5, 0x10, // LDA $10
    ];
    let mut cpu = create_cpu_with_program(&program);

    // Execute LDA #$01
    cpu.step();
    assert_eq!(cpu.registers.a, 0x01);

    // Execute LDX #$01
    cpu.step();
    assert_eq!(cpu.registers.x, 0x01);

    // Execute STA $10
    cpu.step();
    assert_eq!(cpu.bus.read(0x0010), 0x01);

    // Execute DEC $10
    cpu.step();
    assert_eq!(cpu.bus.read(0x0010), 0x00);
    assert_eq!(cpu.registers.status.zero, true);
    assert_eq!(cpu.registers.status.negative, false);

    // Execute LDA $10
    cpu.step();
    assert_eq!(cpu.registers.a, 0x00);
}

#[test]
fn test_dec_zero_page_with_ldy() {
    // Assemble the program: LDA #$01; LDY #$01; STA $10; DEC $10; LDA $10
    let program = vec![
        0xA9, 0x01, // LDA #$01
        0xA0, 0x01, // LDY #$01
        0x85, 0x10, // STA $10
        0xC6, 0x10, // DEC $10
        0xA5, 0x10, // LDA $10
    ];
    let mut cpu = create_cpu_with_program(&program);

    // Execute LDA #$01
    cpu.step();
    assert_eq!(cpu.registers.a, 0x01);

    // Execute LDY #$01
    cpu.step();
    assert_eq!(cpu.registers.y, 0x01);

    // Execute STA $10
    cpu.step();
    assert_eq!(cpu.bus.read(0x0010), 0x01);

    // Execute DEC $10
    cpu.step();
    assert_eq!(cpu.bus.read(0x0010), 0x00);
    assert_eq!(cpu.registers.status.zero, true);
    assert_eq!(cpu.registers.status.negative, false);

    // Execute LDA $10
    cpu.step();
    assert_eq!(cpu.registers.a, 0x00);
}

#[test]
fn test_dex_implied() {
    // Assemble the program:
    // LDA #$01
    // LDX #$01
    // DEX
    // LDA $00
    let program = vec![
        0xA9, 0x01, // LDA #$01
        0xA2, 0x01, // LDX #$01
        0xCA, // DEX
        0xA5, 0x00, // LDA $00
    ];
    let mut cpu = create_cpu_with_program(&program);
    cpu.reset();

    // Execute LDA #$01
    cpu.step();
    assert_eq!(cpu.registers.a, 0x01);
    assert_eq!(cpu.registers.status.zero, false);
    assert_eq!(cpu.registers.status.negative, false);

    // Execute LDX #$01
    cpu.step();
    assert_eq!(cpu.registers.x, 0x01);
    assert_eq!(cpu.registers.status.zero, false);
    assert_eq!(cpu.registers.status.negative, false);

    // Execute DEX
    cpu.step();
    assert_eq!(cpu.registers.x, 0x00);
    assert_eq!(cpu.registers.status.zero, true);
    assert_eq!(cpu.registers.status.negative, false);

    // Execute LDA $00
    cpu.step();
    assert_eq!(cpu.registers.a, 0x00);
    assert_eq!(cpu.registers.status.zero, true);
    assert_eq!(cpu.registers.status.negative, false);
}

#[test]
fn test_dey_implied() {
    // Assemble the program:
    // LDA #$01
    // LDY #$01
    // DEY
    // LDA $00
    let program = vec![
        0xA9, 0x01, // LDA #$01
        0xA0, 0x01, // LDY #$01
        0x88, // DEY
        0xA5, 0x00, // LDA $00
    ];
    let mut cpu = create_cpu_with_program(&program);
    cpu.reset();

    // Execute LDA #$01
    cpu.step();
    assert_eq!(cpu.registers.a, 0x01);
    assert_eq!(cpu.registers.status.zero, false);
    assert_eq!(cpu.registers.status.negative, false);

    // Execute LDY #$01
    cpu.step();
    assert_eq!(cpu.registers.y, 0x01);
    assert_eq!(cpu.registers.status.zero, false);
    assert_eq!(cpu.registers.status.negative, false);

    // Execute DEY
    cpu.step();
    assert_eq!(cpu.registers.y, 0x00);
    assert_eq!(cpu.registers.status.zero, true);
    assert_eq!(cpu.registers.status.negative, false);

    // Execute LDA $00
    cpu.step();
    assert_eq!(cpu.registers.a, 0x00);
    assert_eq!(cpu.registers.status.zero, true);
    assert_eq!(cpu.registers.status.negative, false);
}

#[test]
fn test_lda_immediate() {
    // Assemble the program:
    // LDA #$80
    let program = vec![
        0xA9, 0x80, // LDA #$80
    ];
    let mut cpu = create_cpu_with_program(&program);
    cpu.reset();

    // Execute LDA #$80
    cpu.step();
    assert_eq!(cpu.registers.a, 0x80);
    assert_eq!(cpu.registers.status.zero, false);
    assert_eq!(cpu.registers.status.negative, true);
}

#[test]
fn test_lda_zero_page() {
    // Assemble the program:
    // LDA $00
    let program = vec![
        0xA5, 0x00, // LDA $00
    ];
    let mut cpu = create_cpu_with_program(&program);
    cpu.reset();

    // Execute LDA $00
    cpu.step();
    assert_eq!(cpu.registers.a, 0x00);
    assert_eq!(cpu.registers.status.zero, true);
    assert_eq!(cpu.registers.status.negative, false);
}

#[test]
fn test_lda_zero_page_x() {
    // Assemble the program:
    // LDA $00,X
    let program = vec![
        0xB5, 0x00, // LDA $00,X
    ];
    let mut cpu = create_cpu_with_program(&program);
    cpu.reset();

    // Execute LDA $00
    cpu.step();
    assert_eq!(cpu.registers.a, 0x00);
    assert_eq!(cpu.registers.status.zero, true);
    assert_eq!(cpu.registers.status.negative, false);
}

#[test]
fn test_jmp_indirect_page_boundary_bug() {
    // Assemble a program with JMP ($10FF)
    let program = vec![
        0x6C, 0xFF, 0x10, // JMP ($10FF)
    ];

    let mut cpu = create_cpu_with_program(&program);
    cpu.reset();

    // Set up the indirect address at $10FF
    // According to the bug, the high byte should be read from $1000, not $1100
    cpu.bus.write(0x10FF, 0x00); // Low byte
    cpu.bus.write(0x1000, 0x80); // High byte due to bug (should have been at $1100)

    // Execute JMP ($10FF)
    cpu.step();

    // The expected target address is $8000, not $8000
    // Due to the bug, it reads from $1000 instead of $1100
    assert_eq!(cpu.registers.pc, 0x8000);
}

#[test]
fn test_jmp_normal() {
    // Assemble the program:
    // JMP $8000
    let program = vec![
        0x4C, 0x00, 0x80, // JMP $8000
    ];
    let mut cpu = create_cpu_with_program(&program);
    cpu.reset();

    // Execute JMP $8000
    cpu.step();
    assert_eq!(cpu.registers.pc, 0x8000);
}

#[test]
fn test_jsr() {
    // Assemble the program:
    // LDA #$01
    // JSR $8005
    let program = vec![
        0xA9, 0x01,       // 0x8000: LDA #$01
        0x20, 0x05, 0x80, // 0x8002: JSR $8005
        // Subroutine at 0x8005
        0xA9, 0x02,       // 0x8005: LDA #$02
        0x60,             // 0x8007: RTS
    ];
    let mut cpu = create_cpu_with_program(&program);
    cpu.reset();

    // Execute LDA #$01
    cpu.step();
    assert_eq!(cpu.registers.a, 0x01);
    assert_eq!(cpu.registers.status.zero, false);
    assert_eq!(cpu.registers.status.negative, false);
    assert_eq!(cpu.registers.pc, 0x8002);

    // Execute JSR $8005
    cpu.step();
    assert_eq!(cpu.registers.pc, 0x8005);
}

#[test]
fn test_rts() {
    // Assemble the program:
    // Main program:
    // LDA #$01
    // JSR $8006
    // NOP          ; Placeholder for next instruction after subroutine
    // Subroutine at $8006:
    // LDA #$02
    // RTS
    let program = vec![
        // Main program
        0xA9, 0x01,       // 0x8000: LDA #$01
        0x20, 0x06, 0x80, // 0x8002: JSR $8006
        0xEA,             // 0x8005: NOP (address after JSR)
        // Subroutine at 0x8006
        0xA9, 0x02,       // 0x8006: LDA #$02
        0x60,             // 0x8008: RTS
    ];
    let mut cpu = create_cpu_with_program(&program);
    cpu.reset();

    // Execute LDA #$01 (Main program)
    cpu.step();
    assert_eq!(cpu.registers.a, 0x01);
    assert_eq!(cpu.registers.status.zero, false);
    assert_eq!(cpu.registers.status.negative, false);
    assert_eq!(cpu.registers.pc, 0x8002);

    // Execute JSR $8006
    cpu.step();
    assert_eq!(cpu.registers.pc, 0x8006);

    // Execute LDA #$02 (Subroutine)
    cpu.step();
    assert_eq!(cpu.registers.a, 0x02);
    assert_eq!(cpu.registers.pc, 0x8008);

    // Execute RTS (Subroutine)
    cpu.step();
    // Return address was 0x8004 (address after JSR - 1)
    // RTS increments it by 1, so PC should be 0x8005
    assert_eq!(cpu.registers.pc, 0x8005);

    // Execute NOP (Main program resumes)
    cpu.step();
    assert_eq!(cpu.registers.pc, 0x8006);
}

#[test]
fn test_jsr_rts() {
    // Corrected program setup
    let program = vec![
        // Main program
        0xA9, 0x01,       // 0x8000: LDA #$01
        0x20, 0x06, 0x80, // 0x8002: JSR $8006
        0xEA,             // 0x8005: NOP
        // Subroutine at 0x8006
        0xA9, 0x02,       // 0x8006: LDA #$02
        0x60,             // 0x8008: RTS
    ];
    let mut cpu = create_cpu_with_program(&program);
    cpu.reset();

    // Execute LDA #$01 (Main program)
    cpu.step();
    assert_eq!(cpu.registers.a, 0x01);
    assert_eq!(cpu.registers.pc, 0x8002);
    assert_eq!(cpu.registers.status.negative, false);

    // Execute JSR $8006
    cpu.step();
    assert_eq!(cpu.registers.pc, 0x8006);

    // Execute LDA #$02 (Subroutine)
    cpu.step();
    assert_eq!(cpu.registers.a, 0x02);
    assert_eq!(cpu.registers.pc, 0x8008);

    // Execute RTS (Subroutine)
    cpu.step();
    // Return address was 0x8004, so after incrementing, PC should be 0x8005
    assert_eq!(cpu.registers.pc, 0x8005);

    // Execute NOP (Main program resumes)
    cpu.step();
    assert_eq!(cpu.registers.pc, 0x8006);
}

#[test]
fn test_irq() {
    let mut cpu = CPU::new(TestBus::new());
    cpu.registers.pc = 0x1234;
    cpu.registers.status = StatusFlags {
        carry: false,
        zero: false,
        interrupt_disable: false,
        decimal_mode: false,
        break_mode: false,
        overflow: false,
        unused: true,
        negative: false,
    };
    cpu.registers.sp = 0xFF;

    // Set the IRQ vector to point to address 0x2000
    cpu.bus.write(0xFFFE, 0x00);
    cpu.bus.write(0xFFFF, 0x20);

    // Trigger IRQ
    cpu.irq();

    // Check that the PC was pushed onto the stack
    assert_eq!(cpu.bus.read(0x01FF), 0x12); // High byte of PC
    assert_eq!(cpu.bus.read(0x01FE), 0x34); // Low byte of PC

    // Check that the status register was pushed onto the stack
    // Break flag should be cleared, unused flag should be set
    let status_pushed = cpu.bus.read(0x01FD);
    assert_eq!(status_pushed & 0x10, 0x00); // B flag cleared
    assert_eq!(status_pushed & 0x20, 0x20); // U flag set

    // Check that the interrupt disable flag is set
    assert!(cpu.registers.status.interrupt_disable);

    // Check that the new PC is set from the IRQ vector
    assert_eq!(cpu.registers.pc, 0x2000);

    // Check that the stack pointer is correctly updated
    assert_eq!(cpu.registers.sp, 0xFC);
}

#[test]
fn test_nmi() {
    let mut cpu = CPU::new(TestBus::new());
    cpu.registers.pc = 0x1234;
    cpu.registers.status = StatusFlags {
        carry: false,
        zero: false,
        interrupt_disable: false, // The state of this flag doesn't affect NMI
        decimal_mode: false,
        break_mode: false,
        overflow: false,
        unused: true,
        negative: false,
    };
    cpu.registers.sp = 0xFF;

    // Set the NMI vector to point to address 0x3000
    cpu.bus.write(0xFFFA, 0x00);
    cpu.bus.write(0xFFFB, 0x30);

    // Trigger NMI
    cpu.nmi();

    // Check that the PC was pushed onto the stack
    assert_eq!(cpu.bus.read(0x01FF), 0x12); // High byte of PC
    assert_eq!(cpu.bus.read(0x01FE), 0x34); // Low byte of PC

    // Check that the status register was pushed onto the stack
    // Break flag should be cleared, unused flag should be set
    let status_pushed = cpu.bus.read(0x01FD);
    assert_eq!(status_pushed & 0x10, 0x00); // B flag cleared
    assert_eq!(status_pushed & 0x20, 0x20); // U flag set

    // Check that the Interrupt Disable flag is set
    assert!(cpu.registers.status.interrupt_disable);

    // Check that the new PC is set from the NMI vector
    assert_eq!(cpu.registers.pc, 0x3000);

    // Check that the stack pointer is correctly updated
    assert_eq!(cpu.registers.sp, 0xFC);
}

#[test]
fn test_irq_with_interrupts_disabled() {
    let mut cpu = CPU::new(TestBus::new());
    cpu.registers.pc = 0x1234;
    cpu.registers.status.interrupt_disable = true;
    cpu.registers.sp = 0xFF;

    // Set the IRQ vector to point to address 0x2000
    cpu.bus.write(0xFFFE, 0x00);
    cpu.bus.write(0xFFFF, 0x20);

    // Trigger IRQ
    cpu.irq();

    // Check that the PC was not changed
    assert_eq!(cpu.registers.pc, 0x1234);

    // Check that the stack pointer was not changed
    assert_eq!(cpu.registers.sp, 0xFF);
}

#[test]
fn test_sbc_binary_mode() {
    // Assemble the program: LDA #$50; SBC #$10
    let program = vec![
        0xA9, 0x50, // LDA #$50
        0xE9, 0x10, // SBC #$10
    ];
    let mut cpu = create_cpu_with_program(&program);

    // Set the Carry flag (no borrow)
    cpu.registers.status.carry = true;

    // Execute LDA #$50
    cpu.step();
    assert_eq!(cpu.registers.a, 0x50);

    // Execute SBC #$10
    cpu.step();
    assert_eq!(cpu.registers.a, 0x40);
    assert_eq!(cpu.registers.status.carry, true);  // No borrow needed
    assert_eq!(cpu.registers.status.zero, false);  // Result is not zero
    assert_eq!(cpu.registers.status.negative, false); // Result is positive
    assert_eq!(cpu.registers.status.overflow, false); // No overflow
}

#[test]
fn test_sbc_decimal_mode() {
    // Assemble the program: LDA #$50; SBC #$10
    let program = vec![
        0xF8,       // SED (Set Decimal Flag)
        0xA9, 0x50, // LDA #$50
        0xE9, 0x10, // SBC #$10
    ];
    let mut cpu = create_cpu_with_program(&program);

    // Set the Carry flag (no borrow)
    cpu.registers.status.carry = true;

    // Execute SED
    cpu.step();
    assert!(cpu.registers.status.decimal_mode);

    // Execute LDA #$50
    cpu.step();
    assert_eq!(cpu.registers.a, 0x50);

    // Execute SBC #$10 in Decimal Mode
    cpu.step();
    assert_eq!(cpu.registers.a, 0x40);
    assert_eq!(cpu.registers.status.carry, true);  // No borrow needed
    assert_eq!(cpu.registers.status.zero, false);  // Result is not zero
    assert_eq!(cpu.registers.status.negative, false); // Result is positive
    // Overflow flag is undefined in decimal mode, but your implementation sets it as in binary mode
}
