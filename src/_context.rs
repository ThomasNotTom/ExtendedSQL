use crate::_database::Database;

use std::collections::BTreeMap;

pub struct Context {
    pub databases: BTreeMap<String, Box<Database>>,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            databases: BTreeMap::<String, Box<Database>>::new(),
        }
    }
}

impl Context {
    pub fn parse(_input: &str) {}

    pub fn new(database: BTreeMap<String, Box<Database>>) -> Self {
        Context {
            databases: database,
        }
    }
}
