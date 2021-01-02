
use pest::iterators::Pair;

mod sasm_syntax;

use sasm_syntax:: {
    Label,
    Code,
    CodeElement,
    Intstruction,
    Include,
    DynamicDataBlock
};

#[derive(Parser)]
#[grammar = "sasm_parser/lang.pest"]
pub struct LangParser;

fn parse_label(raw_label :Pair<'_, Rule>, base_name :&Label) -> Label {
    let label_name = raw_label.into_inner().next().unwrap()
        .as_str();

    let full_label_name = (vec![base_name.as_str().to_string(), label_name.to_string()]).join(".");

    return Label(full_label_name);
}

fn parse_code_run(code_run : Pair<'_, Rule>, base_name : &Label) -> Code {
    let mut code_content = vec![];
    
    for code_element in code_run.into_inner() {
        let data = code_element.into_inner().next().unwrap();

        let code_element = match data.as_rule() {
            Rule::label => {
                let full_label_name = parse_label(data, base_name);

                println!("got code label: {:?}", full_label_name);

                CodeElement::Label(full_label_name)
            }
            Rule::code_section => {
                let section = data;
                let mut inner = section.into_inner();

                let full_label_name = parse_label(inner.next().unwrap(), base_name);

                match inner.next() {
                    Some(x) => {
                        println!("start code section label: {:?}", full_label_name);
                        let code = parse_code_run(x, &full_label_name);
                        println!("end code section label: {:?}", full_label_name);
                        CodeElement::Section(code)
                    }
                    None => {
                        println!("empty code section label: {:?}", full_label_name);
                        
                        CodeElement::Section(Code::new(vec![]))
                    }
                }
            }
            Rule::instruction => {
                let ins = data;
                let mut inner = ins.into_inner();
                
                let head = inner.next().unwrap().as_str();

                let op_list: Vec<_> = inner
                    .map(|x| x.as_str())
                    .collect();

                println!("instr. {} with ops {:?}", head, op_list);

                CodeElement::Intstruction(Intstruction::NOP)
            }
            _ => { panic!("74") } 
        };

        code_content.push(code_element);
    }

    return Code::new(code_content);
}

pub fn parse_code_data(code_data :Pair<'_, Rule>) -> Code {
    let code_run = code_data.into_inner().next().unwrap();
    
    parse_code_run(code_run, &Label("global".to_string()))
}

pub fn parse_dynamic_data(dynamic_data_block :Pair<'_, Rule>) -> Option<DynamicDataBlock> {
    dynamic_data_block.into_inner()
        .map(|dynamic_data_declaration| {
            let mut inner = dynamic_data_declaration.into_inner();
            let label_option = inner.next();
            let data_option = inner.next();
            
            let def = match (label_option, data_option) {
                (None, _) => None,
                (_, None) => None,
                (Some(label), Some(data)) => {
                    let mut inner = data.into_inner();
                    let data_type_option = inner.next()
                        .and_then(|x| x.into_inner().next())
                        .map(|x| x.as_str());
                    let amount_option = inner.next();

                    None
                }
            };

            return def;
        })
        .collect::<Option<Vec<_>>>()
        .map(|x| DynamicDataBlock::new(x))
}

pub fn parse_static_data(static_data_block :Pair<'_, Rule>) {
    for static_data_declaration in static_data_block.into_inner() {
        let mut inner = static_data_declaration.into_inner();
        let label = inner.next().unwrap().as_str();
        let data = inner.next().unwrap().into_inner().next().unwrap();
        match data.as_rule() {
            Rule::static_string_data => {
                let mut inner = data.into_inner();
                let string = inner.next().unwrap().as_str();
                println!("static: {}\n\tgot str data {:?}", label, string);
            }
            Rule::static_num_data => {
                let mut inner = data.into_inner();
                let data_type = inner.next().unwrap();
                let bits = data_type.into_inner().next()
                    .unwrap().as_str().parse::<u32>().unwrap();
                let data = inner.next().unwrap().into_inner()
                    .map(|x| x.as_str().parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();
                println!("static: {}\n\tgot num data type with {:?} and {:?}", label, bits, data);
            }
            Rule::static_float_data => {
                let mut inner = data.into_inner();
                let data_type = inner.next().unwrap();
                let bits = data_type.into_inner().next()
                    .unwrap().as_str().parse::<u32>().unwrap();
                let data = inner.next().unwrap().into_inner()
                    .map(|x| x.as_str().parse::<f64>().unwrap())
                    .collect::<Vec<f64>>();
                println!("static: {}\n\tgot float data type with {:?} and {:?}", label, bits, data);
            }
            _ => {println!("{:?}", data);}
        }
    }
}

pub fn parse_include(include_block :Pair<'_, Rule>) -> Option<Vec<Include>> {
    //let mut includes = vec![];

    let x = include_block.into_inner()
        .map(|inner| {
            match inner.as_rule() {
                Rule::include => {
                    let mut inner_rules = inner.into_inner();
                    let option_path = inner_rules.next();
                    let option_name = inner_rules.next();

                    match (option_name, option_path) {
                        (_, None) => None,
                        (None, _) => None,
                        (Some(on), Some(op)) => {
                            let path = op.as_str();
                            let name = on.as_str();

                            let include = Include::new(path.to_string(), name.to_string());

                            println!("{:?}", include);

                            Some(include)
                        }
                    }
                }
                _ => None
            }
        }).collect::<Option<Vec<_>>>();

    return x;
}
