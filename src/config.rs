use std::fs;
use std::io;
use std::io::prelude::*;
use std::path;
use toml;
use error;

#[derive(Deserialize, Serialize, Default, PartialEq, Debug)]
pub struct Config {
    version: f64,
    tools: Vec<Tool>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Tool {
    command: String,
    extensions: Vec<String>,
}

impl Config {
    pub fn load(conf_name: &str) -> Result<Config, error::AppError> {
        let path = path::Path::new(conf_name);
        if !path.exists() {
            return Result::Err(error::AppError::Io(io::Error::new(
                io::ErrorKind::NotFound,
                format!("{} file is not exist", path.to_str().unwrap_or("")),
            )));
        }

        let mut reader = io::BufReader::new(fs::File::open(path).map_err(error::AppError::Io)?);
        let mut s = String::new();
        reader.read_to_string(&mut s).map_err(error::AppError::Io)?;

        let conf: Config = toml::from_str(s.as_str()).map_err(error::AppError::TomlDe)?;

        Ok(conf)
    }

    pub fn save(&self, conf_name: &str) -> Result<(), error::AppError> {
        let path = path::Path::new(conf_name);
        let parent_dir = path.parent();
        match parent_dir {
            Some(d) if d == path::Path::new("") => {}
            Some(d) if !d.exists() => fs::create_dir_all(d).map_err(error::AppError::Io)?,
            Some(_) | None => {}
        }

        let toml = toml::to_string(&self).map_err(error::AppError::TomlSer)?;
        let mut writer = io::BufWriter::new(fs::File::create(path).map_err(error::AppError::Io)?);
        writer.write(toml.as_bytes()).map_err(error::AppError::Io)?;

        Ok(())
    }

    pub fn add(&mut self, command: &str, extension: &str) -> Result<(), error::AppError> {
        let ext = extension.to_owned();

        for t in self.tools.iter_mut() {
            if t.extensions.contains(&ext) {
                println!(".{} is already exists", extension);
                return Ok(()); // FIXME
            }
            if t.command == command {
                t.extensions.push(ext);
                return Ok(());
            }
        }

        self.tools.push(Tool {
            command: command.to_owned(),
            extensions: vec![ext],
        });
        Ok(())
    }

    pub fn remove_command(&mut self, command: &str) -> Result<(), error::AppError> {
        let mut remove_i = None;
        for (i, t) in self.tools.iter_mut().enumerate() {
            if t.command == command {
                remove_i = Some(i);
                break;
            }
        }
        match remove_i {
            Some(i) => {
                self.tools.remove(i);
                Ok(())
            }
            None => {
                Result::Err(error::AppError::Io(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("{} command is not registered", command),
                )))
            }
        }
    }

    pub fn remove_extension(&mut self, extension: &str) -> Result<(), error::AppError> {
        let mut tools_i = None;
        let mut ext_i = None;
        for (ia, t) in self.tools.iter().enumerate() {
            if t.extensions.contains(&extension.to_string()) {
                tools_i = Some(ia);
                for (ib, e) in t.extensions.iter().enumerate() {
                    if e == extension {
                        ext_i = Some(ib);
                        break;
                    }
                }
            }
        }
        match ext_i {
            Some(ext_i) => {
                let tools_i = tools_i.unwrap();
                self.tools[tools_i].extensions.remove(ext_i);
                if self.tools[tools_i].extensions.is_empty() {
                    self.tools.remove(tools_i);
                }
                Ok(())
            }
            None => {
                Result::Err(error::AppError::Io(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!(".{} extension is not registered", extension),
                )))
            }
        }
    }

    pub fn get_command(&self, extension: &str) -> Option<String> {
        let ext_string = extension.to_string();
        for t in self.tools.iter() {
            if t.extensions.contains(&ext_string) {
                return Some(t.command.clone());
            }
        }
        None
    }
}


#[test]
fn test_decode_toml() {
    let toml_config_str = r#"
    version = 0.0

    [[tools]]
    command = "apvlv"
    extensions = [ "pdf" ]

    [[tools]]
    command = "mirage"
    extensions = [ "jpg", "png", "gif" ]

    [[tools]]
    command = "vlc"
    extensions = [ "mp4", "mov", "avi" ]

    [[tools]]
    command = "vim -R"
    extensions = [ "conf" ]
    "#;

    let config: Config = toml::from_str(toml_config_str).unwrap();
    assert_eq!(config.version, 0.0);

    assert_eq!(config.tools[0].command, "apvlv".to_string());
    assert_eq!(config.tools[0].extensions[0], "pdf".to_string());

    assert_eq!(config.tools[1].command, "mirage".to_string());
    assert_eq!(config.tools[1].extensions[0], "jpg".to_string());
    assert_eq!(config.tools[1].extensions[1], "png".to_string());
    assert_eq!(config.tools[1].extensions[2], "gif".to_string());

    assert_eq!(config.tools[2].command, "vlc".to_string());
    assert_eq!(config.tools[2].extensions[0], "mp4".to_string());
    assert_eq!(config.tools[2].extensions[1], "mov".to_string());
    assert_eq!(config.tools[2].extensions[2], "avi".to_string());

    assert_eq!(config.tools[3].command, "vim -R".to_string());
    assert_eq!(config.tools[3].extensions[0], "conf".to_string());
}

