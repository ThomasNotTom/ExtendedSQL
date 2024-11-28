use crate::database::Database;

use std::cell::RefCell;
use std::collections::BTreeMap;

pub struct Context {
    pub databases: BTreeMap<String, RefCell<Database>>,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            databases: BTreeMap::<String, RefCell<Database>>::new(),
        }
    }
}

impl ToString for Context {
    fn to_string(&self) -> String {
        let mut str = String::from("");
        for database in &self.databases {
            str.push_str(database.1.borrow().to_string().as_str());
        }
        return str;
    }
}

impl Context {
    pub fn parse(_input: &str) {}

    pub fn new(database: BTreeMap<String, RefCell<Database>>) -> Self {
        Context {
            databases: database,
        }
    }
}
