# CHIP-8 Emulator in Rust

A CHIP-8 interpreter written in Rust. It runs ROMs in a scaled framebuffer and
includes an optional debugger with register, stack, disassembly, and memory
views.

The emulator targets the original CHIP-8 instruction set described in
[Cowgod's technical reference][chip8-spec]. Core CPU, display, memory,
keyboard, disassembler, and debugger behavior are covered by unit tests.

## Requirements

- Rust stable toolchain
- A desktop environment supported by `minifb`

## Run a ROM

```sh
cargo run --release -- path/to/program.ch8
```

Useful options:

```sh
# Open the separate debugger window.
cargo run --release -- path/to/program.ch8 --debug

# Set the instruction rate and window scale.
cargo run --release -- path/to/program.ch8 --speed 700 --scale 8
```

`--scale` accepts `1`, `2`, `4`, `8`, `16`, or `32`.

## Controls

The CHIP-8 keypad is mapped directly to the hexadecimal keys on a standard
keyboard:

```text
1 2 3 C
4 5 6 D
7 8 9 E
A 0 B F
```

Runtime controls:

| Key | Action |
| --- | --- |
| `Esc` | Quit |
| `F1` | Write a memory dump to `chip8rs_memdump.log` |
| `F2` | Execute one instruction while paused |
| `F3` | Pause execution |
| `F4` | Resume execution |

## Development

```sh
cargo test
cargo clippy --bins --tests -- -D warnings
```

The test suite covers opcode execution, timers, display collision and wrapping
behavior, input handling, memory safety, disassembly, and debugger rendering.

## Related work

The accompanying [chip8-asm](https://github.com/SylvanB/chip8-asm) project is a
CHIP-8 assembler/disassembler.

## License

This project is licensed under the [MIT License](LICENSE).

[chip8-spec]: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
