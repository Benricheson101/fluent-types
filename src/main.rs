mod parse;

use std::{error, fs};

use fluent_syntax::parser;

fn main() -> Result<(), Box<dyn error::Error>> {
    let file = fs::read_to_string("lang.ftl")?;

    let p = parser::parse(file).unwrap();
    let msgs = parse::parse_resource(p);

    println!("{}", msgs.to_typescript());

    Ok(())
}
