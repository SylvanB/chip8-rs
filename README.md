# Chip-8 Emulator in _RUST_
Yet another Chip-8 emulator - this time in _RUST_ 🦀.
_WHAT A CONCEPT._

The spec was based on the brilliant document from [Cowgods neato specification](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.1)

## Implemented
- All instructions (kinda)
- Basic Memory structure
- Basic execution of instructions

## Todo
- [x] Implement the rest of the instructions
- [x] Implement a display
  - [x] For the emulators output
    - [x] Able to write sprites from arbitary memory locations to display using `drw` instruction
    - [ ] Able to use built in sprites for hexidecimal numbers
  - For viewing the internal state of the CPU
    - Semi-completed. Currently able to view internal state of Memory/Display/CPU via `DebugDisplay.view_state()`
- Support for the timers
- Support for the Chip-8 16 key keyboard
- Execution control
  - Ability to step through execution? 
  - Modify memory locations at runtime? 
- Fancy GUI?
- Perhaps support for a basic assembly language? 👀
