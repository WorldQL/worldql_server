use std::error::Error;
use std::process::Command;

fn main() {
    if let Ok(hash) = commit_hash() {
        println!("cargo:rustc-env=GIT_HASH={}", hash);

        let short_hash = &hash[..7];
        println!("cargo:rustc-env=GIT_SHORT_HASH={}", short_hash);
    }
}

fn commit_hash() -> Result<String, Box<dyn Error>> {
    let output = Command::new("git").args(&["rev-parse", "HEAD"]).output()?;
    let hash = String::from_utf8(output.stdout)?;

    Ok(hash)
}
