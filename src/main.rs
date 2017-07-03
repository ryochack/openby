extern crate getopts;
extern crate openby;
use std::env;
use std::io;
use std::io::Write;
use std::path;
use std::process;
use getopts::Options;
use getopts::ParsingStyle;

use openby::error;
use openby::config;

const DEFAULT_CONF_PATH: &str = "~/.config/openby/config";

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

fn run() -> Result<(), error::AppError> {
    let args: Vec<String> = env::args().collect();
    let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let (file_name, conf_name) = parse_options(&args_str)?;
    println!("Ok: {} {}", file_name, conf_name);

    open_by(&conf_name, &file_name)?;

    Ok(())
}

fn parse_options(args: &[&str]) -> Result<(String, String), error::AppError> {
    let program = &(&args[0]);

    let mut opts = Options::new();
    opts.parsing_style(ParsingStyle::FloatingFrees);
    opts.optopt("c", "config", "specify configure file name", "CONFIG");
    opts.optflag("h", "help", "print this help menu");

    let matches = opts.parse(&args[1..]).unwrap();

    if matches.opt_present("h") {
        print_usage(program, &opts);
        process::exit(0);
    }

    let file_name = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(program, &opts);
        return Result::Err(error::AppError::Io(
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


fn open_by(conf_name: &str, file_name: &str) -> Result<(), error::AppError> {
    //let mut conf = config::Config::load(&conf_name)?;
    let mut conf = if path::Path::new(conf_name).exists() {
        config::Config::load(&conf_name)?
    } else {
        config::Config::new()
    };

    let file_path = path::Path::new(file_name);
    if !file_path.exists() {
        return Result::Err(error::AppError::Io(io::Error::new(
            io::ErrorKind::NotFound,
            format!("{} is not exist", file_path.to_str().unwrap_or("")),
        )));
    }

    let ext = file_path.extension().ok_or_else(|| {
        error::AppError::Io(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "{} has not extension",
                file_path.to_str().unwrap_or("")
            ),
        ))
    })?;

    let ext_str = ext.to_str().unwrap();
    let cmdline = conf.get_command(ext_str).unwrap_or_else(|| {
        println!("set the command associated with .{}", ext_str);
        print!("> ");
        let reader = io::stdin();
        let readline = input_command(reader.lock()).unwrap();
        let _ = conf.add(readline.as_str(), ext_str);
        println!("{:?}", conf);
        readline
    });
    let cmds: Vec<&str> = cmdline.split_whitespace().collect();
    let (cmd, option) = cmds.split_first().unwrap();
    println!("exec {:?} {:?} {:?}", cmd, option, file_path);
    process::Command::new(cmd)
        .args(option.into_iter())
        .arg(file_path)
        .spawn()
        .expect("failed to run");

    conf.save(conf_name)?;

    Ok(())
}


fn input_command<T: io::BufRead>(mut reader: T) -> Result<String, error::AppError> {
    let _ = std::io::stdout().flush();
    let mut input = String::new();
    let _ = reader.read_line(&mut input).map_err(error::AppError::Io)?;
    let concated: String = input.split_whitespace().fold(String::new(), |mut s, w| {
        if s.len() > 0 {
            s.push(' ');
        }
        s.push_str(&w);
        s
    });
    Ok(concated)
}


#[test]
fn test_parse_options() {
    let no_arguments = vec!["openby", "input.file", "-c", "configure.file"];

    assert!(parse_options(&no_arguments[0..1]).is_err());
    assert_eq!(
        ("input.file".to_string(), DEFAULT_CONF_PATH.to_string()),
        parse_options(&no_arguments[0..2]).unwrap_or(("".to_string(), "".to_string()))
    );
    assert_eq!(
        ("input.file".to_string(), "configure.file".to_string()),
        parse_options(&no_arguments).unwrap_or(("".to_string(), "".to_string()))
    );
}
