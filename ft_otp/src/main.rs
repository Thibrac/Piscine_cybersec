use clap::Parser;
mod logic;
mod utils;

#[derive(Parser, Debug)]
#[command(version= "1.0", about = "ft_otp", long_about = None)]
struct Args {
    #[arg(
        short = 'g',
        conflicts_with = "k_file",
        required_unless_present = "k_file"
    )]
    g_file: Option<String>,

    #[arg(
        short = 'k',
        conflicts_with = "g_file",
        required_unless_present = "g_file"
    )]
    k_file: Option<String>,
}

fn main() {
    let args = Args::parse();
    if let Some(g_file) = args.g_file {
        match utils::check_gfile(&g_file) {
            Ok((true, input)) => {
                if let Err(e) = logic::crypt_process(&input) {
                    println!("Error: {e}");
                }
            }
            Ok((false, _input)) => {
                println!("Error: key must be 64 hexadecimal characters.");
            }
            Err(e) => {
                println!("Error: {e}");
            }
        }
    } else {
        if let Some(k_file) = args.k_file {
            match logic::decrypt_process(&k_file) {
                Ok(vec) => {
                    let res_totp = logic::totp(&vec);
                    println!("{res_totp}");
                }
                Err(e) => {
                    println!("Error: {e}");
                }
            }
        }
    }
}
