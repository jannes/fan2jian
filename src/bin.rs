use std::{fs, io::Write, path::PathBuf};

use fan2jian::map_text;

const HELP: &str = "\
fan2jian
USAGE:
  app [INPUT_PATH] [OUTPUT_PATH]
FLAGS:
  -h, --help            Prints help information
  -r, --reverse         Do reverse direction: 简体 to 繁体
ARGS:
  <INPUT_PATH>: path to input file
  <OUTPUT_PATH>: path to output file
";


#[derive(Debug)]
struct AppArgs {
    input: PathBuf,
    output: PathBuf,
    reverse: bool,
}

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }
    let args = AppArgs {
        input: pargs.free_from_str()?,
        output: pargs.free_from_str()?,
        reverse: pargs.contains(["-r", "--reverse"]),
    };
    Ok(args)
}


fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };
    let input_text = fs::read_to_string(args.input).expect("could not read input file");
    let output_text = map_text(&input_text, !args.reverse);
    let mut output_file =
        fs::File::create(args.output.as_path()).expect("could not open output file");
    output_file
        .write_all(output_text.as_bytes())
        .expect("could not write to output file");
}
