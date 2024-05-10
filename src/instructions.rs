use crate::addressing::AddressingMode;
use crate::{Cpu, StatusFlags};

pub struct Instruction {
    pub illegal: bool,
    pub opcode: u8,
    pub name: &'static str,
    pub mode: AddressingMode,
    pub cycles: u8,
    // Function pointer to the instruction's implementation
    pub function: fn(cpu: &mut Cpu) -> u8,
}

/// List of all 6502 instructions
pub const INSTRUCTION_LIST: [Instruction; 256] = [
    Instruction {
        illegal: false, opcode: 0x00,
        name: "BRK",
        mode: AddressingMode::Immediate,
        cycles: 7,
        function: brk,
    },
    Instruction {
        illegal: false, opcode: 0x01,
        name: "ORA",
        mode: AddressingMode::IndexedIndirect,
        cycles: 6,
        function: ora,
    },
    Instruction {
        illegal: true, opcode: 0x02,
        name: "KIL",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: kil,
    },
    Instruction {
        illegal: true, opcode: 0x03,
        name: "SLO",
        mode: AddressingMode::IndexedIndirect,
        cycles: 8,
        function: slo,
    },
    Instruction {
        illegal: false, opcode: 0x04,
        name: "NOP",
        mode: AddressingMode::ZeroPage,
        cycles: 3,
        function: nop,
    },
    Instruction {
        illegal: false, opcode: 0x05,
        name: "ORA",
        mode: AddressingMode::ZeroPage,
        cycles: 3,
        function: ora,
    },
    Instruction {
        illegal: false, opcode: 0x06,
        name: "ASL",
        mode: AddressingMode::ZeroPage,
        cycles: 5,
        function: asl,
    },
    Instruction {
        illegal: true, opcode: 0x07,
        name: "SLO",
        mode: AddressingMode::ZeroPage,
        cycles: 5,
        function: slo,
    },
    Instruction {
        illegal: false, opcode: 0x08,
        name: "PHP",
        mode: AddressingMode::Implied,
        cycles: 3,
        function: php,
    },
    Instruction {
        illegal: false, opcode: 0x09,
        name: "ORA",
        mode: AddressingMode::Immediate,
        cycles: 2,
        function: ora,
    },
    Instruction {
        illegal: false, opcode: 0x0A,
        name: "ASL",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: asl,
    },
    Instruction {
        illegal: true, opcode: 0x0B,
        name: "ANC",
        mode: AddressingMode::Immediate,
        cycles: 2,
        function: anc,
    },
    Instruction {
        illegal: false, opcode: 0x0C,
        name: "NOP",
        mode: AddressingMode::Absolute,
        cycles: 4,
        function: nop,
    },
    Instruction {
        illegal: false, opcode: 0x0D,
        name: "ORA",
        mode: AddressingMode::Absolute,
        cycles: 4,
        function: ora,
    },
    Instruction {
        illegal: false, opcode: 0x0E,
        name: "ASL",
        mode: AddressingMode::Absolute,
        cycles: 6,
        function: asl,
    },
    Instruction {
        illegal: true, opcode: 0x0F,
        name: "SLO",
        mode: AddressingMode::Absolute,
        cycles: 6,
        function: slo,
    },
    Instruction {
        illegal: false, opcode: 0x10,
        name: "BPL",
        mode: AddressingMode::Relative,
        cycles: 2,
        function: bpl,
    },
    Instruction {
        illegal: false, opcode: 0x11,
        name: "ORA",
        mode: AddressingMode::IndirectIndexed,
        cycles: 5,
        function: ora,
    },
    Instruction {
        illegal: true, opcode: 0x12,
        name: "KIL",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: kil,
    },
    Instruction {
        illegal: true, opcode: 0x13,
        name: "SLO",
        mode: AddressingMode::IndirectIndexed,
        cycles: 8,
        function: slo,
    },
    Instruction {
        illegal: false, opcode: 0x14,
        name: "NOP",
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
        function: nop,
    },
    Instruction {
        illegal: false, opcode: 0x15,
        name: "ORA",
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
        function: ora,
    },
    Instruction {
        illegal: false, opcode: 0x16,
        name: "ASL",
        mode: AddressingMode::ZeroPageX,
        cycles: 6,
        function: asl,
    },
    Instruction {
        illegal: true, opcode: 0x17,
        name: "SLO",
        mode: AddressingMode::ZeroPageX,
        cycles: 6,
        function: slo,
    },
    Instruction {
        illegal: false, opcode: 0x18,
        name: "CLC",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: clc,
    },
    Instruction {
        illegal: false, opcode: 0x19,
        name: "ORA",
        mode: AddressingMode::AbsoluteY,
        cycles: 4,
        function: ora,
    },
    Instruction {
        illegal: false, opcode: 0x1A,
        name: "NOP",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: nop,
    },
    Instruction {
        illegal: true, opcode: 0x1B,
        name: "SLO",
        mode: AddressingMode::AbsoluteY,
        cycles: 7,
        function: slo,
    },
    Instruction {
        illegal: false, opcode: 0x1C,
        name: "NOP",
        mode: AddressingMode::AbsoluteX,
        cycles: 4,
        function: nop,
    },
    Instruction {
        illegal: false, opcode: 0x1D,
        name: "ORA",
        mode: AddressingMode::AbsoluteX,
        cycles: 4,
        function: ora,
    },
    Instruction {
        illegal: false, opcode: 0x1E,
        name: "ASL",
        mode: AddressingMode::AbsoluteX,
        cycles: 7,
        function: asl,
    },
    Instruction {
        illegal: true, opcode: 0x1F,
        name: "SLO",
        mode: AddressingMode::AbsoluteX,
        cycles: 7,
        function: slo,
    },
    Instruction {
        illegal: false, opcode: 0x20,
        name: "JSR",
        mode: AddressingMode::Absolute,
        cycles: 6,
        function: jsr,
    },
    Instruction {
        illegal: false, opcode: 0x21,
        name: "AND",
        mode: AddressingMode::IndexedIndirect,
        cycles: 6,
        function: and,
    },
    Instruction {
        illegal: true, opcode: 0x22,
        name: "KIL",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: kil,
    },
    Instruction {
        illegal: true, opcode: 0x23,
        name: "RLA",
        mode: AddressingMode::IndexedIndirect,
        cycles: 8,
        function: rla,
    },
    Instruction {
        illegal: false, opcode: 0x24,
        name: "BIT",
        mode: AddressingMode::ZeroPage,
        cycles: 3,
        function: bit,
    },
    Instruction {
        illegal: false, opcode: 0x25,
        name: "AND",
        mode: AddressingMode::ZeroPage,
        cycles: 3,
        function: and,
    },
    Instruction {
        illegal: false, opcode: 0x26,
        name: "ROL",
        mode: AddressingMode::ZeroPage,
        cycles: 5,
        function: rol,
    },
    Instruction {
        illegal: true, opcode: 0x27,
        name: "RLA",
        mode: AddressingMode::ZeroPage,
        cycles: 5,
        function: rla,
    },
    Instruction {
        illegal: false, opcode: 0x28,
        name: "PLP",
        mode: AddressingMode::Implied,
        cycles: 4,
        function: plp,
    },
    Instruction {
        illegal: false, opcode: 0x29,
        name: "AND",
        mode: AddressingMode::Immediate,
        cycles: 2,
        function: and,
    },
    Instruction {
        illegal: false, opcode: 0x2A,
        name: "ROL",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: rol,
    },
    Instruction {
        illegal: true, opcode: 0x2B,
        name: "ANC",
        mode: AddressingMode::Immediate,
        cycles: 2,
        function: anc,
    },
    Instruction {
        illegal: false, opcode: 0x2C,
        name: "BIT",
        mode: AddressingMode::Absolute,
        cycles: 4,
        function: bit,
    },
    Instruction {
        illegal: false, opcode: 0x2D,
        name: "AND",
        mode: AddressingMode::Absolute,
        cycles: 4,
        function: and,
    },
    Instruction {
        illegal: false, opcode: 0x2E,
        name: "ROL",
        mode: AddressingMode::Absolute,
        cycles: 6,
        function: rol,
    },
    Instruction {
        illegal: true, opcode: 0x2F,
        name: "RLA",
        mode: AddressingMode::Absolute,
        cycles: 6,
        function: rla,
    },
    Instruction {
        illegal: false, opcode: 0x30,
        name: "BMI",
        mode: AddressingMode::Relative,
        cycles: 2,
        function: bmi,
    },
    Instruction {
        illegal: false, opcode: 0x31,
        name: "AND",
        mode: AddressingMode::IndirectIndexed,
        cycles: 5,
        function: and,
    },
    Instruction {
        illegal: true, opcode: 0x32,
        name: "KIL",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: kil,
    },
    Instruction {
        illegal: true, opcode: 0x33,
        name: "RLA",
        mode: AddressingMode::IndirectIndexed,
        cycles: 8,
        function: rla,
    },
    Instruction {
        illegal: false, opcode: 0x34,
        name: "NOP",
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
        function: nop,
    },
    Instruction {
        illegal: false, opcode: 0x35,
        name: "AND",
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
        function: and,
    },
    Instruction {
        illegal: false, opcode: 0x36,
        name: "ROL",
        mode: AddressingMode::ZeroPageX,
        cycles: 6,
        function: rol,
    },
    Instruction {
        illegal: true, opcode: 0x37,
        name: "RLA",
        mode: AddressingMode::ZeroPageX,
        cycles: 6,
        function: rla,
    },
    Instruction {
        illegal: false, opcode: 0x38,
        name: "SEC",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: sec,
    },
    Instruction {
        illegal: false, opcode: 0x39,
        name: "AND",
        mode: AddressingMode::AbsoluteY,
        cycles: 4,
        function: and,
    },
    Instruction {
        illegal: false, opcode: 0x3A,
        name: "NOP",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: nop,
    },
    Instruction {
        illegal: true, opcode: 0x3B,
        name: "RLA",
        mode: AddressingMode::AbsoluteY,
        cycles: 7,
        function: rla,
    },
    Instruction {
        illegal: false, opcode: 0x3C,
        name: "NOP",
        mode: AddressingMode::AbsoluteX,
        cycles: 4,
        function: nop,
    },
    Instruction {
        illegal: false, opcode: 0x3D,
        name: "AND",
        mode: AddressingMode::AbsoluteX,
        cycles: 4,
        function: and,
    },
    Instruction {
        illegal: false, opcode: 0x3E,
        name: "ROL",
        mode: AddressingMode::AbsoluteX,
        cycles: 7,
        function: rol,
    },
    Instruction {
        illegal: true, opcode: 0x3F,
        name: "RLA",
        mode: AddressingMode::AbsoluteX,
        cycles: 7,
        function: rla,
    },
    Instruction {
        illegal: false, opcode: 0x40,
        name: "RTI",
        mode: AddressingMode::Implied,
        cycles: 6,
        function: rti,
    },
    Instruction {
        illegal: false, opcode: 0x41,
        name: "EOR",
        mode: AddressingMode::IndexedIndirect,
        cycles: 6,
        function: eor,
    },
    Instruction {
        illegal: true, opcode: 0x42,
        name: "KIL",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: kil,
    },
    Instruction {
        illegal: true, opcode: 0x43,
        name: "SRE",
        mode: AddressingMode::IndexedIndirect,
        cycles: 8,
        function: sre,
    },
    Instruction {
        illegal: false, opcode: 0x44,
        name: "NOP",
        mode: AddressingMode::ZeroPage,
        cycles: 3,
        function: nop,
    },
    Instruction {
        illegal: false, opcode: 0x45,
        name: "EOR",
        mode: AddressingMode::ZeroPage,
        cycles: 3,
        function: eor,
    },
    Instruction {
        illegal: false, opcode: 0x46,
        name: "LSR",
        mode: AddressingMode::ZeroPage,
        cycles: 5,
        function: lsr,
    },
    Instruction {
        illegal: true, opcode: 0x47,
        name: "SRE",
        mode: AddressingMode::ZeroPage,
        cycles: 5,
        function: sre,
    },
    Instruction {
        illegal: false, opcode: 0x48,
        name: "PHA",
        mode: AddressingMode::Implied,
        cycles: 3,
        function: pha,
    },
    Instruction {
        illegal: false, opcode: 0x49,
        name: "EOR",
        mode: AddressingMode::Immediate,
        cycles: 2,
        function: eor,
    },
    Instruction {
        illegal: false, opcode: 0x4A,
        name: "LSR",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: lsr,
    },
    Instruction {
        illegal: true, opcode: 0x4B,
        name: "ALR",
        mode: AddressingMode::Immediate,
        cycles: 2,
        function: alr,
    },
    Instruction {
        illegal: false, opcode: 0x4C,
        name: "JMP",
        mode: AddressingMode::Absolute,
        cycles: 3,
        function: jmp,
    },
    Instruction {
        illegal: false, opcode: 0x4D,
        name: "EOR",
        mode: AddressingMode::Absolute,
        cycles: 4,
        function: eor,
    },
    Instruction {
        illegal: false, opcode: 0x4E,
        name: "LSR",
        mode: AddressingMode::Absolute,
        cycles: 6,
        function: lsr,
    },
    Instruction {
        illegal: true, opcode: 0x4F,
        name: "SRE",
        mode: AddressingMode::Absolute,
        cycles: 6,
        function: sre,
    },
    Instruction {
        illegal: false, opcode: 0x50,
        name: "BVC",
        mode: AddressingMode::Relative,
        cycles: 2,
        function: bvc,
    },
    Instruction {
        illegal: false, opcode: 0x51,
        name: "EOR",
        mode: AddressingMode::IndirectIndexed,
        cycles: 5,
        function: eor,
    },
    Instruction {
        illegal: true, opcode: 0x52,
        name: "KIL",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: kil,
    },
    Instruction {
        illegal: true, opcode: 0x53,
        name: "SRE",
        mode: AddressingMode::IndirectIndexed,
        cycles: 8,
        function: sre,
    },
    Instruction {
        illegal: false, opcode: 0x54,
        name: "NOP",
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
        function: nop,
    },
    Instruction {
        illegal: false, opcode: 0x55,
        name: "EOR",
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
        function: eor,
    },
    Instruction {
        illegal: false, opcode: 0x56,
        name: "LSR",
        mode: AddressingMode::ZeroPageX,
        cycles: 6,
        function: lsr,
    },
    Instruction {
        illegal: true, opcode: 0x57,
        name: "SRE",
        mode: AddressingMode::ZeroPageX,
        cycles: 6,
        function: sre,
    },
    Instruction {
        illegal: false, opcode: 0x58,
        name: "CLI",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: cli,
    },
    Instruction {
        illegal: false, opcode: 0x59,
        name: "EOR",
        mode: AddressingMode::AbsoluteY,
        cycles: 4,
        function: eor,
    },
    Instruction {
        illegal: false, opcode: 0x5A,
        name: "NOP",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: nop,
    },
    Instruction {
        illegal: true, opcode: 0x5B,
        name: "SRE",
        mode: AddressingMode::AbsoluteY,
        cycles: 7,
        function: sre,
    },
    Instruction {
        illegal: false, opcode: 0x5C,
        name: "NOP",
        mode: AddressingMode::AbsoluteX,
        cycles: 4,
        function: nop,
    },
    Instruction {
        illegal: false, opcode: 0x5D,
        name: "EOR",
        mode: AddressingMode::AbsoluteX,
        cycles: 4,
        function: eor,
    },
    Instruction {
        illegal: false, opcode: 0x5E,
        name: "LSR",
        mode: AddressingMode::AbsoluteX,
        cycles: 7,
        function: lsr,
    },
    Instruction {
        illegal: true, opcode: 0x5F,
        name: "SRE",
        mode: AddressingMode::AbsoluteX,
        cycles: 7,
        function: sre,
    },
    Instruction {
        illegal: false, opcode: 0x60,
        name: "RTS",
        mode: AddressingMode::Implied,
        cycles: 6,
        function: rts,
    },
    Instruction {
        illegal: false, opcode: 0x61,
        name: "ADC",
        mode: AddressingMode::IndexedIndirect,
        cycles: 6,
        function: adc,
    },
    Instruction {
        illegal: true, opcode: 0x62,
        name: "KIL",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: kil,
    },
    Instruction {
        illegal: true, opcode: 0x63,
        name: "RRA",
        mode: AddressingMode::IndexedIndirect,
        cycles: 8,
        function: rra,
    },
    Instruction {
        illegal: false, opcode: 0x64,
        name: "NOP",
        mode: AddressingMode::ZeroPage,
        cycles: 3,
        function: nop,
    },
    Instruction {
        illegal: false, opcode: 0x65,
        name: "ADC",
        mode: AddressingMode::ZeroPage,
        cycles: 3,
        function: adc,
    },
    Instruction {
        illegal: false, opcode: 0x66,
        name: "ROR",
        mode: AddressingMode::ZeroPage,
        cycles: 5,
        function: ror,
    },
    Instruction {
        illegal: true, opcode: 0x67,
        name: "RRA",
        mode: AddressingMode::ZeroPage,
        cycles: 5,
        function: rra,
    },
    Instruction {
        illegal: false, opcode: 0x68,
        name: "PLA",
        mode: AddressingMode::Implied,
        cycles: 4,
        function: pla,
    },
    Instruction {
        illegal: false, opcode: 0x69,
        name: "ADC",
        mode: AddressingMode::Immediate,
        cycles: 2,
        function: adc,
    },
    Instruction {
        illegal: false, opcode: 0x6A,
        name: "RORA",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: ror_a,
    },
    Instruction {
        illegal: true, opcode: 0x6B,
        name: "ARR",
        mode: AddressingMode::Immediate,
        cycles: 2,
        function: arr,
    },
    Instruction {
        illegal: false, opcode: 0x6C,
        name: "JMP",
        mode: AddressingMode::Indirect,
        cycles: 5,
        function: jmp,
    },
    Instruction {
        illegal: false, opcode: 0x6D,
        name: "ADC",
        mode: AddressingMode::Absolute,
        cycles: 4,
        function: adc,
    },
    Instruction {
        illegal: false, opcode: 0x6E,
        name: "ROR",
        mode: AddressingMode::Absolute,
        cycles: 6,
        function: ror,
    },
    Instruction {
        illegal: true, opcode: 0x6F,
        name: "RRA",
        mode: AddressingMode::Absolute,
        cycles: 6,
        function: rra,
    },
    Instruction {
        illegal: false, opcode: 0x70,
        name: "BVS",
        mode: AddressingMode::Relative,
        cycles: 2,
        function: bvs,
    },
    Instruction {
        illegal: false, opcode: 0x71,
        name: "ADC",
        mode: AddressingMode::IndirectIndexed,
        cycles: 5,
        function: adc,
    },
    Instruction {
        illegal: true, opcode: 0x72,
        name: "KIL",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: kil,
    },
    Instruction {
        illegal: true, opcode: 0x73,
        name: "RRA",
        mode: AddressingMode::IndirectIndexed,
        cycles: 8,
        function: rra,
    },
    Instruction {
        illegal: false, opcode: 0x74,
        name: "NOP",
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
        function: nop,
    },
    Instruction {
        illegal: false, opcode: 0x75,
        name: "ADC",
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
        function: adc,
    },
    Instruction {
        illegal: false, opcode: 0x76,
        name: "ROR",
        mode: AddressingMode::ZeroPageX,
        cycles: 6,
        function: ror,
    },
    Instruction {
        illegal: true, opcode: 0x77,
        name: "RRA",
        mode: AddressingMode::ZeroPageX,
        cycles: 6,
        function: rra,
    },
    Instruction {
        illegal: false, opcode: 0x78,
        name: "SEI",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: sei,
    },
    Instruction {
        illegal: false, opcode: 0x79,
        name: "ADC",
        mode: AddressingMode::AbsoluteY,
        cycles: 4,
        function: adc,
    },
    Instruction {
        illegal: false, opcode: 0x7A,
        name: "NOP",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: nop,
    },
    Instruction {
        illegal: true, opcode: 0x7B,
        name: "RRA",
        mode: AddressingMode::AbsoluteY,
        cycles: 7,
        function: rra,
    },
    Instruction {
        illegal: false, opcode: 0x7C,
        name: "NOP",
        mode: AddressingMode::AbsoluteX,
        cycles: 4,
        function: nop,
    },
    Instruction {
        illegal: false, opcode: 0x7D,
        name: "ADC",
        mode: AddressingMode::AbsoluteX,
        cycles: 4,
        function: adc,
    },
    Instruction {
        illegal: false, opcode: 0x7E,
        name: "ROR",
        mode: AddressingMode::AbsoluteX,
        cycles: 7,
        function: ror,
    },
    Instruction {
        illegal: true, opcode: 0x7F,
        name: "RRA",
        mode: AddressingMode::AbsoluteX,
        cycles: 7,
        function: rra,
    },
    Instruction {
        illegal: false, opcode: 0x80,
        name: "NOP",
        mode: AddressingMode::Immediate,
        cycles: 2,
        function: nop,
    },
    Instruction {
        illegal: false, opcode: 0x81,
        name: "STA",
        mode: AddressingMode::IndexedIndirect,
        cycles: 6,
        function: sta,
    },
    Instruction {
        illegal: false, opcode: 0x82,
        name: "NOP",
        mode: AddressingMode::Immediate,
        cycles: 2,
        function: nop,
    },
    Instruction {
        illegal: true, opcode: 0x83,
        name: "SAX",
        mode: AddressingMode::IndexedIndirect,
        cycles: 6,
        function: sax,
    },
    Instruction {
        illegal: false, opcode: 0x84,
        name: "STY",
        mode: AddressingMode::ZeroPage,
        cycles: 3,
        function: sty,
    },
    Instruction {
        illegal: false, opcode: 0x85,
        name: "STA",
        mode: AddressingMode::ZeroPage,
        cycles: 3,
        function: sta,
    },
    Instruction {
        illegal: false, opcode: 0x86,
        name: "STX",
        mode: AddressingMode::ZeroPage,
        cycles: 3,
        function: stx,
    },
    Instruction {
        illegal: true, opcode: 0x87,
        name: "SAX",
        mode: AddressingMode::ZeroPage,
        cycles: 3,
        function: sax,
    },
    Instruction {
        illegal: false, opcode: 0x88,
        name: "DEY",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: dey,
    },
    Instruction {
        illegal: false, opcode: 0x89,
        name: "NOP",
        mode: AddressingMode::Immediate,
        cycles: 2,
        function: nop,
    },
    Instruction {
        illegal: false, opcode: 0x8A,
        name: "TXA",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: txa,
    },
    Instruction {
        illegal: true, opcode: 0x8B,
        name: "XAA",
        mode: AddressingMode::Immediate,
        cycles: 2,
        function: xaa,
    },
    Instruction {
        illegal: false, opcode: 0x8C,
        name: "STY",
        mode: AddressingMode::Absolute,
        cycles: 4,
        function: sty,
    },
    Instruction {
        illegal: false, opcode: 0x8D,
        name: "STA",
        mode: AddressingMode::Absolute,
        cycles: 4,
        function: sta,
    },
    Instruction {
        illegal: false, opcode: 0x8E,
        name: "STX",
        mode: AddressingMode::Absolute,
        cycles: 4,
        function: stx,
    },
    Instruction {
        illegal: true, opcode: 0x8F,
        name: "SAX",
        mode: AddressingMode::Absolute,
        cycles: 4,
        function: sax,
    },
    Instruction {
        illegal: false, opcode: 0x90,
        name: "BCC",
        mode: AddressingMode::Relative,
        cycles: 2,
        function: bcc,
    },
    Instruction {
        illegal: false, opcode: 0x91,
        name: "STA",
        mode: AddressingMode::IndirectIndexed,
        cycles: 6,
        function: sta,
    },
    Instruction {
        illegal: true, opcode: 0x92,
        name: "KIL",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: kil,
    },
    Instruction {
        illegal: true, opcode: 0x93,
        name: "AHX",
        mode: AddressingMode::IndirectIndexed,
        cycles: 6,
        function: ahx,
    },
    Instruction {
        illegal: false, opcode: 0x94,
        name: "STY",
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
        function: sty,
    },
    Instruction {
        illegal: false, opcode: 0x95,
        name: "STA",
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
        function: sta,
    },
    Instruction {
        illegal: false, opcode: 0x96,
        name: "STX",
        mode: AddressingMode::ZeroPageY,
        cycles: 4,
        function: stx,
    },
    Instruction {
        illegal: true, opcode: 0x97,
        name: "SAX",
        mode: AddressingMode::ZeroPageY,
        cycles: 4,
        function: sax,
    },
    Instruction {
        illegal: false, opcode: 0x98,
        name: "TYA",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: tya,
    },
    Instruction {
        illegal: false, opcode: 0x99,
        name: "STA",
        mode: AddressingMode::AbsoluteY,
        cycles: 5,
        function: sta,
    },
    Instruction {
        illegal: false, opcode: 0x9A,
        name: "TXS",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: txs,
    },
    Instruction {
        illegal: true, opcode: 0x9B,
        name: "TAS",
        mode: AddressingMode::AbsoluteY,
        cycles: 5,
        function: tas,
    },
    Instruction {
        illegal: true, opcode: 0x9C,
        name: "SHY",
        mode: AddressingMode::AbsoluteX,
        cycles: 5,
        function: shy,
    },
    Instruction {
        illegal: false, opcode: 0x9D,
        name: "STA",
        mode: AddressingMode::AbsoluteX,
        cycles: 5,
        function: sta,
    },
    Instruction {
        illegal: true, opcode: 0x9E,
        name: "SHX",
        mode: AddressingMode::AbsoluteY,
        cycles: 5,
        function: shx,
    },
    Instruction {
        illegal: true, opcode: 0x9F,
        name: "AHX",
        mode: AddressingMode::AbsoluteY,
        cycles: 5,
        function: ahx,
    },
    Instruction {
        illegal: false, opcode: 0xA0,
        name: "LDY",
        mode: AddressingMode::Immediate,
        cycles: 2,
        function: ldy,
    },
    Instruction {
        illegal: false, opcode: 0xA1,
        name: "LDA",
        mode: AddressingMode::IndexedIndirect,
        cycles: 6,
        function: lda,
    },
    Instruction {
        illegal: false, opcode: 0xA2,
        name: "LDX",
        mode: AddressingMode::Immediate,
        cycles: 2,
        function: ldx,
    },
    Instruction {
        illegal: true, opcode: 0xA3,
        name: "LAX",
        mode: AddressingMode::IndexedIndirect,
        cycles: 6,
        function: lax,
    },
    Instruction {
        illegal: false, opcode: 0xA4,
        name: "LDY",
        mode: AddressingMode::ZeroPage,
        cycles: 3,
        function: ldy,
    },
    Instruction {
        illegal: false, opcode: 0xA5,
        name: "LDA",
        mode: AddressingMode::ZeroPage,
        cycles: 3,
        function: lda,
    },
    Instruction {
        illegal: false, opcode: 0xA6,
        name: "LDX",
        mode: AddressingMode::ZeroPage,
        cycles: 3,
        function: ldx,
    },
    Instruction {
        illegal: true, opcode: 0xA7,
        name: "LAX",
        mode: AddressingMode::ZeroPage,
        cycles: 3,
        function: lax,
    },
    Instruction {
        illegal: false, opcode: 0xA8,
        name: "TAY",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: tay,
    },
    Instruction {
        illegal: false, opcode: 0xA9,
        name: "LDA",
        mode: AddressingMode::Immediate,
        cycles: 2,
        function: lda,
    },
    Instruction {
        illegal: false, opcode: 0xAA,
        name: "TAX",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: tax,
    },
    Instruction {
        illegal: true, opcode: 0xAB,
        name: "LAX",
        mode: AddressingMode::Immediate,
        cycles: 2,
        function: lax,
    },
    Instruction {
        illegal: false, opcode: 0xAC,
        name: "LDY",
        mode: AddressingMode::Absolute,
        cycles: 4,
        function: ldy,
    },
    Instruction {
        illegal: false, opcode: 0xAD,
        name: "LDA",
        mode: AddressingMode::Absolute,
        cycles: 4,
        function: lda,
    },
    Instruction {
        illegal: false, opcode: 0xAE,
        name: "LDX",
        mode: AddressingMode::Absolute,
        cycles: 4,
        function: ldx,
    },
    Instruction {
        illegal: true, opcode: 0xAF,
        name: "LAX",
        mode: AddressingMode::Absolute,
        cycles: 4,
        function: lax,
    },
    Instruction {
        illegal: false, opcode: 0xB0,
        name: "BCS",
        mode: AddressingMode::Relative,
        cycles: 2,
        function: bcs,
    },
    Instruction {
        illegal: false, opcode: 0xB1,
        name: "LDA",
        mode: AddressingMode::IndirectIndexed,
        cycles: 5,
        function: lda,
    },
    Instruction {
        illegal: true, opcode: 0xB2,
        name: "KIL",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: kil,
    },
    Instruction {
        illegal: true, opcode: 0xB3,
        name: "LAX",
        mode: AddressingMode::IndirectIndexed,
        cycles: 5,
        function: lax,
    },
    Instruction {
        illegal: false, opcode: 0xB4,
        name: "LDY",
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
        function: ldy,
    },
    Instruction {
        illegal: false, opcode: 0xB5,
        name: "LDA",
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
        function: lda,
    },
    Instruction {
        illegal: false, opcode: 0xB6,
        name: "LDX",
        mode: AddressingMode::ZeroPageY,
        cycles: 4,
        function: ldx,
    },
    Instruction {
        illegal: true, opcode: 0xB7,
        name: "LAX",
        mode: AddressingMode::ZeroPageY,
        cycles: 4,
        function: lax,
    },
    Instruction {
        illegal: false, opcode: 0xB8,
        name: "CLV",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: clv,
    },
    Instruction {
        illegal: false, opcode: 0xB9,
        name: "LDA",
        mode: AddressingMode::AbsoluteY,
        cycles: 4,
        function: lda,
    },
    Instruction {
        illegal: false, opcode: 0xBA,
        name: "TSX",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: tsx,
    },
    Instruction {
        illegal: true, opcode: 0xBB,
        name: "LAS",
        mode: AddressingMode::AbsoluteY,
        cycles: 4,
        function: las,
    },
    Instruction {
        illegal: false, opcode: 0xBC,
        name: "LDY",
        mode: AddressingMode::AbsoluteX,
        cycles: 4,
        function: ldy,
    },
    Instruction {
        illegal: false, opcode: 0xBD,
        name: "LDA",
        mode: AddressingMode::AbsoluteX,
        cycles: 4,
        function: lda,
    },
    Instruction {
        illegal: false, opcode: 0xBE,
        name: "LDX",
        mode: AddressingMode::AbsoluteY,
        cycles: 4,
        function: ldx,
    },
    Instruction {
        illegal: true, opcode: 0xBF,
        name: "LAX",
        mode: AddressingMode::AbsoluteY,
        cycles: 4,
        function: lax,
    },
    Instruction {
        illegal: false, opcode: 0xC0,
        name: "CPY",
        mode: AddressingMode::Immediate,
        cycles: 2,
        function: cpy,
    },
    Instruction {
        illegal: false, opcode: 0xC1,
        name: "CMP",
        mode: AddressingMode::IndexedIndirect,
        cycles: 6,
        function: cmp,
    },
    Instruction {
        illegal: false, opcode: 0xC2,
        name: "NOP",
        mode: AddressingMode::Immediate,
        cycles: 2,
        function: nop,
    },
    Instruction {
        illegal: true, opcode: 0xC3,
        name: "DCP",
        mode: AddressingMode::IndexedIndirect,
        cycles: 8,
        function: dcp,
    },
    Instruction {
        illegal: false, opcode: 0xC4,
        name: "CPY",
        mode: AddressingMode::ZeroPage,
        cycles: 3,
        function: cpy,
    },
    Instruction {
        illegal: false, opcode: 0xC5,
        name: "CMP",
        mode: AddressingMode::ZeroPage,
        cycles: 3,
        function: cmp,
    },
    Instruction {
        illegal: false, opcode: 0xC6,
        name: "DEC",
        mode: AddressingMode::ZeroPage,
        cycles: 5,
        function: dec,
    },
    Instruction {
        illegal: true, opcode: 0xC7,
        name: "DCP",
        mode: AddressingMode::ZeroPage,
        cycles: 5,
        function: dcp,
    },
    Instruction {
        illegal: false, opcode: 0xC8,
        name: "INY",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: iny,
    },
    Instruction {
        illegal: false, opcode: 0xC9,
        name: "CMP",
        mode: AddressingMode::Immediate,
        cycles: 2,
        function: cmp,
    },
    Instruction {
        illegal: false, opcode: 0xCA,
        name: "DEX",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: dex,
    },
    Instruction {
        illegal: true, opcode: 0xCB,
        name: "AXS",
        mode: AddressingMode::Immediate,
        cycles: 2,
        function: axs,
    },
    Instruction {
        illegal: false, opcode: 0xCC,
        name: "CPY",
        mode: AddressingMode::Absolute,
        cycles: 4,
        function: cpy,
    },
    Instruction {
        illegal: false, opcode: 0xCD,
        name: "CMP",
        mode: AddressingMode::Absolute,
        cycles: 4,
        function: cmp,
    },
    Instruction {
        illegal: false, opcode: 0xCE,
        name: "DEC",
        mode: AddressingMode::Absolute,
        cycles: 6,
        function: dec,
    },
    Instruction {
        illegal: true, opcode: 0xCF,
        name: "DCP",
        mode: AddressingMode::Absolute,
        cycles: 6,
        function: dcp,
    },
    Instruction {
        illegal: false, opcode: 0xD0,
        name: "BNE",
        mode: AddressingMode::Relative,
        cycles: 2,
        function: bne,
    },
    Instruction {
        illegal: false, opcode: 0xD1,
        name: "CMP",
        mode: AddressingMode::IndirectIndexed,
        cycles: 5,
        function: cmp,
    },
    Instruction {
        illegal: true, opcode: 0xD2,
        name: "KIL",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: kil,
    },
    Instruction {
        illegal: true, opcode: 0xD3,
        name: "DCP",
        mode: AddressingMode::IndirectIndexed,
        cycles: 8,
        function: dcp,
    },
    Instruction {
        illegal: false, opcode: 0xD4,
        name: "NOP",
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
        function: nop,
    },
    Instruction {
        illegal: false, opcode: 0xD5,
        name: "CMP",
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
        function: cmp,
    },
    Instruction {
        illegal: false, opcode: 0xD6,
        name: "DEC",
        mode: AddressingMode::ZeroPageX,
        cycles: 6,
        function: dec,
    },
    Instruction {
        illegal: true, opcode: 0xD7,
        name: "DCP",
        mode: AddressingMode::ZeroPageX,
        cycles: 6,
        function: dcp,
    },
    Instruction {
        illegal: false, opcode: 0xD8,
        name: "CLD",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: cld,
    },
    Instruction {
        illegal: false, opcode: 0xD9,
        name: "CMP",
        mode: AddressingMode::AbsoluteY,
        cycles: 4,
        function: cmp,
    },
    Instruction {
        illegal: false, opcode: 0xDA,
        name: "NOP",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: nop,
    },
    Instruction {
        illegal: true, opcode: 0xDB,
        name: "DCP",
        mode: AddressingMode::AbsoluteY,
        cycles: 7,
        function: dcp,
    },
    Instruction {
        illegal: false, opcode: 0xDC,
        name: "NOP",
        mode: AddressingMode::AbsoluteX,
        cycles: 4,
        function: nop,
    },
    Instruction {
        illegal: false, opcode: 0xDD,
        name: "CMP",
        mode: AddressingMode::AbsoluteX,
        cycles: 4,
        function: cmp,
    },
    Instruction {
        illegal: false, opcode: 0xDE,
        name: "DEC",
        mode: AddressingMode::AbsoluteX,
        cycles: 7,
        function: dec,
    },
    Instruction {
        illegal: false, opcode: 0xDF,
        name: "DCP",
        mode: AddressingMode::AbsoluteX,
        cycles: 7,
        function: dcp,
    },
    Instruction {
        illegal: false, opcode: 0xE0,
        name: "CPX",
        mode: AddressingMode::Immediate,
        cycles: 2,
        function: cpx,
    },
    Instruction {
        illegal: false, opcode: 0xE1,
        name: "SBC",
        mode: AddressingMode::IndexedIndirect,
        cycles: 6,
        function: sbc,
    },
    Instruction {
        illegal: false, opcode: 0xE2,
        name: "NOP",
        mode: AddressingMode::Immediate,
        cycles: 2,
        function: nop,
    },
    Instruction {
        illegal: true, opcode: 0xE3,
        name: "ISC",
        mode: AddressingMode::IndexedIndirect,
        cycles: 8,
        function: isc,
    },
    Instruction {
        illegal: false, opcode: 0xE4,
        name: "CPX",
        mode: AddressingMode::ZeroPage,
        cycles: 3,
        function: cpx,
    },
    Instruction {
        illegal: false, opcode: 0xE5,
        name: "SBC",
        mode: AddressingMode::ZeroPage,
        cycles: 3,
        function: sbc,
    },
    Instruction {
        illegal: false, opcode: 0xE6,
        name: "INC",
        mode: AddressingMode::ZeroPage,
        cycles: 5,
        function: inc,
    },
    Instruction {
        illegal: true, opcode: 0xE7,
        name: "ISC",
        mode: AddressingMode::ZeroPage,
        cycles: 5,
        function: isc,
    },
    Instruction {
        illegal: false, opcode: 0xE8,
        name: "INX",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: inx,
    },
    Instruction {
        illegal: false, opcode: 0xE9,
        name: "SBC",
        mode: AddressingMode::Immediate,
        cycles: 2,
        function: sbc,
    },
    Instruction {
        illegal: false, opcode: 0xEA,
        name: "NOP",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: nop,
    },
    Instruction {
        illegal: false, opcode: 0xEB,
        name: "SBC",
        mode: AddressingMode::Immediate,
        cycles: 2,
        function: sbc,
    },
    Instruction {
        illegal: false, opcode: 0xEC,
        name: "CPX",
        mode: AddressingMode::Absolute,
        cycles: 4,
        function: cpx,
    },
    Instruction {
        illegal: false, opcode: 0xED,
        name: "SBC",
        mode: AddressingMode::Absolute,
        cycles: 4,
        function: sbc,
    },
    Instruction {
        illegal: false, opcode: 0xEE,
        name: "INC",
        mode: AddressingMode::Absolute,
        cycles: 6,
        function: inc,
    },
    Instruction {
        illegal: true, opcode: 0xEF,
        name: "ISC",
        mode: AddressingMode::Absolute,
        cycles: 6,
        function: isc,
    },
    Instruction {
        illegal: false, opcode: 0xF0,
        name: "BEQ",
        mode: AddressingMode::Relative,
        cycles: 2,
        function: beq,
    },
    Instruction {
        illegal: false, opcode: 0xF1,
        name: "SBC",
        mode: AddressingMode::IndirectIndexed,
        cycles: 5,
        function: sbc,
    },
    Instruction {
        illegal: true, opcode: 0xF2,
        name: "KIL",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: kil,
    },
    Instruction {
        illegal: true, opcode: 0xF3,
        name: "ISC",
        mode: AddressingMode::IndirectIndexed,
        cycles: 8,
        function: isc,
    },
    Instruction {
        illegal: false, opcode: 0xF4,
        name: "NOP",
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
        function: nop,
    },
    Instruction {
        illegal: false, opcode: 0xF5,
        name: "SBC",
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
        function: sbc,
    },
    Instruction {
        illegal: false, opcode: 0xF6,
        name: "INC",
        mode: AddressingMode::ZeroPageX,
        cycles: 6,
        function: inc,
    },
    Instruction {
        illegal: true, opcode: 0xF7,
        name: "ISC",
        mode: AddressingMode::ZeroPageX,
        cycles: 6,
        function: isc,
    },
    Instruction {
        illegal: false, opcode: 0xF8,
        name: "SED",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: sed,
    },
    Instruction {
        illegal: false, opcode: 0xF9,
        name: "SBC",
        mode: AddressingMode::AbsoluteY,
        cycles: 4,
        function: sbc,
    },
    Instruction {
        illegal: false, opcode: 0xFA,
        name: "NOP",
        mode: AddressingMode::Implied,
        cycles: 2,
        function: nop,
    },
    Instruction {
        illegal: true, opcode: 0xFB,
        name: "ISC",
        mode: AddressingMode::AbsoluteY,
        cycles: 7,
        function: isc,
    },
    Instruction {
        illegal: false, opcode: 0xFC,
        name: "NOP",
        mode: AddressingMode::AbsoluteX,
        cycles: 4,
        function: nop,
    },
    Instruction {
        illegal: false, opcode: 0xFD,
        name: "SBC",
        mode: AddressingMode::AbsoluteX,
        cycles: 4,
        function: sbc,
    },
    Instruction {
        illegal: false, opcode: 0xFE,
        name: "INC",
        mode: AddressingMode::AbsoluteX,
        cycles: 7,
        function: inc,
    },
    Instruction {
        illegal: true, opcode: 0xFF,
        name: "ISC",
        mode: AddressingMode::AbsoluteX,
        cycles: 7,
        function: isc,
    },
];

