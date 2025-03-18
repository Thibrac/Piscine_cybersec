use crate::utils;
use crate::Args;
use std::fs::{File, rename};
use std::io::Write;
use std::path::{PathBuf};
use aes::cipher::{generic_array::GenericArray, BlockDecrypt, KeyInit};
use aes::Aes256;

fn decrypt_content(path: &PathBuf, content: Vec<u8>, args: &Args) -> std::io::Result<()> {
    if let Some(key) = &args.key {
        let key_array = GenericArray::clone_from_slice(&key.as_bytes());
        let cipher = Aes256::new(&key_array);
            
        let mut new_path = path.clone();
        new_path.set_extension("");
        let mut dest_aes = File::create(path)?;
        rename(path, new_path)?;
            
        let mut decrypted = Vec::new();
        let mut i = 0;
        while i < content.len() {
            let mut block = GenericArray::clone_from_slice(&content[i..i + 16]);
            cipher.decrypt_block(&mut block);
            decrypted.extend_from_slice(&block);
            i += 16;
        }
        if !decrypted.is_empty() {
            let value = decrypted[decrypted.len() - 1];
            let start = decrypted.len() - value as usize;
            decrypted.truncate(start);
        }
        dest_aes.write_all(&decrypted)?;
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
                            println!("🔓File decrypted: {:?}", path);
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

// Key = 00000000000000000000000000000000