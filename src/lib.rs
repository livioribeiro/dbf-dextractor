use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read, Seek};
use std::marker::PhantomData;
use std::path::Path;

use serde::de::DeserializeOwned;

mod dbf;
mod deserialize;
mod error;
mod model;
mod value;

use dbf::DbfReader;
use deserialize::DbfDeserializer;

pub use model::{Date, Timestamp};
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
    DbfReader::from_reader(table_file, memo_file).map(RecordIterator::new)
}

pub fn read_values<P>(
    table_path: P,
    memo_path: Option<P>,
) -> Result<ValuesIterator<File>, Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
{
    let table_file = File::open(table_path.as_ref())?;
    let memo_file = memo_path.map(File::open).transpose()?;
    DbfReader::from_reader(table_file, memo_file).map(|r| ValuesIterator { reader: r })
}

pub struct RecordIterator<R, T>
where
    R: Read + Seek,
    T: DeserializeOwned,
{
    reader: DbfReader<R>,
    deserializer: DbfDeserializer,
    _type: PhantomData<T>,
}

impl<R: Read + Seek, T: DeserializeOwned> RecordIterator<R, T> {
    pub fn new(reader: DbfReader<R>) -> Self {
        let deserializer = DbfDeserializer::new(reader.fields().clone());
        Self {
            reader,
            deserializer,
            _type: PhantomData,
        }
    }
}

impl<R: Read + Seek, T: DeserializeOwned> Iterator for RecordIterator<R, T> {
    type Item = Result<T, Box<dyn std::error::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        let values = match self.reader.next_record() {
            Ok(Some(val)) => val,
            Ok(None) => return None,
            Err(e) => return Some(Err(e)),
        };

        self.deserializer.set_values(values);
        T::deserialize(&mut self.deserializer)
            .map(Some)
            .map_err(|e| e.into())
            .transpose()
    }
}

pub struct ValuesIterator<R>
where
    R: Read + Seek,
{
    reader: DbfReader<R>,
}

impl<R: Read + Seek> Iterator for ValuesIterator<R> {
    type Item = Result<BTreeMap<String, Value>, Box<dyn std::error::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        let values = match self.reader.next_record() {
            Ok(Some(val)) => val,
            Ok(None) => return None,
            Err(e) => return Some(Err(e)),
        };

        let key_iter = self.reader.fields().iter().map(|f| f.name.clone());
        let val_iter = values.into_iter().map(From::from);

        Some(Ok(key_iter.zip(val_iter).collect()))
    }
}
