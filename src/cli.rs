use clap::{Arg, Command};

pub fn build_cli() -> Command<'static> {
    Command::new("Fluent Types")
        .about(
            "Generate TypeScript type declarations for Fluent language files",
        )
        .arg(
            Arg::new("output")
                .help("the output file")
                .short('o')
                .takes_value(true)
                .default_value("-"),
        )
        .arg(
            Arg::new("files")
                .takes_value(true)
                .help("input fluent files")
                .multiple_occurrences(true)
                .required(true),
        )
}
