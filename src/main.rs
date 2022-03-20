mod engine;
mod tx;

use anyhow::Result;
use clap::Parser;
use engine::Engine;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use tx::Tx;

fn stream_records_from_reader<'r>(
    reader: &'r mut dyn Read,
) -> impl Iterator<Item = Result<Tx>> + 'r {
    csv::Reader::from_reader(reader)
        .into_deserialize()
        .map(|r| r.map_err(Into::into))
}

fn write_cols(writer: &mut csv::Writer<&mut dyn Write>) -> Result<()> {
    writer.write_record(&[
        "client",
        "available",
        "held",
        "total",
        "locked",
    ])?;

    Ok(())
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
    let mut stdout = io::stdout();
    let mut writer = csv::Writer::from_writer(&mut stdout as &mut dyn Write);
    let mut engine = Engine::new();

    for tx in tx_stream {
        if let Err(err) = engine.apply(tx?) {
            eprintln!("{}", err);
        }
    }

    write_cols(&mut writer)?;
    for account in engine.accounts() {
        writer.serialize(account)?;
    }

    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::stream_records_from_reader;
    use super::Engine;
    use std::fs::File;
    use std::io::Cursor;
    use std::io::Write;
    use super::write_cols;

    #[test]
    fn verify_processing() {
        let mut file = File::open("sample.csv").unwrap();
        let tx_stream = stream_records_from_reader(&mut file);
        let mut data = Vec::new();
        let mut wtr = Cursor::new(&mut data);
        let mut writer = csv::Writer::from_writer(&mut wtr as &mut dyn Write);
        let mut engine = Engine::new();

        for tx in tx_stream {
            engine.apply(tx.unwrap()).unwrap();
        }

        write_cols(&mut writer).unwrap();
        for account in engine.accounts() {
            writer.serialize(account).unwrap();
        }

        writer.flush().unwrap();
        drop(writer);
        
        let readable = String::from_utf8(data).unwrap();
        insta::assert_snapshot!(readable);
    }
}
