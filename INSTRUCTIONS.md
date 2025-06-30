# Instructions

This document lists the instructions supported by the assembler.

## Line Format

```
[Label:] Instruction [Operand] [;Comment]
```

- Labels get detected by splitting the line on the first `:`.
- Comments are anything after a `;`.

## Registers

The following registers are supported:

- `A`: Accumulator
- `B`: General-purpose register

## Supported Instructions

| Instruction       | Opcode | Bytes | Description                                             |
| ----------------- | ------ | ----- | ------------------------------------------------------- |
| `NOP`             | `0x00` | 1     | No operation                                            |
| `INPUT`           | `0x04` | 1     | Input from keyboard                                     |
| `OUTPUT`          | `0x05` | 1     | Output to screen                                        |
| `JMP <addr>`      | `0x06` | 2     | Jump to address (via number or label)                   |
| `INC A`           | `0x0C` | 1     | Increment register `A`                                  |
| `INC B`           | `0x10` | 1     | Increment register `B`                                  |
| `MOV B, A`        | `0x0D` | 1     | Move value of `A` to `B`                                |
| `MOV A, B`        | `0x11` | 1     | Move value of `B` to `A`                                |
| `ADD A, B`        | `0x0E` | 1     | Add register `A` to `B` and store in `A`                |
| `HALT`            | `0x0F` | 1     | Halt program                                            |
| `SUB A, B`        | `0x12` | 1     | Subtract `B` from `A` and store in `A`                  |
| `NAND A, B`       | `0x13` | 1     | NAND `A` and `B` and store in `A`                       |
| `OR A, B`         | `0x14` | 1     | OR `A` and `B` and store in `A`                         |
| `CMP A, B`        | `0x15` | 1     | Compare `A` and `B`, if equal store `1` else `0` in `A` |
| `JZ <addr>`       | `0x16` | 2     | Jump to address if `A` is zero                          |
| `STORE A, <addr>` | `0x1A` | 2     | Store value of `A` at address                           |
| `LOAD A, <addr>`  | `0x1D` | 2     | Load value at address and store in `A`                  |
| `LOAD A, <imm>`   | `0x09` | 2     | Load immediate value and store in `A`                   |

Pseudo-instructions:

| Instruction | Description                                                                               |
| ----------- | ----------------------------------------------------------------------------------------- |
| `DB <imm>`  | Store immediate byte at the instruction's address                                         |
| `EQU <imm>` | Create a constant with the given value and replace all references to it with the constant |
| `RESB <n>`  | Reserve `n` bytes of memory at the instructions address                                   |
