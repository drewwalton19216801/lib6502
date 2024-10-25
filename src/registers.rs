/// Represents the CPU registers.
pub struct Registers {
    pub a: u8,      // Accumulator
    pub x: u8,      // X Register
    pub y: u8,      // Y Register
    pub sp: u8,     // Stack Pointer
    pub pc: u16,    // Program Counter
    pub status: StatusFlags, // Status Register
}

impl Registers {
    /// Creates a new `Registers` instance with default values.
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            sp: 0xFD,
            pc: 0x0000,
            status: StatusFlags::new(),
        }
    }
}

/// Represents the status flags in the status register.
pub struct StatusFlags {
    pub negative: bool,         // N Flag
    pub overflow: bool,         // V Flag
    pub unused: bool,           // Unused (always set to 1)
    pub break_mode: bool,       // B Flag
    pub decimal_mode: bool,     // D Flag
    pub interrupt_disable: bool,// I Flag
    pub zero: bool,             // Z Flag
    pub carry: bool,            // C Flag
}

impl StatusFlags {
    /// Creates a new `StatusFlags` instance with default values.
    pub fn new() -> Self {
        Self {
            negative: false,
            overflow: false,
            unused: true,
            break_mode: false,
            decimal_mode: false,
            interrupt_disable: false,
            zero: false,
            carry: false,
        }
    }

    pub fn contains(&self, flag: StatusFlags) -> bool {
        self.negative == flag.negative
            && self.overflow == flag.overflow
            && self.unused == flag.unused
            && self.break_mode == flag.break_mode
            && self.decimal_mode == flag.decimal_mode
            && self.interrupt_disable == flag.interrupt_disable
            && self.zero == flag.zero
            && self.carry == flag.carry
    }

    /// Converts the status flags into a byte.
    pub fn to_byte(&self) -> u8 {
        (if self.negative { 1 << 7 } else { 0 })
            | (if self.overflow { 1 << 6 } else { 0 })
            | (if self.unused { 1 << 5 } else { 0 })
            | (if self.break_mode { 1 << 4 } else { 0 })
            | (if self.decimal_mode { 1 << 3 } else { 0 })
            | (if self.interrupt_disable { 1 << 2 } else { 0 })
            | (if self.zero { 1 << 1 } else { 0 })
            | (if self.carry { 1 } else { 0 })
    }

    /// Sets the status flags from a byte.
    pub fn from_byte(&mut self, byte: u8) {
        self.negative = byte & (1 << 7) != 0;
        self.overflow = byte & (1 << 6) != 0;
        self.unused = byte & (1 << 5) != 0;
        self.break_mode = byte & (1 << 4) != 0;
        self.decimal_mode = byte & (1 << 3) != 0;
        self.interrupt_disable = byte & (1 << 2) != 0;
        self.zero = byte & (1 << 1) != 0;
        self.carry = byte & 1 != 0;
    }
}
