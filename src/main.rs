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

    let parsed_lines: Result<Vec<Line>, parser::ParseError> = source
        .lines()
        .filter(|line| !line.trim().is_empty())
        .enumerate()
        .map(parser::parse_line)
        .collect();

    let parsed_lines = match parsed_lines {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
            std::process::exit(1);
        }
    };

    let corrected_lines = parser::replace_constants(&parsed_lines);
    let corrected_lines = match corrected_lines {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("Error replacing constants: {}", e);
            std::process::exit(1);
        }
    };
    let symbols = encoder::build_symbol_table(&corrected_lines);

    let program: Vec<u8> = corrected_lines
        .iter()
        .filter_map(|line| line.instruction.as_ref())
        .flat_map(|instr| encoder::assemble_instruction(instr, &symbols))
        .collect();

    writer::write_hex_output(&program, &args.output);
}
