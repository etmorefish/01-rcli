use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, process_http_serve,
    Base64SubCommand, HttpSubCommand, Opts, SubCommand,
};
use tracing::info;

/// rcli csv -i input.csv -o output.csv --header -d ','
/// cargo run -- csv -i assets/juventus.csv --format yaml
/// cargo run -- genpass
/// cargo run -- base64 encode --input cargo.toml --format standard
/// cargo run -- base64 decode --input assets/b64.txt --format urlsafe
/// cargo run -- http serve --dir assets --port 3000
/// $Env:RUST_LOG="info"

// fn main() -> anyhow::Result<()> {
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    info!("Starting rcli");

    let opts = Opts::parse();
    info!("{:?}", opts);
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{:?}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
            }
        },
        SubCommand::Http(cmd) => match cmd {
            HttpSubCommand::Serve(opts) => {
                process_http_serve(opts.dir, opts.port).await?;
            }
        },
    }
    Ok(())
}
