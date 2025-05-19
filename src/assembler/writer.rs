use std::fs::File;
use std::io::Write;

pub fn write_hex_output(program: &[u8], filename: &str) {
    let hex_string = program
        .iter()
        .map(|byte| format!("{:02X}", byte))
        .collect::<Vec<String>>()
        .join(" ");

    let mut file = File::create(filename).expect("Cannot create file");
    file.write_all(hex_string.as_bytes())
        .expect("Cannot write file");
}
