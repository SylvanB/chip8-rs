# Chip-8 Emulator in _RUST_
Yet another Chip-8 emulator - this time in _RUST_ 🦀.
_WHAT A CONCEPT._

The spec was based on the brilliant document from [Cowgods neato specification](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.1)

## Implemented
- Most of the instructions.
- The remaining instructions are used for keyboard actions, timers, drawin to the display, and reading memory locations.
  - Remaining Instructions to be implemented:
    - Fx29 - LD F, Vx
- Basic Memory structure
- Basic execution of instructions

## Todo
- Implement the rest of the instructions
- Implement a display
  - For the emulators output
    - WIP: Able to write sprites from arbitary memory locations to display using `drw` instruction
  - For viewing the internal state of the CPU
    - Semi-completed. Currently able to view internal state of Memory/Display/CPU via `DebugDisplay.view_state()`
- Support for the timers
- Support for the Chip-8 16 key keyboard
- Execution control
  - Ability to step through execution? 
  - Modify memory locations at runtime? 
- Fancy GUI?
- Perhaps support for a basic assembly language? 👀
