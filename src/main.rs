mod assembler;
mod types;
use clap::Parser;

use assembler::{encoder, parser, writer};
use types::*;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Input assembly file
    input: String,
    /// Output hex file
    output: Option<String>,
}

fn main() {
    let args = Args::parse();

    let source = std::fs::read_to_string(args.input).expect("Could not read file");

    let parsed_lines: Vec<Line> = source
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| parser::parse_line(line))
        .collect();

    let symbols = encoder::build_symbol_table(&parsed_lines);

    let mut program = Vec::new();
    for line in &parsed_lines {
        if let Some(instr) = &line.instruction {
            let bytes = encoder::assemble_instruction(instr, &symbols);
            program.extend(bytes);
        }
    }

    writer::write_hex_output(&program, &args.output);
    println!("Assembled successfully. Wrote {} bytes.", program.len());
}
