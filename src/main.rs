mod tx;
mod engine;

use tx::Tx;
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use std::fs::File;
use engine::Engine;
use std::io::{self, Read};

pub fn stream_records_from_reader<'r>(
    reader: &'r mut dyn Read,
) -> impl Iterator<Item = Result<Tx>> + 'r {
    csv::Reader::from_reader(reader)
        .into_deserialize()
        .map(|r| r.map_err(Into::into))
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    file: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut file = File::open(&args.file)?;
    let tx_stream = stream_records_from_reader(&mut file);
    let mut writer = csv::Writer::from_writer(io::stdout());
    let mut engine = Engine::new();

    for tx in tx_stream {
        if let Err(err) = engine.apply(tx?) {
            eprintln!("{}", err);
        }
    }

    for account in engine.accounts() {
        writer.serialize(account)?;
    }

    writer.flush()?;
    Ok(())
}
