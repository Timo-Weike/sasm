#[derive(Debug)]
pub struct Program {
    includes : Vec<Include>,
    static_data_block : StaticDataBlock,
    dynamic_data_block : DynamicDataBlock,
    code : Code
}

#[derive(Debug)]
pub struct Label(pub String);

impl Label {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug)]
pub struct Include {
    file_path : String,
    name : Label
}

impl Include {
    pub fn new(path :String, name :String) -> Include {
        Include {
            file_path : path,
            name : Label(name)
        }
    }
}

#[derive(Debug)]
pub struct StaticDataBlock {
    defs : Vec<StaticDataDef>
}

#[derive(Debug)]
pub struct StaticDataDef {
    name : Label,
    data : StaticData
}

#[derive(Debug)]
pub enum IntWidth {
    W8,
    W16,
    W32,
    W64
}

#[derive(Debug)]
pub enum FloatWidth {
    W32,
    W64
}

#[derive(Debug)]
pub enum StaticData {
    StringData(String),
    NumData {
        width : IntWidth,
        data : Vec<i64>
    },
    FloatData {
        width : FloatWidth,
        data : Vec<f64>
    }
}

#[derive(Debug)]
pub struct DynamicDataBlock {
    defs : Vec<DynamicDataDef>
}

impl DynamicDataBlock {
    pub fn new(defs :Vec<DynamicDataDef>) -> DynamicDataBlock {
        DynamicDataBlock { defs : defs }
    }
}

#[derive(Debug)]
pub struct DynamicDataDef {
    name : Label,
    width : IntWidth,
    amount : u64,
}

#[derive(Debug)]
pub struct Code {
    elements : Vec<CodeElement>
}

impl Code {
    pub fn new(elements :Vec<CodeElement>) -> Code {
        Code { elements : elements }
    }
}

#[derive(Debug)]
pub enum CodeElement {
    Intstruction(Intstruction),
    Label(Label),
    Section(Code)
}

#[derive(Debug)]
pub enum Operant {
    Addressing(Addressing),
    Register(Register)
}

#[derive(Debug)]
pub enum AddressingElement {
    Register(Register),
    Label(Label),
    Lit(i64)
}

#[derive(Debug)]
pub enum Addressing {
    Simple(AddressingElement),
    Complex {
        base : AddressingElement,
        factor : i64,
        offset : AddressingElement,
    }
}

#[derive(Debug)]
pub enum Register {
    Normal(u64),
    RI,
    RS
}

#[derive(Debug)]
pub enum Intstruction {
    NOP,
    Add {
        width : IntWidth,
        op1 : Operant,
        op2 : Operant
    }
}