pub fn get_cycles(opcode: u8) -> u8 {
    INSTRUCTION_LIST[opcode as usize].cycles
}

pub fn get_addr_mode(opcode: u8) -> AddressingMode {
    INSTRUCTION_LIST[opcode as usize].mode
}

fn adc(cpu: &mut Cpu) -> u8 {
    // TODO: Add ADC implementation
    0
}

fn and(cpu: &mut Cpu) -> u8 {
    // TODO: Add AND implementation
    0
}

fn asl(cpu: &mut Cpu) -> u8 {
    // TODO: Add ASL implementation
    0
}

fn bcc(cpu: &mut Cpu) -> u8 {
    // TODO: Add BCC implementation
    0
}

fn bcs(cpu: &mut Cpu) -> u8 {
    // TODO: Add BCS implementation
    0
}

fn beq(cpu: &mut Cpu) -> u8 {
    // TODO: Add BEQ implementation
    0
}

fn bit(cpu: &mut Cpu) -> u8 {
    // TODO: Add BIT implementation
    0
}

fn bmi(cpu: &mut Cpu) -> u8 {
    // TODO: Add BMI implementation
    0
}

fn bne(cpu: &mut Cpu) -> u8 {
    // TODO: Add BNE implementation
    0
}

fn bpl(cpu: &mut Cpu) -> u8 {
    // TODO: Add BPL implementation
    0
}

