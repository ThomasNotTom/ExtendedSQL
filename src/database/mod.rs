use crate::table::Table;
use std::collections::BTreeMap;

use std::rc::Rc;

pub struct Database {
    pub name: String,
    pub tables: BTreeMap<String, Rc<Table>>,
}

impl Default for Database {
    fn default() -> Self {
        Database {
            name: "Default".to_string(),
            tables: BTreeMap::<String, Rc<Table>>::new(),
        }
    }
}

impl ToString for Database {
    fn to_string(&self) -> String {
        let mut out = String::from("");
        for table in &self.tables {
            out.push_str(table.1.to_string().as_str());
        }

        return out;
    }
}

impl Database {
    pub fn new(name: String, tables: BTreeMap<String, Rc<Table>>) -> Self {
        Database { name, tables }
    }
    pub fn add_table(&mut self, name: String, table: Rc<Table>) {
        self.tables.insert(name, table);
    }
}
