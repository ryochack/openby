#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate getopts;
use std::io;
use std::result;
use std::env;
use getopts::Options;
use getopts::ParsingStyle;

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

pub type Result<T> = result::Result<T, io::Error>;

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options] FILE", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let exit_code = run();
    std::process::exit(exit_code);
}

fn run() -> i32 {
    let args: Vec<String> = env::args().collect();
    let ref program = &args[0];

    let mut opts = Options::new();
    opts.parsing_style(ParsingStyle::FloatingFrees);
    opts.optopt("c", "config", "specify configure file name", "CONFIG");
    opts.optflag("h", "help", "print this help menu");

    let matches = opts.parse(&args[1..]).unwrap();

    if matches.opt_present("h") {
        print_usage(&program, &opts);
        return 0;
    }

    let filename = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, &opts);
        return 1;
    };

    println!("{:?}", matches.free);
    0
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

    assert_eq!(config.tools[0].command, toml::Value::String("apvlv".to_string()));
    assert_eq!(config.tools[0].extentions[0], toml::Value::String("pdf".to_string()));

    assert_eq!(config.tools[1].command, toml::Value::String("mirage".to_string()));
    assert_eq!(config.tools[1].extentions[0], toml::Value::String("jpg".to_string()));
    assert_eq!(config.tools[1].extentions[1], toml::Value::String("png".to_string()));
    assert_eq!(config.tools[1].extentions[2], toml::Value::String("gif".to_string()));

    assert_eq!(config.tools[2].command, toml::Value::String("vlc".to_string()));
    assert_eq!(config.tools[2].extentions[0], toml::Value::String("mp4".to_string()));
    assert_eq!(config.tools[2].extentions[1], toml::Value::String("mov".to_string()));
    assert_eq!(config.tools[2].extentions[2], toml::Value::String("avi".to_string()));

    assert_eq!(config.tools[3].command, toml::Value::String("vim -R".to_string()));
    assert_eq!(config.tools[3].extentions[0], toml::Value::String("conf".to_string()));
}
