use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use sha1::Digest;

const SHA1_HEX_STRING_LENGTH: usize = 40;
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage!");
        println!("SHA Cracker: txt. file");
        return Ok(());
    }

    let hash = args[2].trim();
    if hash.len() != SHA1_HEX_STRING_LENGTH {
        return Err("SHA1 Hash is not vaild".into());
    }

    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist_file);

    for line in reader.lines() {
        let line = line?;
        let common_password = line.trim();
        if hash == hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
            println!("Password Found: {}", &common_password);
            return Ok(());
        }
    }
    println!("Passwor Not Found in Wordlist");
    Ok(())
}
