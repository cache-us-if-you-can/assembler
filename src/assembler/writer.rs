use crate::types::*;
use std::io;
use tabled::{
    Table, Tabled,
    settings::{
        Color, Modify, Style,
        object::{Columns, Rows},
    },
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WriteError {
    #[error("Cannot write to file {filename}: {source}")]
    CannotWriteToFile {
        filename: String,
        #[source]
        source: io::Error,
    },
}

pub fn write_hex_output(program: &[u8], output: &Option<String>) -> Result<(), WriteError> {
    let padded = program
        .iter()
        .copied()
        .chain(std::iter::repeat(0))
        .take(256);
    let hex_lines = std::iter::once(String::from("v3.0 hex words addressed"))
        .chain(
            padded
                .collect::<Vec<u8>>()
                .chunks(16)
                .enumerate()
                .map(|(i, chunk)| {
                    format!(
                        "{:02X}: {}",
                        i * 16,
                        chunk
                            .iter()
                            .map(|b| format!("{:02X}", b))
                            .collect::<Vec<_>>()
                            .join(" ")
                    )
                }),
        )
        .collect::<Vec<String>>()
        .join("\n");
    match output {
        Some(filename) => {
            std::fs::write(filename, hex_lines).map_err(|e| WriteError::CannotWriteToFile {
                filename: filename.clone(),
                source: e,
            })?
        }
        None => println!("{}", hex_lines),
    }
    Ok(())
}

#[derive(Tabled)]
struct DisplayLine {
    #[tabled(rename = "Line")]
    line_number: String,
    #[tabled(rename = "Label")]
    label: String,
    #[tabled(rename = "Instruction")]
    instruction: String,
    #[tabled(rename = "Hex Bytes")]
    hex: String,
}

pub fn write_side_by_side_output(
    compiled_lines: &[(Line, Vec<u8>)],
    output: &Option<String>,
) -> Result<(), WriteError> {
    let display_lines: Vec<DisplayLine> = compiled_lines
        .iter()
        .map(|(line, bytes)| {
            let line_number = format!("{:02}", line.index);
            let label = line.label.clone().unwrap_or_default();
            let instruction = line
                .instruction
                .as_ref()
                .map(|instr| format!("{}", instr))
                .unwrap_or_default();
            let hex = bytes
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ");

            DisplayLine {
                line_number,
                label,
                instruction,
                hex,
            }
        })
        .collect();

    let table = Table::new(display_lines)
        .with(Style::modern())
        .with(Modify::new(Columns::one(0)).with(Color::FG_GREEN))
        .with(Modify::new(Columns::one(1)).with(Color::FG_YELLOW))
        .with(Modify::new(Columns::one(2)).with(Color::FG_RED))
        .with(Modify::new(Columns::one(3)).with(Color::FG_MAGENTA))
        .with(Modify::new(Rows::first()).with(Color::FG_BLUE))
        .to_string();

    match output {
        Some(path) => std::fs::write(path, table).map_err(|e| WriteError::CannotWriteToFile {
            filename: path.clone(),
            source: e,
        }),
        None => {
            println!("{}", table);
            Ok(())
        }
    }
}
