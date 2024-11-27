use crate::Database;
use std::collections::BTreeMap;

pub struct Context {
    pub databases: BTreeMap<String, Box<Database::Database>>,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            databases: BTreeMap::<String, Box<Database::Database>>::new(),
        }
    }
}

impl Context {
    pub fn parse(_input: &str) {}

    pub fn new(database: BTreeMap<String, Box<Database::Database>>) -> Self {
        Context {
            databases: database,
        }
    }
}
