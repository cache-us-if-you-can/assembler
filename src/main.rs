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
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let source = std::fs::read_to_string(&args.input)?;

    let parsed_lines: Vec<Line> = source
        .lines()
        .filter(|line| !line.trim().is_empty())
        .zip(1..)
        .map(parser::parse_line)
        .collect::<Result<_, _>>()?;

    let corrected_lines = parser::replace_constants(&parsed_lines)?;
    let symbols = encoder::build_symbol_table(&corrected_lines);

    let program: Vec<u8> = corrected_lines
        .iter()
        .filter_map(|line| line.instruction.as_ref().map(|instr| (line.index, instr)))
        .map(|(index, instr)| encoder::assemble_instruction(index, instr, &symbols))
        .collect::<Result<Vec<Vec<u8>>, _>>()
        .map(|chunks| chunks.concat())?;

    writer::write_hex_output(&program, &args.output);

    Ok(())
}
