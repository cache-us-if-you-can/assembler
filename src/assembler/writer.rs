pub fn write_hex_output(program: &[u8], output: &Option<String>) {
    let hex_string = program
        .iter()
        .map(|byte| format!("{:02X}", byte))
        .collect::<Vec<String>>()
        .join(" ");

    match output {
        Some(filename) => {
            std::fs::write(filename, hex_string).expect("Cannot write to file");
            println!(
                "Assembled successfully. Wrote to {} ({} bytes)",
                filename,
                program.len()
            );
        }
        None => {
            println!("{}", hex_string);
        }
    }
}
