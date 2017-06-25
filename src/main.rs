#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate getopts;
use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::io::Write;
use std::path;
use std::process;
use getopts::Options;
use getopts::ParsingStyle;

const DEFAULT_CONF_PATH: &str = "~/.config/openby/config";

#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Toml(toml::de::Error),
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Config {
    version: f64,
    tools: Vec<Tools>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Tools {
    command: String,
    extentions: Vec<String>,
}

fn main() {
    let exit_code = match run() {
        Ok(_) => 0,
        Err(e) => {
            writeln!(&mut io::stderr(), "{:?}", e).unwrap();
            1
        }
    };
    process::exit(exit_code);
}

fn run() -> Result<(), AppError> {
    let args: Vec<String> = env::args().collect();
    let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let (file_name, conf_name) = parse_options(args_str)?;
    println!("Ok: {} {}", file_name, conf_name);

    let conf = load_config(&conf_name)?;

    open_by(&conf, &file_name)?;

    Ok(())
}

fn parse_options(args: Vec<&str>) -> Result<(String, String), AppError> {
    let ref program = &args[0];

    let mut opts = Options::new();
    opts.parsing_style(ParsingStyle::FloatingFrees);
    opts.optopt("c", "config", "specify configure file name", "CONFIG");
    opts.optflag("h", "help", "print this help menu");

    let matches = opts.parse(&args[1..]).unwrap();

    if matches.opt_present("h") {
        print_usage(&program, &opts);
        process::exit(0);
    }

    let file_name = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, &opts);
        return Result::Err(AppError::Io(
            io::Error::new(io::ErrorKind::NotFound, "FILE is not found"),
        ));
    };

    let conf_name = if let Some(c) = matches.opt_str("c") {
        c
    } else {
        DEFAULT_CONF_PATH.to_string()
    };

    Ok((file_name, conf_name))
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options] FILE", program);
    println!("{}", opts.usage(&brief));
}

fn load_config(conf_name: &str) -> Result<Config, AppError> {
    let path = path::Path::new(conf_name);
    if path.exists() == false {
        return Result::Err(AppError::Io(io::Error::new(
            io::ErrorKind::NotFound,
            format!("{} is not exist", path.to_str().unwrap_or("")),
        )));
    }

    let mut reader = io::BufReader::new(fs::File::open(path).map_err(AppError::Io)?);
    let mut s = String::new();
    reader.read_to_string(&mut s).map_err(AppError::Io)?;

    let conf: Config = toml::from_str(s.as_str()).map_err(AppError::Toml)?;

    Ok(conf)
}

fn open_by(conf: &Config, file_name: &str) -> Result<(), AppError> {
    let file_path = path::Path::new(file_name);
    if file_path.exists() == false {
        return Result::Err(AppError::Io(io::Error::new(
            io::ErrorKind::NotFound,
            format!("{} is not exist", file_path.to_str().unwrap_or("")),
        )));
    }

    let ext = file_path.extension().ok_or(AppError::Io(io::Error::new(
        io::ErrorKind::NotFound,
        format!(
            "{} has not extension",
            file_path.to_str().unwrap_or("")
        ),
    )))?;

    match get_commnad_from_extension(conf, ext.to_str().unwrap()) {
        Some(cmd) => {
            println!("file = {:?}", file_path);
            process::Command::new(cmd).arg(file_path).status().expect(
                "failed to run",
            );
        }
        None => {
            println!("command = None");
            // TODO: register command about this extension.
        }
    }

    Ok(())
}

fn get_commnad_from_extension(conf: &Config, extension: &str) -> Option<String> {
    let ext_string = extension.to_string();
    for t in conf.tools.iter() {
        if t.extentions.contains(&ext_string) {
            return Some(t.command.clone());
        }
    }
    None
}

#[test]
fn test_parse_options() {
    let no_arguments = vec!["openby", "input.file", "-c", "configure.file"];

    assert!(parse_options(no_arguments[0..1].to_vec()).is_err());
    assert_eq!(
        ("input.file".to_string(), DEFAULT_CONF_PATH.to_string()),
        parse_options(no_arguments[0..2].to_vec()).unwrap_or(("".to_string(), "".to_string()))
    );
    assert_eq!(
        ("input.file".to_string(), "configure.file".to_string()),
        parse_options(no_arguments).unwrap_or(("".to_string(), "".to_string()))
    );
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
