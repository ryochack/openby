use std::io;
use toml;

#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    Toml(toml::de::Error),
}

