# Assembler

This is a simple assembler for the **COMARCH** univeristy project. The program takes a `.asm` file as input and outputs a sequence of hex values to the console or a file.
It is written in Rust and uses the [clap](https://github.com/clap-rs/clap) library for command line parsing.

## Installation

Add the repository to your `flake.nix` file:

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

To run the assembler, use one of the following commands:

### Cargo (when using the development shell)

```bash
cargo run -- input.asm
cargo run -- input.asm output.txt
```

### Nix (compiled version)

```bash
nix run . -- input.asm
nix run . -- input.asm output.txt
```
