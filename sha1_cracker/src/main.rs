use sha1::Digest;
use std::{env, error::Error, fs::File, io::BufRead, io::BufReader};

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 3 {
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>");
        return Ok(());
    }

    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("SHA1 hash is not valid".into());
    }

    let word_list = File::open(&args[1])?;
    let reader = BufReader::new(&word_list);

    for line in reader.lines() {
        let line = line?;
        let common_password = line.trim();

        if hash_to_crack == hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
            println!("Password FOUND: {}", common_password);
            return Ok(());
        }
    }

    println!("Password not found in this wordlist");
    Ok(())
}