fn brk(cpu: &mut Cpu) -> u8 {
    // TODO: Add BRK implementation
    0
}

fn bvc(cpu: &mut Cpu) -> u8 {
    // TODO: Add BVC implementation
    0
}

fn bvs(cpu: &mut Cpu) -> u8 {
    // TODO: Add BVS implementation
    0
}

fn clc(cpu: &mut Cpu) -> u8 {
    // TODO: Add CLC implementation
    0
}

fn cld(cpu: &mut Cpu) -> u8 {
    // TODO: Add CLD implementation
    0
}

fn cli(cpu: &mut Cpu) -> u8 {
    // TODO: Add CLI implementation
    0
}

fn clv(cpu: &mut Cpu) -> u8 {
    // TODO: Add CLV implementation
    0
}

fn cmp(cpu: &mut Cpu) -> u8 {
    // TODO: Add CMP implementation
    0
}

fn cpx(cpu: &mut Cpu) -> u8 {
    // TODO: Add CPX implementation
    0
}

fn cpy(cpu: &mut Cpu) -> u8 {
    // TODO: Add CPY implementation
    0
}

fn dec(cpu: &mut Cpu) -> u8 {
    // TODO: Add DEC implementation
    0
}

fn dex(cpu: &mut Cpu) -> u8 {
    // TODO: Add DEX implementation
    0
}

