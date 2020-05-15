extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::iterators::Pair;



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
                parse_include(line);
            }
            Rule::static_data_block => {
                parse_static_data(line);
            }
            _ => {}
        }
    }
}

fn parse_static_data(static_data_block :Pair<'_, Rule>) {
    for static_data_declaration in static_data_block.into_inner() {
        let mut inner = static_data_declaration.into_inner();
        let label = inner.next().unwrap().as_str();
        let data = inner.next().unwrap().into_inner().next().unwrap();
        match data.as_rule() {
            Rule::static_string_data => {
                let mut inner = data.into_inner();
                let string = inner.next().unwrap().as_str();
                println!("we got string data {} with labelname {:?}", string, label);
            }
            _ => {println!("{:?}", data);}
        }
    }
}

fn parse_include(include_block :Pair<'_, Rule>) {
    for inner in include_block.into_inner() {
        match inner.as_rule() {
            Rule::include => {
                let mut inner_rules = inner.into_inner(); // path
                let path = inner_rules.next().unwrap().as_str();
                println!("shall include file: {}", path);
            }
            x => {
                println!("Error, {:?}", x);
            }
        }
    }

}

#[derive(Parser)]
#[grammar = "lang.pest"]
pub struct LangParser;
