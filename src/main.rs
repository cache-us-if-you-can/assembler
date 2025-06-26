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
    #[arg(short, long)]
    side_by_side: bool,
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
        .map(Line::parse)
        .collect::<Result<_, _>>()?;

    let corrected_lines = parser::replace_constants(&parsed_lines)?;
    let symbols = encoder::build_symbol_table(&corrected_lines);

    let compiled_lines: Vec<(Line, Vec<u8>)> = corrected_lines
        .into_iter()
        .map(|line| {
            let bytes = match &line.instruction {
                Some(instr) => instr.encode(line.index, &symbols)?,
                None => vec![],
            };
            Ok((line, bytes))
        })
        .collect::<Result<Vec<(Line, Vec<u8>)>, Box<dyn std::error::Error>>>()?;

    if args.side_by_side {
        writer::write_side_by_side_output(compiled_lines, &args.output)?;
    } else {
        let program: Vec<u8> = compiled_lines
            .iter()
            .flat_map(|(_, bytes)| bytes)
            .copied()
            .collect();
        writer::write_hex_output(&program, &args.output)?;
    }

    Ok(())
}
