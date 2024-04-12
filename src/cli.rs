use clap::{Arg, Command};

pub struct Config {
    pub input: String,
    pub output: String,
}

pub fn parse_args() -> Config {
    let matches = Command::new("PNG Chunk Filter")
        .version("0.1")
        .author("Mehmet Ekemen. <ekemenms@gmail.com>")
        .about("Removes non-critical chunks from PNG files")
        .arg(
            Arg::new("input")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .help("Sets the output file")
                .required(true)
                .index(2),
        )
        .get_matches();

    Config {
        input: matches.get_one::<String>("input").unwrap().to_string(),
        output: matches.get_one::<String>("output").unwrap().to_string(),
    }
}
