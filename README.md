# Chip-8 Emulator in _RUST_
Yet another Chip-8 emulator - this time in _RUST_ 🦀.
_WHAT A CONCEPT._

The spec was based on the brilliant document from [Cowgods neato specification](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.1)

## Todo
- [x] All instructions (kinda)
- [x] Basic Memory structure
- [x] Basic execution of instructions
- [x] Loading ROMs from disk
- [x] Implement the rest of the instructions
- [x] Implement a display
  - [x] For the emulators output
    - [x] Able to write sprites from arbitary memory locations to display using `drw` instruction
    - [x] Able to use built in sprites for hexidecimal numbers
  - GUI for viewing the internal state of the CPU
    - Semi-completed. Currently able to view internal state of Memory/Display/CPU via `DebugDisplay.view_state()`
- [x] Support for the timers
- [x] Support for the Chip-8 16 key keyboard
  - Done I think? Need to double check this functionality. 
- Execution control
  - [x] Ability to step through execution? 
  - Modify memory locations at runtime? 
- [ ] Fancy GUI?
- [ ] Perhaps support for a basic assembly language? 👀
  - [x] Implemented a disassembler for Chip8 ROMS (see Chip8-asm)
