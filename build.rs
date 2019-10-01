use std::{error::Error, fs::File, io::BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let token_reader = File::open("./TOKENS.json")
        .map(|key_file| BufReader::new(key_file))
        .expect("No 'TOKEN.json' file in the workspace folder!");
    let tokens: serde_json::Value = serde_json::from_reader(token_reader)?;
    println!("cargo:rerun-if-changed=TOKENS.json");
    println!("cargo:rustc-env=CONSUMER_KEY={}",
        tokens["consumer_key"].as_str().unwrap());
    println!("cargo:rustc-env=CONSUMER_SECRET={}",
        tokens["consumer_secret"].as_str().unwrap());
    println!("cargo:rustc-env=ACCESS_KEY={}",
        tokens["access_key"].as_str().unwrap());
    println!("cargo:rustc-env=ACCESS_SECRET={}",
        tokens["access_secret"].as_str().unwrap());

    Ok(())
}