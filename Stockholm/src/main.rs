mod free_files;
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
    if let Some(key) = args.key {
        free_files::decrypt_files(&key);
    } else {
        
    }
}
