// mod free_files;
mod atk_files;
mod utils;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version= "1.0", about= "Stockholm", long_about = None)]
struct Args {
    #[arg(short = 'r', long = "reverse")]
    key: Option<String>,

    #[arg(short = 's', long = "silent")]
    silent: bool,
}

fn main() {
    let args = Args::parse();
    if let Err(e) = utils::start_randsom(&args) {
        println!("{e}");
    }
}
