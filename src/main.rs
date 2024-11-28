use extended_sql::context::Context;
use extended_sql::executor::execute_elements;
use extended_sql::parsing::parse_block;
use std::{
    env,
    io::{self, Read},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide a file name");
    }

    let file_name = &args[1];

    let mut file = std::fs::File::open(&file_name).expect("File does not exist");
    let mut buf = Vec::new();

    file.read_to_end(&mut buf)?;
    let _output = String::from_utf8_lossy(&buf);
    let _ctx = Context::parse(&_output);

    let parsed_context = parse_block(_output.to_string());

    let out_context = execute_elements(parsed_context);

    println!("{}", out_context.to_string());
    return Ok(());
}
