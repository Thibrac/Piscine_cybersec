use std::{fs::File, io::Read};

pub fn check_gfile(file_path: &str) -> Result<(bool, String), Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    if buffer.len() < 64 {
        return Ok((false, buffer));
    } else if !buffer.chars().all(|c| c.is_ascii_hexdigit()) {
        return Ok((false, buffer));
    }

    Ok((true, buffer))
}
