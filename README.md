# Chip-8 Emulator in _RUST_
Yet another Chip-8 emulator - this time in _RUST_ 🦀.
WHAT A CONCEPT.

The spec was based on the brilliant document from [Cowgods brilliantly clear specification](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.1)

## Implemented
- Most of the instructions.
- - The remaining instructions are used for keyboard actions, timers, drawin to the display, and reading memory locations.
- - Remaining Instructions:
- - - Dxyn - DRW Vx, Vy, nibble
- - - Ex9E - SKP Vx
- - - ExA1 - SKNP Vx
- - - Fx07 - LD Vx, DT
- - - Fx0A - LD Vx, K
- - - Fx15 - LD DT, Vx
- - - Fx18 - LD ST, Vx
- - - Fx29 - LD F, Vx
- - - Fx65 - LD Vx, [I]
- Basic Memory structure
- Basic execution of instructions

## Todo
- Implement the rest of the instructions
- Implement a display
- - For the emulators output
- - For viewing the internal state of the CPU
- Support for the timers
- Support for the Chip-8 16 key keyboard
- Execution control
- - Ability to step through execution? 
- - Modify memory locations at runtime? 
- Fancy GUI?
- Perhaps support for a basic assembly language? 👀
