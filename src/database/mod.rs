use crate::table::Table;
use std::cell::RefCell;
use std::collections::BTreeMap;

use std::rc::Rc;

pub struct Database {
    pub name: String,
    pub tables: BTreeMap<String, RefCell<Table>>,
}

impl Default for Database {
    fn default() -> Self {
        Database {
            name: "Default".to_string(),
            tables: BTreeMap::<String, RefCell<Table>>::new(),
        }
    }
}

impl ToString for Database {
    fn to_string(&self) -> String {
        let mut out = String::from("");
        out.push_str(" -- ");
        out.push_str(&self.name);
        out.push_str(" -- ");
        for table in &self.tables {
            out.push_str(table.1.borrow().to_string().as_str());
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
