#[derive(Clone, Copy, Debug)]
pub enum Keywords {
    CREATE,
    TABLE,
    DATABASE,
}

pub fn string_to_keyword(string: &str) -> Option<Keywords> {
    match string {
        "CREATE" => Some(Keywords::CREATE),
        "TABLE" => Some(Keywords::TABLE),
        "DATABASE" => Some(Keywords::DATABASE),
        _ => None,
    }
}

pub fn keyword_to_string(string: Keywords) -> String {
    match string {
        Keywords::CREATE => "CREATE".to_string(),
        Keywords::TABLE => "TABLE".to_string(),
        Keywords::DATABASE => "DATABASE".to_string(),
    }
}

#[derive(Clone, Debug)]
pub enum PreParseData {
    Keyword(Keywords),
    UserText(String),
    NormalBracketText(String),
}

pub fn pre_parse(line: String) -> Vec<PreParseData> {
    let mut pre_parsed = vec![];

    let chars = line.chars().collect::<Vec<char>>();
    let mut buffer = String::new();

    let mut bracket_depth = 0;
    let mut inner_bracket_text = String::new();

    for char in chars {
        match char {
            ' ' => {
                if bracket_depth != 0 {
                    continue;
                }
                let matched_keyword = string_to_keyword(&buffer.clone());
                if matched_keyword.is_none() {
                    pre_parsed.push(PreParseData::UserText(buffer.clone()));
                } else {
                    pre_parsed.push(PreParseData::Keyword(matched_keyword.expect("")));
                }
                buffer = String::new();
                continue;
            }
            '(' => {
                bracket_depth += 1;
                continue;
            }
            ')' => {
                bracket_depth -= 1;
                if bracket_depth == 0 {
                    pre_parsed.push(PreParseData::NormalBracketText(inner_bracket_text.clone()));
                    inner_bracket_text.clear();
                }
                continue;
            }
            _ => {
                if bracket_depth == 0 {
                    buffer.push(char);
                } else {
                    inner_bracket_text.push(char);
                }
                continue;
            }
        }
    }

    if &buffer.clone() != "" {
        let matched_keyword = string_to_keyword(&buffer.clone());
        if matched_keyword.is_none() {
            pre_parsed.push(PreParseData::UserText(buffer.clone()));
        } else {
            pre_parsed.push(PreParseData::Keyword(matched_keyword.expect("")));
        }
    }

    pre_parsed
}
