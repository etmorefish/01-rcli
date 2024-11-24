use clap::Parser;
use rcli::{process_csv, process_genpass, Opts, SubCommand};

// rcli csv -i input.csv -o output.csv --header -d ','
// cargo run -- csv -i assets/juventus.csv --format yaml
// cargo run -- genpass

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
    }
    Ok(())
}