fn dey(cpu: &mut Cpu) -> u8 {
    // TODO: Add DEY implementation
    0
}

fn eor(cpu: &mut Cpu) -> u8 {
    // TODO: Add EOR implementation
    0
}

fn inc(cpu: &mut Cpu) -> u8 {
    // TODO: Add INC implementation
    0
}

fn inx(cpu: &mut Cpu) -> u8 {
    // TODO: Add INX implementation
    0
}

fn iny(cpu: &mut Cpu) -> u8 {
    // TODO: Add INY implementation
    0
}

fn jmp(cpu: &mut Cpu) -> u8 {
    cpu.pc = cpu.addr_abs;
    0
}

fn jsr(cpu: &mut Cpu) -> u8 {
    // TODO: Add JSR implementation
    0
}

fn lda(cpu: &mut Cpu) -> u8 {
    cpu.fetch();
    cpu.a = cpu.fetched_data;
    cpu.set_zn_flags();
    1
}

fn ldx(cpu: &mut Cpu) -> u8 {
    // TODO: Add LDX implementation
    0
}

fn ldy(cpu: &mut Cpu) -> u8 {
    // TODO: Add LDY implementation
    0
}

fn lsr(cpu: &mut Cpu) -> u8 {
    // TODO: Add LSR implementation
    0
}

