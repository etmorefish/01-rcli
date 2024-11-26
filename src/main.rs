use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, Base64SubCommand, Opts,
    SubCommand,
};

// rcli csv -i input.csv -o output.csv --header -d ','
// cargo run -- csv -i assets/juventus.csv --format yaml
// cargo run -- genpass
// cargo run -- base64 encode --input cargo.toml --format standard
// cargo run -- base64 decode --input assets/b64.txt --format urlsafe

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    println!("{:?}", opts);
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
            // println!("{}", opts)
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
                println!("{:?}", opts.format);
                process_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                println!("{:?}", opts.format);
                process_decode(&opts.input, opts.format)?;
            }
        },
    }
    Ok(())
}
