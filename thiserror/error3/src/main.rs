#[derive(thiserror::Error, Debug)]
enum MyError {
    #[error("env not found")]
    EnvironmentError(#[from] std::env::VarError),
    #[error("io error")]
    IoError(#[from] std::io::Error),
}

fn main() -> Result<(), MyError> {
    let _home = std::env::var("HOME")?;
    let _file = std::fs::File::open("foo.txt")?;
    Ok(())
}
