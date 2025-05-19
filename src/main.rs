mod assembler;
mod types;

use assembler::{encoder, parser, writer};
use types::*;

fn main() {
    let source = std::fs::read_to_string("input.txt").expect("Could not read file");

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

    writer::write_hex_output(&program, "output.txt");
    println!("Assembled successfully. Wrote {} bytes.", program.len());
}
