use std::cell::RefCell;
use std::collections::BTreeMap;

use crate::{
    context::Context,
    database::Database,
    parsing::{ContextElement, CreateDatabase},
};

pub fn execute_create_database(context: &mut Context, data: CreateDatabase) {
    let name = &data.database_name.to_string();
    context.databases.insert(
        name.to_string(),
        RefCell::new(Database::new(name.to_string(), BTreeMap::new())),
    );
}

pub fn execute_line(context: &mut Context, element: ContextElement) {
    match element {
        ContextElement::CreateDatabase(data) => execute_create_database(context, data),
    }
}

pub fn execute_elements(elements: Vec<ContextElement>) -> Context {
    let mut out_context = Context::default();

    for element in elements {
        execute_line(&mut out_context, element);
    }

    return out_context;
}