fn nop(cpu: &mut Cpu) -> u8 {
    // TODO: Add NOP implementation
    0
}

fn ora(cpu: &mut Cpu) -> u8 {
    // TODO: Add ORA implementation
    0
}

fn pha(cpu: &mut Cpu) -> u8 {
    // TODO: Add PHA implementation
    0
}

fn php(cpu: &mut Cpu) -> u8 {
    // TODO: Add PHP implementation
    0
}

fn pla(cpu: &mut Cpu) -> u8 {
    // TODO: Add PLA implementation
    0
}

fn plp(cpu: &mut Cpu) -> u8 {
    // TODO: Add PLP implementation
    0
}

fn rol(cpu: &mut Cpu) -> u8 {
    // TODO: Add ROL implementation
    0
}

fn ror_a(cpu: &mut Cpu) -> u8 {
    // TODO: Add ROR implementation
    0
}

fn ror(cpu: &mut Cpu) -> u8 {
    // TODO: Add ROR implementation
    0
}

fn ror_a_nmos(cpu: &mut Cpu) -> u8 {
    // TODO: Add ROR implementation
    0
}

fn ror_a_cmos(cpu: &mut Cpu) -> u8 {
    // TODO: Add ROR implementation
    0
}

fn ror_nmos(cpu: &mut Cpu) -> u8 {
    // TODO: Add ROR implementation
    0
}

