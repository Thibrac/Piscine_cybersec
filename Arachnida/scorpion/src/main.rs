mod logic;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version= "1.0", about = "Scorpion", long_about = None)]
struct Args {
    #[arg(required = true)]
    files: Vec<String>,
}

fn main() {
    let args = Args::parse();
    for file in args.files {
        if let Err(e) = logic::extract_exif(&file) {
            println!("Error = {e}");
        }
    }
}
