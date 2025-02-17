use exif::Reader;
use std::fs::File;
use std::io::BufReader;

pub fn extract_exif(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mut bufreader = BufReader::new(&file);

    let exif_reader = Reader::new();
    let exif = exif_reader.read_from_container(&mut bufreader)?;

    for field in exif.fields() {
        println!("{}: {}", field.tag, field.display_value().with_unit(&exif));
    }
    Ok(())
}
