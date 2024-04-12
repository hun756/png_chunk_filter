mod cli;
mod png;
mod chunk;

fn main() -> std::io::Result<()> {
    let config = cli::parse_args();
    png::process_file(&config.input, &config.output)
}