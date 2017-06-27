use std::io;
use toml;

#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    TomlDe(toml::de::Error),
    TomlSer(toml::ser::Error),
}