fn ror_cmos(cpu: &mut Cpu) -> u8 {
    // TODO: Add ROR implementation
    0
}

fn rti(cpu: &mut Cpu) -> u8 {
    // TODO: Add RTI implementation
    0
}

fn rts(cpu: &mut Cpu) -> u8 {
    // TODO: Add RTS implementation
    0
}

fn sbc(cpu: &mut Cpu) -> u8 {
    // TODO: Add SBC implementation
    0
}

fn sec(cpu: &mut Cpu) -> u8 {
    // TODO: Add SEC implementation
    0
}

fn sed(cpu: &mut Cpu) -> u8 {
    // TODO: Add SED implementation
    0
}

fn sei(cpu: &mut Cpu) -> u8 {
    // TODO: Add SEI implementation
    0
}

fn sta(cpu: &mut Cpu) -> u8 {
    cpu.write(cpu.addr_abs, cpu.a);
    0
}

fn stx(cpu: &mut Cpu) -> u8 {
    // TODO: Add STX implementation
    0
}

fn sty(cpu: &mut Cpu) -> u8 {
    // TODO: Add STY implementation
    0
}

fn tax(cpu: &mut Cpu) -> u8 {
    // TODO: Add TAX implementation
    0
}

fn tay(cpu: &mut Cpu) -> u8 {
    // TODO: Add TAY implementation
    0
}

