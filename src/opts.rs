use clap::Parser;
use std::path::Path;

#[derive(Debug, Parser)]
#[clap(name = "rcli", author, version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}
#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or Convert CSV to JSON")]
    Csv(CsvOpts),
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    // #[arg(short, long, default_value = "output.json")]
    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long,value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,

    #[arg(long, default_value_t = true)]
    pub header: bool,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
}

pub fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(&filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

pub fn parse_format(format: &str) -> Result<OutputFormat, &'static str> {
    match format {
        "json" => Ok(OutputFormat::Json),
        "yaml" => Ok(OutputFormat::Yaml),
        // "toml" => Ok(OutputFormat::Toml),
        _ => Err("Invalid format"),
    }
}
