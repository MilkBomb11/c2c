use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::{
    display_wrapper::ToDisplayWrapper, parse::Parser, print::Printer, token::Token, tokenize,
};

#[derive(Default, Debug)]
pub struct Flags {
    pub is_stdout: bool,
    pub is_help: bool,
    pub is_debug: bool,
}

fn parse_cli() -> (Flags, Vec<String>) {
    let args: Vec<String> = env::args().collect();
    let mut flags: Flags = Flags::default();
    let mut paths: Vec<String> = Vec::new();
    for arg in args.into_iter().skip(1) {
        match arg.as_str() {
            "-s" | "--stdout" => flags.is_stdout = true,
            "-h" | "--help" => flags.is_help = true,
            "--debug" => flags.is_debug = true,
            _ => paths.push(arg),
        }
    }
    (flags, paths)
}

fn print_usage() {
    println!("Usage: <path/to/c2c> [-h|--help] [-s|--stdout] [--debug] <arg1> <arg2> ...");
    println!("Flags:");
    println!("    -s | --stdout : print translation results to stdout instead of writing to file.");
    println!("         --debug  : for debugging purposes.");
    println!("    -h | --help   : print this message.");
    println!("Args:");
    println!("    .c2c : will be translated to .c.");
    println!("    .c2h : will be translated to .h.");
}

fn create_printer<'a>(flags: &Flags, text: &'a str) -> Printer<'a> {
    let tokens: Vec<Token> = tokenize::get_tokens(text);
    if flags.is_debug {
        for token in tokens.iter() {
            println!("{}", token.with_text(text));
        }
    }

    let mut parser: Parser = Parser::new(tokens);
    let (replace_dests, replace_srcs) = match parser.parse(text) {
        Ok(repos) => repos,
        Err(e) => panic!("{}", e.with_text(text)),
    };
    if flags.is_debug {
        println!("{}", replace_dests.with_text(text));
        println!("{}", replace_srcs);
    }

    let printer: Printer = Printer::new(replace_dests, replace_srcs, text);
    printer
}

pub fn driver() {
    let (flags, paths) = parse_cli();
    if flags.is_help {
        print_usage();
        return;
    }
    for path in paths.into_iter() {
        let path = Path::new(&path);
        let content = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => {
                println!("Failed to load file.");
                return;
            }
        };
        let printer = create_printer(&flags, content.as_str());
        if flags.is_stdout {
            println!("{}", printer);
            continue;
        }
        let out_file_path = match path.extension() {
            None => path.with_added_extension("c"),
            Some(os_str) => match os_str.to_str() {
                Some("c2c") => path.with_extension("c"),
                Some("c2h") => path.with_extension("h"),
                Some(_) => path.with_added_extension("c"),
                None => panic!("Extension of file {} is not valid utf-8.", os_str.display()),
            },
        };
        let mut out_file = match File::options()
            .append(false)
            .write(true)
            .create(true)
            .open(out_file_path)
        {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };
        let _ = write!(out_file, "{}", printer);
    }
}
