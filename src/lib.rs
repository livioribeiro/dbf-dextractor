use std::fs::File;
use std::path::Path;

use serde::de::DeserializeOwned;

mod dbf;
mod deserialize;
mod error;
mod model;
mod value;

pub use dbf::{DbfReader, MemoReader, RecordIterator};
pub use model::Date;
pub use value::Value;

pub fn read<P, T>(
    table_path: P,
    memo_path: Option<P>,
) -> Result<RecordIterator<File, T>, Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let table_file = File::open(table_path.as_ref())?;
    let memo_file = memo_path.map(File::open).transpose()?;
    DbfReader::from_reader(table_file, memo_file).map(|r| r.records())
}
