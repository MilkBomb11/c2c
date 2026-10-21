use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::{
    display_wrapper::ToDisplayWrapper, error::CliError, parse::Parser, print::Printer,
    token::Token, tokenize,
};

#[derive(Default, Debug)]
pub struct Flags {
    pub is_stdout: bool,
    pub is_help: bool,
    pub is_debug: bool,
    pub extension: Option<String>,
}

fn parse_cli() -> (Flags, Vec<String>) {
    let args: Vec<String> = env::args().collect();
    let mut flags: Flags = Flags::default();
    let mut paths: Vec<String> = Vec::new();
    if args.len() == 1 {
        flags.is_help = true;
        return (flags, paths);
    }
    let mut it = args.into_iter().skip(1);
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "-s" | "--stdout" => flags.is_stdout = true,
            "-h" | "--help" => flags.is_help = true,
            "--debug" => flags.is_debug = true,
            "-x" | "--extenstion" => {
                let extension = match it.next() {
                    Some(s) => s,
                    None => panic!("Expected extension to be provided for {}.", arg.as_str()),
                };
                flags.extension = Some(extension);
            }
            _ => paths.push(arg),
        }
    }
    (flags, paths)
}

fn print_usage() {
    let usage = [
        "Usage: <path/to/c2c> [-h|--help] [-s|--stdout] [--debug] <arg1> <arg2> ...",
        "    -s | --stdout    : print translation results to stdout instead of writing to file.",
        "    -x | --extension : define fallback extension. default is c.",
        "         --debug     : for debugging purposes.",
        "    -h | --help      : print this message.",
        "Args: ",
        "    *.c2c     : will be translated to *.c.",
        "    *.c2h     : will be translated to *.h.",
        "    otherwise : will be translated to the fallback extension or *.c if not specified. ",
    ];

    for line in usage {
        println!("{}", line)
    }
}

fn create_printer<'a>(
    flags: &Flags,
    text: &'a str,
    file_path: &Path,
) -> Result<Printer<'a>, CliError> {
    let tokens: Vec<Token> = tokenize::get_tokens(text);
    if flags.is_debug {
        for token in tokens.iter() {
            println!("{}", token.with_text(text));
        }
    }

    let mut parser: Parser = Parser::new(tokens);
    let (replace_dests, replace_srcs) = match parser.parse(text) {
        Ok(repos) => repos,
        Err(e) => {
            return Err(CliError::new(format!(
                "{}: {}",
                file_path.display(),
                e.with_text(text)
            )));
        }
    };
    if flags.is_debug {
        println!("{}", replace_dests.with_text(text));
        println!("{}", replace_srcs);
    }

    let printer: Printer = Printer::new(replace_dests, replace_srcs, text);
    Ok(printer)
}

pub fn driver() -> Result<(), CliError> {
    let (flags, paths) = parse_cli();
    if flags.is_help {
        print_usage();
        return Ok(());
    }
    for path in paths.into_iter() {
        let path = Path::new(&path);
        let content = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => {
                return Err(CliError::new(format!(
                    "Failed to load file {}.",
                    path.display()
                )));
            }
        };
        let printer = create_printer(&flags, content.as_str(), &path)?;
        if flags.is_stdout {
            println!("{}", printer);
            continue;
        }

        let out_file_path = {
            let tmp_c = String::from("c");
            let fall_back_extension = flags.extension.as_ref().unwrap_or(&tmp_c);
            match path.extension() {
                None => path.with_added_extension(fall_back_extension),
                Some(os_str) => match os_str.to_str() {
                    Some("c2c") => path.with_extension("c"),
                    Some("c2h") => path.with_extension("h"),
                    Some(_) => path.with_added_extension(fall_back_extension),
                    None => {
                        return Err(CliError::new(format!(
                            "Extension of file {} is not valid utf-8.",
                            os_str.display()
                        )));
                    }
                },
            }
        };
        let mut out_file = match File::options()
            .append(false)
            .write(true)
            .create(true)
            .open(out_file_path)
        {
            Ok(f) => f,
            Err(e) => return Err(CliError::new(format!("{}", e))),
        };
        let _ = write!(out_file, "{}", printer);
    }
    Ok(())
}
