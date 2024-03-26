pub mod basic_types;

use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct Column {
    name: String,
    data_type: basic_types::BasicType,
    nullable: bool,
}

impl Column {
    pub fn new(name: String, data_type: basic_types::BasicType, nullable: bool) -> Column {
        Column {
            name,
            data_type,
            nullable,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Table {
    name: String,
    columns: Vec<Column>,
    primary_key: u32,
}

impl Table {
    pub fn new(name: String, columns: Vec<Column>, primary_key: u32) -> Table {
        Table {
            name,
            columns,
            primary_key,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Catalogue {
    tables: Vec<Table>,
}

impl Catalogue {
    pub fn new(tables: Vec<Table>) -> Catalogue {
        Catalogue { tables }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let mut file = std::fs::File::create("catalogue.json")?;
        let json = serde_json::to_string(self)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn load() -> Result<Catalogue, std::io::Error> {
        let file = std::fs::File::open("catalogue.json")?;
        let reader = std::io::BufReader::new(file);
        let catalogue = serde_json::from_reader(reader)?;
        Ok(catalogue)
    }
}