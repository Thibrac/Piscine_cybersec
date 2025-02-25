use exif::Reader;
use std::fs::File;
use std::io::BufReader;

pub fn data_jpg(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mut bufreader = BufReader::new(&file);

    let exif_reader = Reader::new();
    let exif = exif_reader.read_from_container(&mut bufreader)?;

    for field in exif.fields() {
        println!("{}: {}", field.tag, field.display_value().with_unit(&exif));
    }
    Ok(())
}

pub fn data_png(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let decoder = png::Decoder::new(file);
    let reader = decoder.read_info()?;

    let (width, height) = reader.info().size();
    let (color_type, bit_depth) = reader.output_color_type();

    println!("Dimensions : {} x {} px", width, height);
    println!("Color type: {:?}", color_type);
    println!("Bit depth : {:?}", bit_depth);
    Ok(())
}

// pub fn data_gif(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {}
