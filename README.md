# lib6502 CPU emulation library
## What is this?
It's an experimental 6502 emulation library written in Rust.
## Why is this?
Because I thought it would be fun, I was also inspired by [Ben Eater's excellent 6502 video series](https://eater.net/6502). Originally I had wanted to create a breadboard computer, but realized that I didn't have the extra cash for things like logic analyzers or oscilloscopes. Besides, emulating in software gives me flexibility to create whatever hardware I want!

## Project Status

 - [X] 100% legal opcode implementation
 - [ ] Illegal opcode support
 - [ ] 100% test coverage
 - [ ] Example implementation
 - [ ] Cycle-accurate instructions
	 - [X] Instruction-level accuracy
	 - [ ] Cycle-level accuracy

## Building
I don't know why you would want to build just this, but:

    cargo build

## Testing

    cargo test

## Running
Coming soon.

## Helpful Links
[NesDev CPU wiki](https://www.nesdev.org/wiki/CPU) - Fantastic resource for 6502 information, specifically the NES version of the 6502.

[mass:werk 6502 tools](https://www.masswerk.at/6502/) - A fully functional 6502 CPU emulator, assembler, and disassembler, as well as a great resource for 6502 instruction set internals