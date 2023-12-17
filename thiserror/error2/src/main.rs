#[derive(Debug)]
enum MyError {
    EnvironmentError(std::env::VarError),
    IoError(std::io::Error),
}

impl From<std::env::VarError> for MyError {
    fn from(err: std::env::VarError) -> Self {
        MyError::EnvironmentError(err)
    }
}

impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        MyError::IoError(err)
    }
}

fn get_env() -> Result<String, MyError> {
    let home = std::env::var("HOME")?;
    let mut path = std::path::PathBuf::from(home);
    path.push(".config");
    path.push("myapp");
    std::fs::create_dir_all(&path)?;
    path.push("config.toml");
    Ok(path.to_str().unwrap().to_string())
}

fn main() {
    match get_env() {
        Ok(path) => println!("Path: {}", path),
        Err(err) => println!("Error: {:?}", err),
    }
}
