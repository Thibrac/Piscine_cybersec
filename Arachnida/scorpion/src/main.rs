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
        if file.ends_with(".jpg") {
            if let Err(e) = logic::data_jpg(&file) {
                println!("Error = {e}");
            }
        } else if file.ends_with(".jpeg") {
            if let Err(e) = logic::data_jpg(&file) {
                println!("Error = {e}");
            }
        } else if file.ends_with(".png") {
            if let Err(e) = logic::data_png(&file) {
                println!("Error = {e}");
            }
        }
        // else if file.ends_with(".gif") {
        //     if let Err(e) = logic::data_gif(&file) {
        //         println!("Error = {e}");
        //     }
        // }
    }
}
