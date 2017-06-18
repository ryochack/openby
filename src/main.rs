#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate getopts;
use std::env;
use std::process;
use getopts::Options;
use getopts::ParsingStyle;

const DEFAULT_CONF_PATH: &str = "~/.config/openby/config";

#[allow(dead_code)]
#[derive(Deserialize)]
struct Config {
    version: toml::Value,
    tools: Vec<Tools>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Tools {
    command: toml::Value,
    extentions: toml::value::Array,
}

fn main() {
    let exit_code = match run() {
        Ok(_) => 0,
        Err(e) => e,
    };
    process::exit(exit_code);
}

fn run() -> Result<i32, i32> {
    let args: Vec<String> = env::args().collect();
    let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let (file_name, conf_name) = parse_options(args_str)?;
    println!("Ok: {} {}", file_name, conf_name);
    Ok(0)
}

fn parse_options(args: Vec<&str>) -> Result<(String, String), i32> {
    let ref program = &args[0];

    let mut opts = Options::new();
    opts.parsing_style(ParsingStyle::FloatingFrees);
    opts.optopt("c", "config", "specify configure file name", "CONFIG");
    opts.optflag("h", "help", "print this help menu");

    let matches = opts.parse(&args[1..]).unwrap();

    if matches.opt_present("h") {
        print_usage(&program, &opts);
        return Err(0);
    }

    let file_name = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, &opts);
        return Err(1);
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
    print!("{}", opts.usage(&brief));
}

#[test]
fn test_parse_options() {
    let no_arguments = vec!["openby", "input.file", "-c", "configure.file"];

    assert_eq!(Err(1), parse_options(no_arguments[0..1]));
    assert_eq!(Ok(("input.file".to_string(), DEFAULT_CONF_PATH.to_string())),
               parse_options(no_arguments[0..2]));
    assert_eq!(Ok(("input.file".to_string(), "configure.file".to_string())),
               parse_options(no_arguments));
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
    assert_eq!(config.version, toml::Value::Float(0.0));

    assert_eq!(config.tools[0].command,
               toml::Value::String("apvlv".to_string()));
    assert_eq!(config.tools[0].extentions[0],
               toml::Value::String("pdf".to_string()));

    assert_eq!(config.tools[1].command,
               toml::Value::String("mirage".to_string()));
    assert_eq!(config.tools[1].extentions[0],
               toml::Value::String("jpg".to_string()));
    assert_eq!(config.tools[1].extentions[1],
               toml::Value::String("png".to_string()));
    assert_eq!(config.tools[1].extentions[2],
               toml::Value::String("gif".to_string()));

    assert_eq!(config.tools[2].command,
               toml::Value::String("vlc".to_string()));
    assert_eq!(config.tools[2].extentions[0],
               toml::Value::String("mp4".to_string()));
    assert_eq!(config.tools[2].extentions[1],
               toml::Value::String("mov".to_string()));
    assert_eq!(config.tools[2].extentions[2],
               toml::Value::String("avi".to_string()));

    assert_eq!(config.tools[3].command,
               toml::Value::String("vim -R".to_string()));
    assert_eq!(config.tools[3].extentions[0],
               toml::Value::String("conf".to_string()));
}
