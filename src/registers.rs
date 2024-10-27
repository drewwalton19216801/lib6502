//! The `registers` module defines the CPU registers for the 6502.

/// The `Registers` struct represents the 6502 CPU registers.
pub struct Registers {
    /// Accumulator (A)
    pub a: u8,
    /// X register
    pub x: u8,
    /// Y register
    pub y: u8,
    /// Stack pointer
    pub sp: u8,
    /// Program counter
    pub pc: u16,
    /// Status flags
    pub status: StatusFlags,
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

/// The `StatusFlags` struct represents the status flags for the 6502.
pub struct StatusFlags {
    /// N flag (bit 7)
    pub negative: bool,
    /// V flag (bit 6)
    pub overflow: bool,
    /// U flag (bit 5)
    pub unused: bool,
    /// B flag (bit 4)
    pub break_mode: bool,
    /// D flag (bit 3)
    pub decimal_mode: bool,
    /// I flag (bit 2)
    pub interrupt_disable: bool,
    /// Z flag (bit 1)
    pub zero: bool,
    /// C flag (bit 0)
    pub carry: bool,
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

    /// Checks if all flags are set to the same value as the given flags.
    ///
    /// # Arguments
    ///
    /// * `flag`: The flags to compare with.
    ///
    /// # Returns
    ///
    /// `true` if all flags are set to the same value, `false` otherwise.
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

    /// Converts the flags to a byte.
    ///
    /// # Returns
    ///
    /// A byte where each bit corresponds to a flag.
    /// The bit positions are:
    /// - 7: N flag
    /// - 6: V flag
    /// - 5: U flag (always 1)
    /// - 4: B flag
    /// - 3: D flag
    /// - 2: I flag
    /// - 1: Z flag
    /// - 0: C flag
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

    /// Sets the flags from a byte.
    ///
    /// # Arguments
    ///
    /// * `byte`: The byte to read the flags from.
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
