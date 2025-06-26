use thiserror::Error;

#[derive(Error, Debug)]
pub enum WriteError {
    #[error("Cannot write to file {0}")]
    CannotWriteToFile(String),
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
        Some(filename) => std::fs::write(filename, hex_lines)
            .map_err(|_| WriteError::CannotWriteToFile(filename.clone()))?,
        None => println!("{}", hex_lines),
    }
    Ok(())
}
