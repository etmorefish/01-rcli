mod base64;
mod csv;
mod genpass;

use clap::Parser;

pub use self::{base64::*, csv::*, genpass::*};
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

    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),

    #[command(subcommand)]
    Base64(Base64SubCommand),
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

pub fn parse_base64_format(format: &str) -> Result<Base64Format, &'static str> {
    match format {
        "standard" => Ok(Base64Format::Standard),
        "urlsafe" => Ok(Base64Format::UrlSafe),
        _ => Err("Invalid format"),
    }
}
