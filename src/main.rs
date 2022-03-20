mod tx;

use tx::{Amount, Client, Tx, TxId, TxType};
use anyhow::Result;

pub fn stream_records_from_reader<'r>(
    reader: &'r mut dyn std::io::Read,
) -> impl Iterator<Item = Result<Tx>> + 'r {
    csv::Reader::from_reader(reader)
        .into_deserialize()
        .map(|r| r.map_err(Into::into))
}

fn main() {
    println!("Hello, world!");
}
