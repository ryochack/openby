use std::fs;
use std::io;
use std::io::prelude::*;
use std::path;
use toml;
use error;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Config {
    version: f64,
    tools: Vec<Tools>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Tools {
    command: String,
    extentions: Vec<String>,
}

pub fn load_config(conf_name: &str) -> Result<Config, error::AppError> {
    let path = path::Path::new(conf_name);
    if path.exists() == false {
        return Result::Err(error::AppError::Io(io::Error::new(
            io::ErrorKind::NotFound,
            format!("{} is not exist", path.to_str().unwrap_or("")),
        )));
    }

    let mut reader = io::BufReader::new(fs::File::open(path).map_err(error::AppError::Io)?);
    let mut s = String::new();
    reader.read_to_string(&mut s).map_err(error::AppError::Io)?;

    let conf: Config = toml::from_str(s.as_str()).map_err(error::AppError::Toml)?;

    Ok(conf)
}

pub fn get_commnad(conf: &Config, extension: &str) -> Option<String> {
    let ext_string = extension.to_string();
    for t in conf.tools.iter() {
        if t.extentions.contains(&ext_string) {
            return Some(t.command.clone());
        }
    }
    None
}

#[test]
fn test_decode_toml() {
    let toml_config_str = r#"
    version = 0.0

    [[tools]]
    command = "apvlv"
    extentions = [ "pdf" ]

    [[tools]]
    command = "mirage"
    extentions = [ "jpg", "png", "gif" ]

    [[tools]]
    command = "vlc"
    extentions = [ "mp4", "mov", "avi" ]

    [[tools]]
    command = "vim -R"
    extentions = [ "conf" ]
    "#;

    let config: Config = toml::from_str(toml_config_str).unwrap();
    assert_eq!(config.version, 0.0);

    assert_eq!(config.tools[0].command, "apvlv".to_string());
    assert_eq!(config.tools[0].extentions[0], "pdf".to_string());

    assert_eq!(config.tools[1].command, "mirage".to_string());
    assert_eq!(config.tools[1].extentions[0], "jpg".to_string());
    assert_eq!(config.tools[1].extentions[1], "png".to_string());
    assert_eq!(config.tools[1].extentions[2], "gif".to_string());

    assert_eq!(config.tools[2].command, "vlc".to_string());
    assert_eq!(config.tools[2].extentions[0], "mp4".to_string());
    assert_eq!(config.tools[2].extentions[1], "mov".to_string());
    assert_eq!(config.tools[2].extentions[2], "avi".to_string());

    assert_eq!(config.tools[3].command, "vim -R".to_string());
    assert_eq!(config.tools[3].extentions[0], "conf".to_string());
}

#[test]
fn test_get_commnad() {
    let conf = Config {
        version: 0.0,
        tools: vec![
            Tools {
                command: "cat".to_string(),
                extentions: vec!["txt".to_string(), "log".to_string()],
            },
        ],
    };

    assert_eq!(get_commnad(&conf, "txt").unwrap(), "cat");
    assert_eq!(get_commnad(&conf, "log").unwrap(), "cat");
    assert_eq!(
        get_commnad(&conf, "jpg").unwrap_or("none".to_string()),
        "none"
    );
}
