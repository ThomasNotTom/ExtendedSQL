use core::panic;

use crate::{
    header::Header,
    preparser::{self, keyword_to_string, Keywords, PreParseData},
};

#[derive(Debug)]
pub struct NamingData {
    name: String,
}

#[derive(Debug)]
pub struct MathsData {
    left_var: String,
    operator: String,
    right_var: String,
}

#[derive(Debug)]
pub struct CreateDatabase {
    pub name: String,
}

#[derive(Debug)]
pub struct CreateTable {
    pub database_name: String,
    pub table_name: String,
    pub columns: Vec<Header>,
}

#[derive(Debug)]
pub enum ContextElement {
    CreateDatabase(CreateDatabase),
    CreateTable(CreateTable),
}

pub fn parse_create_table(tokens: Vec<PreParseData>) -> ContextElement {
    let names_pre_parsed = &tokens[2];
    let names_string = match names_pre_parsed {
        PreParseData::Keyword(keyword) => keyword_to_string(*keyword),
        PreParseData::UserText(text) => text.to_string(),
    };
    let names = names_string.split(".").collect::<Vec<&str>>();

    let database_name = names[0].to_string();
    let table_name = names[1].to_string();

    let columns = vec![];

    return ContextElement::CreateTable(CreateTable {
        database_name: database_name,
        table_name: table_name,
        columns: columns,
    });
}

pub fn parse_create_database(tokens: Vec<PreParseData>) -> ContextElement {
    let name_token = &tokens[2];

    let name = match name_token {
        PreParseData::UserText(text) => text.clone(), // Clone the string for safe ownership
        PreParseData::Keyword(keyword) => keyword_to_string(keyword.clone()),
        _ => panic!(""),
    };

    return ContextElement::CreateDatabase(CreateDatabase {
        name: name.to_string(),
    });
}

pub fn parse_create(tokens: Vec<PreParseData>) -> ContextElement {
    match tokens[1] {
        PreParseData::Keyword(Keywords::DATABASE) => return parse_create_database(tokens),
        PreParseData::Keyword(Keywords::TABLE) => return parse_create_table(tokens),
        _ => panic!(),
    }
}

pub fn parse_line(text: String) -> ContextElement {
    let tokens = preparser::pre_parse(text);

    return match tokens[0] {
        PreParseData::Keyword(Keywords::CREATE) => parse_create(tokens),
        _ => panic!(""),
    };
}

pub fn parse_block(text: String) -> Vec<ContextElement> {
    return text
        .split(";")
        .map(|str| {
            let mut out = String::new();
            for char in str.split("") {
                if char != "\n" {
                    out.push_str(char);
                }
            }
            return out.to_string();
        })
        .filter(|str| {
            return str != "";
        })
        .map(|line| parse_line(line))
        .collect::<Vec<ContextElement>>();
}