#[test]
fn test_new() {
    let conf1 = Config::default();
    let conf2 = Config::default();
    assert_eq!(conf1, conf2);
}

#[test]
fn test_save() {
    let conf = Config {
        version: 0.0,
        tools: vec![
            Tool {
                command: "cat".to_string(),
                extensions: vec!["txt".to_string(), "log".to_string()],
            },
        ],
    };

    assert_eq!(conf.save("test_config").unwrap(), ());
    let loaded_conf = Config::load("test_config").unwrap();
    assert_eq!(conf, loaded_conf);
    assert_eq!(conf.save("./test_config").unwrap(), ());

    let loaded_conf = Config::load("./test_config").unwrap();
    assert_eq!(conf, loaded_conf);

    let _ = fs::remove_file("test_config");
}

#[test]
fn test_add() {
    let mut conf = Config {
        version: 0.0,
        tools: vec![
            Tool {
                command: "cat".to_string(),
                extensions: vec!["txt".to_string(), "log".to_string()],
            },
        ],
    };
    let conf_added_extension = Config {
        version: 0.0,
        tools: vec![
            Tool {
                command: "cat".to_string(),
                extensions: vec!["txt".to_string(), "log".to_string(), "new_ext".to_string()],
            },
        ],
    };

    // add same command
    assert_ne!(conf, conf_added_extension);
    conf.add("cat", "new_ext").unwrap();
    assert_eq!(conf, conf_added_extension);

    let conf_added_command = Config {
        version: 0.0,
        tools: vec![
            Tool {
                command: "cat".to_string(),
                extensions: vec!["txt".to_string(), "log".to_string(), "new_ext".to_string()],
            },
            Tool {
                command: "new_cmd".to_string(),
                extensions: vec!["xxx".to_string()],
            },
        ],
    };

    // add new command and extension
    assert_ne!(conf, conf_added_command);
    conf.add("new_cmd", "xxx").unwrap();
    assert_eq!(conf, conf_added_command);

    // add same command and extension
    conf.add("new_cmd", "xxx").unwrap();
    assert_eq!(conf, conf_added_command);
}

#[test]
fn test_remove_command() {
    let mut conf = Config {
        version: 0.0,
        tools: vec![
            Tool {
                command: "cat".to_string(),
                extensions: vec!["txt".to_string(), "log".to_string()],
            },
            Tool {
                command: "objdump".to_string(),
                extensions: vec!["o".to_string()],
            },
        ],
    };

    // remove_command
    assert!(conf.remove_command("cat").is_ok());
    let conf_expected = Config {
        version: 0.0,
        tools: vec![
            Tool {
                command: "objdump".to_string(),
                extensions: vec!["o".to_string()],
            },
        ],
    };
    assert_eq!(conf, conf_expected);

    // remove_command
    assert!(conf.remove_command("objdump").is_ok());
    let conf_expected = Config {
        version: 0.0,
        tools: Vec::new(),
    };
    assert_eq!(conf, conf_expected);

    // error occured
    assert!(conf.remove_command("objdump").is_err());
}

#[test]
fn test_remove_extension() {
    let mut conf = Config {
        version: 0.0,
        tools: vec![
            Tool {
                command: "cat".to_string(),
                extensions: vec!["txt".to_string(), "log".to_string()],
            },
            Tool {
                command: "objdump".to_string(),
                extensions: vec!["o".to_string()],
            },
        ],
    };

    // remove_command
    assert!(conf.remove_extension("txt").is_ok());
    let conf_expected = Config {
        version: 0.0,
        tools: vec![
            Tool {
                command: "cat".to_string(),
                extensions: vec!["log".to_string()],
            },
            Tool {
                command: "objdump".to_string(),
                extensions: vec!["o".to_string()],
            },
        ],
    };
    assert_eq!(conf, conf_expected);

    assert!(conf.remove_extension("o").is_ok());
    let conf_expected = Config {
        version: 0.0,
        tools: vec![
            Tool {
                command: "cat".to_string(),
                extensions: vec!["log".to_string()],
            },
        ],
    };
    assert_eq!(conf, conf_expected);

    // error occured
    assert!(conf.remove_extension("o").is_err());
}

#[test]
fn test_get_command() {
    let conf = Config {
        version: 0.0,
        tools: vec![
            Tool {
                command: "cat".to_string(),
                extensions: vec!["txt".to_string(), "log".to_string()],
            },
        ],
    };

    assert_eq!(conf.get_command("txt").unwrap(), "cat");
    assert_eq!(conf.get_command("log").unwrap(), "cat");
    assert_eq!(
        conf.get_command("jpg").unwrap_or("none".to_string()),
        "none"
    );
}
