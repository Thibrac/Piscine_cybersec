use crate::utils;
use crate::Args;
use std::path::{PathBuf};
use aes::cipher::{generic_array::GenericArray, BlockDecrypt, KeyInit};
use aes::Aes256;

fn decrypt_content(path: &PathBuf, content: Vec<u8>, args: &Args) -> std::io::Result<()> {
    if let Some(key) = &args.key {
        let key_array = GenericArray::clone_from_slice(&key.as_bytes());
        let cipher = Aes256::new(&key_array);
        
    }
    Ok(())
}

pub fn free_process(path: &PathBuf, args: &Args) {
    let extension = path.extension();
    match extension {
        Some(ex) => {
            if ex == "ft" {
                let content = utils::get_file_content(path);
                match content {
                    Some(res) => {
                        if let Err(_e) = decrypt_content(path, res, args) {
                            return;
                        }
                        if !args.silent {
                            println!("🔒File decrypted: {:?}", path);
                        }
                    },
                    None => {
                        return;
                    }
                }
            }
        },
        None => {}
    }
}