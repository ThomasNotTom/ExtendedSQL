use crate::table::Table;
use std::collections::BTreeMap;

use std::{cell::RefCell, rc::Rc};

pub struct Database {
    pub tables: BTreeMap<String, Rc<RefCell<Table>>>,
}

impl Default for Database {
    fn default() -> Self {
        Database {
            tables: BTreeMap::<String, Rc<RefCell<Table>>>::new(),
        }
    }
}

impl ToString for Database {
    fn to_string(&self) -> String {
        let mut out = String::from("");

        for table in &self.tables {
            out.push_str(table.1.borrow_mut().to_string().as_str());
        }

        return out;
    }
}

impl Database {
    pub fn new(tables: BTreeMap<String, Rc<RefCell<Table>>>) -> Self {
        Database { tables }
    }
    pub fn add_table(&mut self, name: String, table: Rc<RefCell<Table>>) {
        self.tables.insert(name, table);
    }
}
