use clap::Parser;

use super::{parse_format, verify_input_file};

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
