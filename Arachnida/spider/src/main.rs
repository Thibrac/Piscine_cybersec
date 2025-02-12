mod logic;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version= "1.0", about = "extract all the images from a website", long_about = None)]
struct Args {
    url: String,

    #[arg(short = 'r', long = "recursive")]
    recursive: bool,

    #[arg(short = 'l', long = "depth_level", default_value_t = 5)]
    depth: u32,

    #[arg(short = 'p', long = "path", default_value = "./data/")]
    path: String,
}

fn main() {
    let args = Args::parse();
    if !args.recursive {
        match logic::check_url(&args.url) {
            Ok(response) => {
                logic::extract_img(&args.url, &args.path, &response.text().unwrap());
            }
            Err(e) => {
                println!("Error: {e}");
            }
        }
    } else {
        logic::extract_recursive(&args.url, &args.path, args.depth);
    }
}
