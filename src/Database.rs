use crate::table::table;
use std::collections::BTreeMap;

use std::{cell::RefCell, rc};

pub struct Database {
    pub tables: BTreeMap<String, rc::Rc<RefCell<table::Table>>>,
}

impl Default for Database {
    fn default() -> Self {
        Database {
            tables: BTreeMap::<String, rc::Rc<RefCell<table::Table>>>::new(),
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
    pub fn new(tables: BTreeMap<String, rc::Rc<RefCell<table::Table>>>) -> Self {
        Database { tables: tables }
    }
    pub fn add_table(&mut self, name: String, table: rc::Rc<RefCell<table::Table>>) {
        self.tables.insert(name, table);
    }
}
