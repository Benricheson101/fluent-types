mod cli;
mod parse;

use std::{error, fs, process};

use fluent_syntax::parser;

fn main() -> Result<(), Box<dyn error::Error>> {
    let app = cli::build_cli();
    let matches = app.get_matches();

    let mut src = vec![];
    if let Some(files) = matches.values_of("files") {
        for file in files {
            match fs::read_to_string(file) {
                Ok(content) => src.push(content),
                Err(err) => {
                    eprintln!("Failed to read file {}: {}", file, err);
                    process::exit(1);
                },
            }
        }
    }

    let source = src.join("\n");
    let parsed =
        parser::parse(source).expect("failed to parse Fluent resources");
    let msgs = parse::parse_resource(parsed);
    let ts = msgs.to_typescript();

    if let Some(out_file) = matches.value_of("output") {
        if let Err(err) = fs::write(out_file, ts) {
            eprintln!("failed to write to output file: {}", err);
        }
    } else {
        println!("{}", ts);
    }

    Ok(())
}
