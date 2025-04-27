use clap::Parser;
use rcli::{process_csv, process_decode, process_encode};
use rcli::{process_gen_pass, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(csv_opts) => {
            let output = if let Some(output) = csv_opts.output {
                output
            } else {
                format!("output.{}", csv_opts.format)
            };
            process_csv(csv_opts.input, output, csv_opts.format)?;
        }
        SubCommand::GenPass(csv_opts) => {
            process_gen_pass(&csv_opts)?;
        }
        SubCommand::Base64(base64_opts) => match base64_opts {
            rcli::Base64SubCommand::Encode(opts) => {
                println!("{:?}", opts);
                process_encode(opts.input, opts.format)?;
            }
            rcli::Base64SubCommand::Decode(opts) => {
                process_decode(opts.input, opts.format)?;
            }
        },
    }
    Ok(())
}
