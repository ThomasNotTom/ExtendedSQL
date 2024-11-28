use crate::context::Context;

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
    pub database_name: String,
}

#[derive(Debug)]
pub enum ContextElement {
    CreateDatabase(CreateDatabase),
}

// pub struct ParsingContext {
//     pub database_context: Context,
//     pub context_stack: Vec<ContextElement>,
// }

pub fn parse_create_database(tokens: Vec<&str>) -> ContextElement {
    let name = tokens[0];

    return ContextElement::CreateDatabase(CreateDatabase {
        database_name: name.to_string(),
    });
}

pub fn parse_create(tokens: Vec<&str>) -> ContextElement {
    match (tokens[0]) {
        "DATABASE" => return parse_create_database(tokens[1..].to_vec()),
        _ => panic!(),
    }
}

pub fn parse_line(text: String) -> ContextElement {
    let tokens = text.split(" ").collect::<Vec<&str>>();

    match (tokens[0]) {
        "CREATE" => return parse_create(tokens[1..].to_vec()),
        _ => panic!(""),
    }
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
