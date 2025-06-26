# Assembler

This is a simple assembler for the **COMARCH** univeristy project. The program takes a `.asm` source file and outputs machine code as hex values, either printed to the console or saved to a file. It supports an optional side-by-side view with assembly and the compiled opcodes.
It is written in Rust and uses the [clap](https://github.com/clap-rs/clap) library for command line parsing.

## Overview

This assembler supports a simple assembly language, which includes:

- Labels (for symbolic addresses)
- Instructions with operands
- Comments (starting with `;`)
- Constants replacement
- Memory addressing up to 256 bytes

For a detailed list of instructions and opcode encodings, see the [Instructions](INSTRUCTIONS.md).

## Installation

Add the repository as a flake input in your `flake.nix`:

```nix
{
    inputs.assembler.url = "github:cache-us-if-you-can/assembler";
    # ...
}
```

Then, add the package to your environment:

```nix
{
    environment.systemPackages = [inputs.assembler.packages.${system}.default];
}
```

## Usage

Run `assembler --help` for a list of options:

```bash
Usage: assembler [OPTIONS] <INPUT> [OUTPUT]

Arguments:
  <INPUT>   Input assembly file
  [OUTPUT]  Output hex file

Options:
  -s, --side-by-side
  -h, --help          Print help
  -V, --version       Print version
```

### Examples

- Compile `input.asm` to `output.hex`:

```bash
assembler input.asm output.hex
```

`output.hex` will contain the hex values of the compiled machine code:

```bash
v3.0 hex words addressed
00: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
10: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
20: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
30: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
40: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
50: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
60: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
70: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
80: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
90: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
A0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
B0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
C0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
D0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
E0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
F0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
```

This can be directly loaded into the RAM in `logisim-evolution`.

- Side-by-side view of `input.asm`'s assembly and machine code:

```bash
assembler -s input.asm
```

This results in a table with the assembly and the corresponding machine code:

```
┌──────┬───────┬─────────────┬───────────┐
│ Line │ Label │ Instruction │ Hex Bytes │
├──────┼───────┼─────────────┼───────────┤
│ 01   │ START │ LOAD A #255 │ 09 FF     │
├──────┼───────┼─────────────┼───────────┤
│ 02   │       │ STORE A 21  │ 1A 15     │
├──────┼───────┼─────────────┼───────────┤
│ 03   │ TEST  │             │           │
├──────┼───────┼─────────────┼───────────┤
│ 04   │       │             │           │
├──────┼───────┼─────────────┼───────────┤
│ 05   │ LOOP  │ OUTPUT      │ 05        │
├──────┼───────┼─────────────┼───────────┤
│ 06   │       │ INC A       │ 0C        │
├──────┼───────┼─────────────┼───────────┤
│ 07   │       │ JZ LOOP     │ 16 04     │
├──────┼───────┼─────────────┼───────────┤
│ 08   │       │ HALT        │ 0F        │
└──────┴───────┴─────────────┴───────────┘
```
