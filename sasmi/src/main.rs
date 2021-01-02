extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

mod sasm_parser;
use sasm_parser::LangParser;
use sasm_parser::Rule;

use std::fs;

fn main() {
    let unparsed_file = fs::read_to_string("./docs/example_we.sasm")
        .expect("cannot read file");

    let file = LangParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails

    
    for line in file.into_inner() {
        match line.as_rule() {
            Rule::include_block => {
                sasm_parser::parse_include(line);
            }
            Rule::static_data_block => {
                sasm_parser::parse_static_data(line);
            }
            Rule::dynamic_data_block => {
                sasm_parser::parse_dynamic_data(line);
            }
            Rule::code_block => {
                let code = sasm_parser::parse_code_data(line);

                println!("code is {:?}", code);
            }
            _ => {}
        }
    }
}