fn tsx(cpu: &mut Cpu) -> u8 {
    // TODO: Add TSX implementation
    0
}

fn txa(cpu: &mut Cpu) -> u8 {
    // TODO: Add TXA implementation
    0
}

fn txs(cpu: &mut Cpu) -> u8 {
    // TODO: Add TXS implementation
    0
}

fn tya(cpu: &mut Cpu) -> u8 {
    // TODO: Add TYA implementation
    0
}

/** Illegal instructions */
fn ahx(cpu: &mut Cpu) -> u8 {
    // TODO: Add AHX implementation
    0
}

fn alr(cpu: &mut Cpu) -> u8 {
    // TODO: Add ALR implementation
    0
}

fn anc(cpu: &mut Cpu) -> u8 {
    // TODO: Add ANC implementation
    0
}

fn arr(cpu: &mut Cpu) -> u8 {
    // TODO: Add ARR implementation
    0
}

fn axs(cpu: &mut Cpu) -> u8 {
    // TODO: Add AXS implementation
    0
}

fn dcp(cpu: &mut Cpu) -> u8 {
    // TODO: Add DCP implementation
    0
}

fn isc(cpu: &mut Cpu) -> u8 {
    // TODO: Add ISC implementation
    0
}

fn kil(cpu: &mut Cpu) -> u8 {
    // TODO: Add KIL implementation
    0
}

fn las(cpu: &mut Cpu) -> u8 {
    // TODO: Add LAS implementation
    0
}

fn lax(cpu: &mut Cpu) -> u8 {
    // TODO: Add LAX implementation
    0
}

fn rla(cpu: &mut Cpu) -> u8 {
    // TODO: Add RLA implementation
    0
}

fn rra(cpu: &mut Cpu) -> u8 {
    // TODO: Add RRA implementation
    0
}

fn sax(cpu: &mut Cpu) -> u8 {
    // TODO: Add SAX implementation
    0
}

fn shx(cpu: &mut Cpu) -> u8 {
    // TODO: Add SHX implementation
    0
}

fn shy(cpu: &mut Cpu) -> u8 {
    // TODO: Add SHY implementation
    0
}

fn slo(cpu: &mut Cpu) -> u8 {
    // TODO: Add SLO implementation
    0
}

fn sre(cpu: &mut Cpu) -> u8 {
    // TODO: Add SRE implementation
    0
}

fn tas(cpu: &mut Cpu) -> u8 {
    // TODO: Add TAS implementation
    0
}

fn xaa(cpu: &mut Cpu) -> u8 {
    // TODO: Add XAA implementation
    0
}