use std::{
    borrow::Borrow,
    cell::{Ref, RefCell},
    collections::BTreeMap,
    ops::Deref,
    rc::Rc,
};

use crate::{
    context::Context,
    database::Database,
    parsing::{ContextElement, CreateDatabase, CreateTable},
    table::Table,
};

pub fn execute_create_database(context: &mut Context, data: CreateDatabase) {
    let name = &data.name;
    context.databases.insert(
        name.clone(),
        RefCell::new(Database::new(name.clone(), BTreeMap::new())),
    );
}

pub fn execute_create_table(context: &mut Context, data: CreateTable) {
    let table_name: String = data.table_name;
    let database_name: &String = &data.database_name;
    let database = context.databases.get_mut(database_name).expect("");
    database.add_table(table_name, RefCell::new(Table::default()));
}

pub fn execute_line(context: &mut Context, element: ContextElement) {
    match element {
        ContextElement::CreateDatabase(data) => execute_create_database(context, data),
        ContextElement::CreateTable(data) => execute_create_table(context, data),
    }
}

pub fn execute_elements(elements: Vec<ContextElement>) -> Context {
    let mut out_context = Context::default();

    for element in elements {
        execute_line(&mut out_context, element);
    }

    return out_context;
}
