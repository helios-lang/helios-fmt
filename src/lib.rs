use colored::*;
use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    FormatError,
    IoError(String),
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FormatError => {
                write!(f, "Failed to format file")
            }
            Self::IoError(error) => {
                write!(f, "An IO error occurred: {}", error)
            }
        }
    }
}

fn __format_file(path: &str) -> Result<()> {
    let _source = std::fs::read_to_string(path)?;

    Ok(())
}

pub fn format_file(path: &str) {
    println!("\n{} {}\n", "Formatting".green().bold(), path.underline());

    if let Err(error) = __format_file(path) {
        let error = format!("{}", error).red().bold();
        eprintln!("{}", error);
        std::process::exit(1);
    }

    println!("{}", "Finished formatting".green().bold());
}
