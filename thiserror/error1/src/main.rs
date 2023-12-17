use std::{fs::File, io::Read};

fn main() -> Result<(), std::io::Error> {
    let contents = read_file("Cargo.toml")?;
    println!("File contents: {}", contents);
    Ok(())
}

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
