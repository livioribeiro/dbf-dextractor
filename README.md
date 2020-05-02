# dbf-dextractor

Extract data from dbf files and deserialize with serde.

## Usage

```rust
use std::collections::BTreeMap;

use serde::{Deserialize};

use dbf_dextractor::{Value, Date, Timestamp};

const DBF_FILE: &str = "/path/to/data.dbf";
const DBT_FILE: &str = "/path/to/data.dbt";

#[derive(Deserialize, Debug)]
struct Record {
    boolean: bool,
    date: Date,
    timestamp: Timestamp,
    decimal: f64,
    id: u32,
    name: String,
    note: Option<String>,
}

for record in dbf_dextractor::read(DBF_FILE, Some(DBT_FILE))? {
    let record: Record = record?;
    println!("{:#?}", record);
}

let records: Vec<BTreeMap<String, Value>>
    = dbf_dextractor::read_values(DBF_FILE, Some(DBT_FILE))?
        .collect::<Result<_, _>>()?;

println!("{:#?}", records);
```