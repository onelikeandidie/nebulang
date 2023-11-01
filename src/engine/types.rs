use std::{fmt::Display};

#[derive(Debug, Default, Clone)]
pub struct Symbol {
    pub symbol: String,
    pub start: Cursor,
    pub end: Cursor,
}

impl Symbol {
    pub fn new(string: String, cursor: Cursor) -> Self {
        let len: u64;
        match string.len() {
            0 => {
                len = 1;
            },
            _ => {
                len = string.len() as u64;
            }
        }
        let start = Cursor {
            column: cursor.column - (len - 1),
            pos: cursor.pos - (len - 1),
            line: cursor.line
        };
        Self {
            symbol: string,
            start,
            end: cursor,
        } 
    }
    pub fn len(self: &Self) -> usize {
        return (self.end.pos - self.start.pos + 1).try_into().unwrap();
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}'({}:{})", self.symbol, self.start.line, self.start.column)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Node {
    pub id: u64,
    pub parent: u64,
    pub token: TokenTypes,
    pub children: Vec<u64>,
    pub cursor: Cursor,
    pub len: usize,
}

impl Node {
    pub fn root() -> Self {
        Self {
            token: TokenTypes::Root,
            ..Default::default()
        }
    }
    pub fn new(
        id: u64,
        token: TokenTypes,
        cursor: Cursor,
        parent: u64,
        len: usize
    ) -> Self {
        Self {
            id,
            token,
            cursor,
            parent,
            len,
            children: vec![]
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenTypes {
    Unknown,
    Delimiter,
    Comment,
    Keyword,
    Separator,
    Operator(Operators),
    Type(DataTypes),
    Generic,
    LiteralFloat(f64),
    LiteralInt(i64),
    LiteralString(String),
    Function(String),
    Variable(String),
    Struct(String),
    Call(String),
    CallParams,
    Statement,
    Implement,
    Body,
    Params,
    Root,
    Return,
    ShortReturn,
}

impl Default for TokenTypes {
    fn default() -> Self {
        return Self::Unknown;
    }
}

impl Display for TokenTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            // TokenTypes::Delimiter => todo!(),
            // TokenTypes::Keyword => todo!(),
            // TokenTypes::Separator => todo!(),
            TokenTypes::Operator(operator) => format!("{}", operator),
            TokenTypes::Type(t) => format!("Type({})", t),
            TokenTypes::LiteralFloat(flt) => format!("LitFlt({})", flt),
            TokenTypes::LiteralInt(int) => format!("LitInt({})", int),
            TokenTypes::LiteralString(string) => format!("LitStr({})", string),
            TokenTypes::Function(identifier) => format!("Fun({})", identifier),
            TokenTypes::Variable(var) => format!("Var({})", var),
            TokenTypes::Call(function) => format!("Call({})", function),
            // TokenTypes::Struct(_) => todo!(),
            // TokenTypes::Implement => todo!(),
            TokenTypes::CallParams => "CallParams".to_string(),
            TokenTypes::Generic => "Generic".to_string(),
            TokenTypes::Body => "Body".to_string(),
            TokenTypes::Statement => "Statement".to_string(),
            TokenTypes::Params => "Params".to_string(),
            TokenTypes::Root => "Root".to_string(),
            TokenTypes::Return => "Return".to_string(),
            TokenTypes::ShortReturn => "ShortReturn".to_string(),
            _ => "Unkown".to_string()
})
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum DataTypes {
    Unknown,
    Int,
    Flt,
    Bol,
    Chr,
    Str,
    Vec,
    User(String)
}

impl TryFrom<String> for DataTypes {
    type Error = &'static str;
    fn try_from(symbol: String) -> Result<Self, Self::Error> {
        match symbol.as_str() {
            "int" => Ok(Self::Int),
            "flt" => Ok(Self::Flt),
            "bol" => Ok(Self::Bol),
            "chr" => Ok(Self::Chr),
            "str" => Ok(Self::Str),
            "vec" => Ok(Self::Vec),
            _ => Err("Not a basic type")
            // _ => Ok(Self::User(symbol))
        }
    }
}

impl Default for DataTypes {
    fn default() -> Self {
        return Self::Unknown;
    }
}

impl Display for DataTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operators {
    Ass, // Assignment
    Mul,
    Add,
    Sub,
    Div,
    Mod, // Modulo
    Lt, // <
    Gt, // >
    BitOr,
    BitAnd,
    AddAdd, // ++
    SubSub, // --
    AssAdd, // +=
    AssSub, // -=
    Equ, // Equality
    EquGt,
    EquLt,
    BitLeft,
    BitRight,
}

impl TryFrom<String> for Operators {
    type Error = &'static str;
    fn try_from(symbol: String) -> Result<Self, Self::Error> {
        match symbol.as_str() {
            "=" => Ok(Self::Ass),
            "*" => Ok(Self::Mul),
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Sub),
            "/" => Ok(Self::Div),
            "%" => Ok(Self::Mod),
            "<" => Ok(Self::Lt),
            ">" => Ok(Self::Gt),
            "|" => Ok(Self::BitOr),
            "&" => Ok(Self::BitAnd),
            "++" => Ok(Self::AddAdd),
            "--" => Ok(Self::SubSub),
            "+=" => Ok(Self::AssAdd),
            "-=" => Ok(Self::AssSub),
            "==" => Ok(Self::Equ),
            ">=" => Ok(Self::EquGt),
            "<=" => Ok(Self::EquLt),
            "<<" => Ok(Self::BitLeft),
            ">>" => Ok(Self::BitRight),
            _ => Err("Expected operator")
        }
    }
}

impl Display for Operators {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    pub pos: u64,
    pub line: u64,
    pub column: u64,
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            pos: 0,
            line: 1,
            column: 1,
        }
    }
}

impl Display for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}->{}:{}", self.pos, self.line, self.column)
    }
}

pub struct Scope {
    pub functions: Vec<String>,
    pub variables: Vec<String>,
}

impl Default for Scope {
    fn default() -> Self {
        Self {
            functions: vec![],
            variables: vec![]
        }
    }
}

type ScopeError = &'static str;

impl Scope {
    pub fn function_exists(
        self: &Self,
        function: &String,
    ) -> bool {
        self.functions.contains(&function)
    }
    pub fn insert_function(
        self: &mut Self,
        function: String
    ) -> Result<(), ScopeError> {
        if self.functions.contains(&function) {
            return Err("Function was already defined");
        }
        self.functions.push(function);
        return Ok(())
    }
    pub fn variable_exists(
        self: &Self,
        variable: &String,
    ) -> bool {
        self.variables.contains(&variable)
    }
    pub fn insert_variable(
        self: &mut Self,
        variable: String
    ) -> Result<(), ScopeError> {
        if self.variable_exists(&variable) {
            return Ok(())
        }
        self.variables.push(variable);
        return Ok(())
    }
}