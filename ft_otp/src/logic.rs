use aes::cipher::BlockDecrypt;
use aes::cipher::{generic_array::GenericArray, BlockEncrypt, KeyInit};
use aes::Aes256;
use rand::RngCore;
use sha1::{Digest, Sha1};
use std::fs::File;
use std::io::{Read, Write};
use std::time::SystemTime;

pub fn crypt_process(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let input_bytes = hex::decode(input)?;

    let mut block1 = GenericArray::clone_from_slice(&input_bytes[0..16]);
    let mut block2 = GenericArray::clone_from_slice(&input_bytes[16..32]);

    let mut key = [0u8; 32];
    rand::rng().fill_bytes(&mut key);
    let key_array = GenericArray::clone_from_slice(&key);
    let cipher = Aes256::new(&key_array);

    cipher.encrypt_block(&mut block1);
    cipher.encrypt_block(&mut block2);

    let mut dest_aes = File::create("aes.key")?;
    dest_aes.write_all(&key_array)?;
    let mut dest_otp = File::create("ft_otp.key")?;
    dest_otp.write_all(&block1)?;
    dest_otp.write_all(&block2)?;

    Ok(())
}

pub fn decrypt_process(k_file: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut aes_file = File::open("aes.key")?;
    let mut otp_file = File::open(k_file)?;

    let mut otp_blocks = [0u8; 32];
    let mut aes_key = [0u8; 32];

    otp_file.read_exact(&mut otp_blocks)?;
    aes_file.read_exact(&mut aes_key)?;

    let key_array = GenericArray::clone_from_slice(&aes_key);
    let mut otp_block1 = GenericArray::clone_from_slice(&otp_blocks[0..16]);
    let mut otp_block2 = GenericArray::clone_from_slice(&otp_blocks[16..32]);
    let cipher = Aes256::new(&key_array);

    cipher.decrypt_block(&mut otp_block1);
    cipher.decrypt_block(&mut otp_block2);

    let mut decrypted_bytes = Vec::new();
    decrypted_bytes.extend_from_slice(&otp_block1);
    decrypted_bytes.extend_from_slice(&otp_block2);

    let result = hex::encode(decrypted_bytes);

    Ok(result.into_bytes())
}

pub fn hmac_sha1(key: &Vec<u8>, message: &[u8]) -> Vec<u8> {
    let mut outer_key = vec![0x5c; 64];
    let mut inner_key = vec![0x36; 64];
    for i in 0..64 {
        outer_key[i] ^= key[i];
        inner_key[i] ^= key[i];
    }

    let mut hasher = Sha1::new();
    hasher.update(&inner_key);
    hasher.update(message);
    let result = hasher.finalize();
    let mut hmac = vec![0; 20];
    hmac.copy_from_slice(&result);

    let mut hasher = Sha1::new();
    hasher.update(&outer_key);
    hasher.update(&hmac);
    let result = hasher.finalize();
    let mut hmac = vec![0; 20];
    hmac.copy_from_slice(&result);
    hmac
}

pub fn totp(key: &Vec<u8>) -> u32 {
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let time = time / 30;
    let time = time.to_be_bytes();
    let hmac = hmac_sha1(key, &time);
    let offset = (hmac[19] & 0xf) as usize;
    let value = ((hmac[offset] as u32 & 0x7f) << 24)
        | ((hmac[offset + 1] as u32) << 16)
        | ((hmac[offset + 2] as u32) << 8)
        | (hmac[offset + 3] as u32);
    value % 1_000_000
}
