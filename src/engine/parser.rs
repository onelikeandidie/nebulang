use std::fmt::{Debug};
use std::fs::File;

use hashbrown::HashMap;

use super::config::Config;
use super::util::{Conveyor, CharReader};
use super::symbols::*;
use super::types::*;

#[derive(Debug)]
pub struct Parser {
    pub nodes: Vec<Node>,
    pub symbols: Vec<Symbol>,
    pub functions: HashMap<String, String>,
    config: Config,
}

#[derive(Debug)]
pub enum ParseError {
    Syntax,
    Arguments,
    Unknown,
    Expected,
}

#[derive(Debug)]
pub struct SymbolState {
    is_in_string: bool,
    is_escaping: bool,
    is_in_comment: bool,
    is_setting_string_delimiter: bool,
    could_be_double: Vec<char>,
    current_string_delimiter: char,
}

impl Default for SymbolState {
    fn default() -> Self {
        Self {
            is_in_string:   false,
            is_escaping:    false,
            is_in_comment:  false,
            is_setting_string_delimiter: false,
            could_be_double:vec![],
            current_string_delimiter: '"',
        }
    }
}

pub struct TokenizerState {

}

impl Default for TokenizerState {
    fn default() -> Self {
        Self {
            
        }
    }
}

#[derive(Debug)]
pub enum LexResult {
    None,
    New(Node),
    ChangeTo(u64),
    Up,
}

impl LexResult {
    fn short_code(self: &Self) -> String {
        match self {
            Self::None => "",
            Self::New(_) => "N",
            Self::ChangeTo(_) => "C",
            Self::Up => "U",
        }.to_string()
    }
}

impl Parser {
    pub fn new(config: Config) -> Self {
        Self {
            nodes:      vec![],
            symbols:    vec![],
            functions:  HashMap::new(),
            config,
        }
    }

    pub fn parse(self: &mut Self, file: File) -> Result<(), ParseError> {
        let symbols = self.extract_symbols(file)
            .expect("Could not parse symbols");
        self.symbols = symbols.clone();
        self.register_functions(super::core::register())
            .expect("Could not register core functions");
        let nodes = self.tokenize_symbols(symbols)
            .expect("Could not tokenize");
        self.nodes = nodes;
        Ok(())
    }

    pub fn extract_symbols(
        self: &mut Self,
        file: File
    ) -> Result<Vec<Symbol>, ParseError> {
        let mut reader = CharReader::new(file, Some(self.config.low_mem));
        let mut char_buf: Conveyor<char> = Conveyor::new(64);
        let mut cursor = Cursor::default();
        let mut symbol_state = SymbolState::default();
        let mut symbols = Vec::new();
        while let Some(character) = reader.next() {
            char_buf.push(character);
            cursor.pos += 1;
            if let Some(new_symbols) = self.read_symbol(
                character,
                &mut char_buf,
                &cursor,
                &mut symbol_state,
            ) {
                for symbol in new_symbols {
                    symbols.push(symbol);
                }
            }
            if character == '\n' {
                cursor.line += 1;
                cursor.column = 0;
            }
            cursor.column += 1;
        }
        Ok(symbols)
    }

    pub fn read_symbol(
        self: &mut Self, 
        character: char,
        char_buf: &mut Conveyor<char>,
        cursor: &Cursor,
        mut symbol_state: &mut SymbolState,
    ) -> Option<Vec<Symbol>> {
        let c_string = character.to_string();
        let c_str = c_string.as_str();
        if character == '\\' {
            symbol_state.is_escaping = true;
            return None;
        }
        if symbol_state.is_escaping {
            symbol_state.is_escaping = false;
        }
        if symbol_state.is_in_comment {
            if DELIMITERS.contains(&c_str) {
                char_buf.clear();
                symbol_state.is_in_comment = false;
            }
            return None;
        }
        if symbol_state.is_setting_string_delimiter {
            // Set the string delimiter after qq
            symbol_state.current_string_delimiter = character;
            symbol_state.is_setting_string_delimiter = false;
            symbol_state.is_in_string = true;
            // char_buf.clear();
            return None;
        }
        if !symbol_state.could_be_double.is_empty() {
            let len = symbol_state.could_be_double.len();
            if len > 0 {
                let last = symbol_state.could_be_double[len - 1].to_string();
                if c_str == COMMENT[1]
                && last.as_str() == COMMENT[0] {
                    symbol_state.is_in_comment = true;
                    char_buf.clear();
                    symbol_state.could_be_double.clear();
                    return None;
                }
            }
            // Check if it is changing the string delimiter
            if STR_DELIMETER_DECLARATOR_DUO
                .map(|v| v[len]).contains(&c_str) {
                symbol_state.is_setting_string_delimiter = true;
                symbol_state.could_be_double.clear();
                return None;
            }
            if DOUBLE_OPERATORS.map(|v| v[len]).contains(&c_str)
            || DOUBLE_KEYWORDS .map(|v| v[len]).contains(&c_str) {
                char_buf.push(character);
                let identifier = char_buf.to_string();
                let s = Symbol::new(
                    identifier,
                    cursor.clone()
                );
                char_buf.clear();
                symbol_state.could_be_double.clear();
                return Some(vec![s]);
            }
            let single_maybe = String::from_iter(
                symbol_state.could_be_double.clone()
            );
            let single_maybe_str = &single_maybe.as_str();
            if SINGLE_OPERATORS.contains(single_maybe_str)
            || SINGLE_KEYWORDS .contains(single_maybe_str) {
                let identifier = single_maybe;
                let s = Symbol::new(
                    identifier,
                    cursor.clone()
                );
                symbol_state.could_be_double.clear();
                if let Some(mut next_ss) = self.read_symbol(
                    character,
                    char_buf,
                    cursor,
                    symbol_state,
                ) {
                    let mut result = vec![s];
                    result.append(&mut next_ss);
                    return Some(result);
                }
                // char_buf.clear();
                return Some(vec![s]);
            }
        }
        if character == symbol_state.current_string_delimiter.clone() {
            if !symbol_state.is_in_string {
                symbol_state.is_in_string = true;
                return None;
            } else {
                symbol_state.is_in_string = false;
                // reset string delimiter to default
                symbol_state.current_string_delimiter = self.config.string_delimiter;
                let identifier = char_buf.to_string();
                char_buf.clear();
                if identifier.len() > 0 {
                    let result = Symbol::new(
                        identifier,
                        cursor.clone()
                    );
                    return Some(vec![result]);
                }
            }
        }
        if symbol_state.is_in_string {
            return None;
        }
        if DOUBLE_OPERATORS
            .map(|v| v[0]).contains(&c_str)
        || DOUBLE_KEYWORDS 
            .map(|v| v[0]).contains(&c_str)
        || STR_DELIMETER_DECLARATOR_DUO 
            .map(|v| v[0]).contains(&c_str)
            {
            symbol_state.could_be_double.push(character);
            char_buf.pop();
            let identifier = char_buf.to_string();
            char_buf.clear();
            if identifier.len() > 0 {
                // Cursor in the identifier should be the original minus the
                // space occupied by the reserved char
                let result_cursor = Cursor {
                    pos: cursor.pos - 1,
                    column: cursor.column - 1,
                    line: cursor.line
                };
                let result = Symbol::new(
                    identifier,
                    result_cursor
                );
                return Some(vec![result]);
            }
            return None;
        }
        if DELIMITERS.contains(&c_str)
        || OPEN_SYMBOLS.contains(&c_str) 
        || CLOSE_SYMBOLS.contains(&c_str) 
        || SEPARATORS.contains(&c_str) 
        || SINGLE_OPERATORS.contains(&c_str)
        || SINGLE_KEYWORDS .contains(&c_str)
        {
            char_buf.pop();
            let identifier = char_buf.to_string();
            char_buf.clear();
            let mut new_symbols: Vec<Symbol> = vec![];
            if identifier.len() > 0 {
                // Cursor in the identifier should be the original minus the
                // space occupied by the reserved char
                let result_cursor = Cursor {
                    pos: cursor.pos - 1,
                    column: cursor.column - 1,
                    line: cursor.line
                };
                let result = Symbol::new(
                    identifier,
                    result_cursor
                );
                new_symbols.push(result);
            }
            if character != ' ' {
                let s_cursor = Cursor {
                    pos: cursor.pos,
                    column: cursor.column,
                    line: cursor.line
                };
                let s = Symbol::new(
                    c_string.clone(),
                    s_cursor,
                );
                new_symbols.push(s);
            }
            return Some(new_symbols);
        }
        None
    }

    pub fn register_functions(
        self: &mut Self,
        functions: Vec<(String, String)>
    ) -> Result<(), ()> {
        for (call, function) in functions {
            if let Some(_) = self.functions.get(&call) {
                return Err(());
            }
            self.functions.insert(call, function);
        }
        Ok(())
    }

    pub fn tokenize_symbols(
        self: &mut Self, 
        symbols: Vec<Symbol>
    ) -> Result<Vec<Node>, ParseError> {
        let mut nodes: Vec<Node> = vec![];
        let mut working_id: u64 = 0;
        nodes.push(Node::root());
        let mut carryover: Conveyor<Symbol> = Conveyor::new(16);
        let mut scope = Scope::default();
        for symbol in symbols {
            let next_id = nodes.len() as u64;
            let mut working_node = nodes.get(
                working_id as usize
            ).unwrap().clone();
            print!("{} ", symbol.symbol);
            if let Some(lex_result) = self.lex(
                &working_node,
                &symbol,
                &mut carryover,
                next_id,
                &scope,
            ) {
                for result in lex_result {
                    print!("{} ", result.short_code());
                    match result {
                        LexResult::New(node) => {
                            print!("{} {} ", node.id, node.token);
                            // If the new node is a variable add them to scope
                            match node.token.clone() {
                                TokenTypes::Variable(var) => {
                                    scope.insert_variable(var)
                                        .unwrap();
                                }
                                TokenTypes::Function(fun) => {
                                    scope.insert_function(fun)
                                        .unwrap();
                                    // Clear variable scope
                                    scope.variables.clear();
                                }
                                _ => {}
                            }
                            nodes.push(node);
                        }
                        LexResult::ChangeTo(node_id) => {
                            print!("{}->{} ", working_node.id, node_id);
                            working_id = node_id;
                        }
                        LexResult::Up => {
                            print!("{}->{} ", working_node.id, working_node.parent);
                            // If the exit is from a Function node, the scope
                            // should be reset for variables
                            match working_node.token.clone() {
                                TokenTypes::Function(_) => {
                                    println!("{:?}", scope.variables);
                                }
                                _ => {}
                            }
                            working_id = working_node.parent;
                            working_node = nodes.get(working_id as usize).unwrap().clone();
                        }
                        _ => {},
                    }
                }
            }
            println!();
        }
        println!("{:?}", working_id);
        Ok(nodes)
    }

    pub fn lex(
        self: &mut Self,
        working_node: &Node,
        symbol: &Symbol,
        carryover: &mut Conveyor<Symbol>,
        next_id: u64,
        scope: &Scope,
    ) -> Option<Vec<LexResult>> {
        let c_symbol = symbol.symbol.as_str();
        match working_node.token {
            TokenTypes::Function(_) => {
                if let Some(carry) = carryover.last() {
                    match carry.symbol.as_str() {
                        "}" => {
                            carryover.pop();
                            return Some(vec![
                                LexResult::Up,
                            ]);
                        }
                        _ => {},
                    }
                }
                match c_symbol {
                    "(" => {
                        let result = Node::new(
                            next_id, 
                            TokenTypes::Params,
                            symbol.start,
                            working_node.id,
                            symbol.len()
                        );
                        return Some(vec![
                            LexResult::New(result),
                            LexResult::ChangeTo(next_id)
                        ]);
                    },
                    "{" => {
                        let result = Node::new(
                            next_id, 
                            TokenTypes::Body,
                            symbol.start,
                            working_node.id,
                            symbol.len()
                        );
                        return Some(vec![
                            LexResult::New(result),
                            LexResult::ChangeTo(next_id)
                        ]);
                    }
                    // One liner functions
                    "<<" => {
                        let result = Node::new(
                            next_id,
                            TokenTypes::ShortReturn,
                            symbol.start,
                            working_node.id,
                            symbol.len()
                        );
                        let statement = Node::new(
                            next_id + 1,
                            TokenTypes::Statement,
                            symbol.start,
                            next_id,
                            symbol.len()
                        );
                        return Some(vec![
                            LexResult::New(result),
                            LexResult::New(statement),
                            LexResult::ChangeTo(next_id + 1)
                        ]);
                    }
                    _ => {
                        // Return type?
                        if let Ok(operator) = symbol.symbol.clone().try_into() {
                            let result = Node::new(
                                next_id,
                                TokenTypes::Type(operator),
                                symbol.start,
                                working_node.id,
                                symbol.len()
                            );
                            return Some(vec![
                                LexResult::New(result),
                            ]);
                        }
                    },
                }
            }
            TokenTypes::Params => {
                match c_symbol {
                    ")" => {
                        return Some(vec![
                            LexResult::Up
                        ]);
                    }
                    "," => {
                        return None;
                    }
                    _ => {
                        if let Ok(operator) = symbol.symbol.clone().try_into() {
                            let result = Node::new(
                                next_id,
                                TokenTypes::Type(operator),
                                symbol.start,
                                working_node.id,
                                symbol.len()
                            );
                            return Some(vec![
                                LexResult::New(result),
                                LexResult::ChangeTo(next_id)
                            ]);
                        }
                    },
                }
            }
            TokenTypes::Type(_) => {
                match c_symbol {
                    // Support generic<types>
                    "<" => {
                        let result = Node::new(
                            next_id, 
                            TokenTypes::Generic,
                            symbol.start,
                            working_node.id,
                            symbol.len()
                        );
                        return Some(vec![
                            LexResult::New(result),
                            LexResult::ChangeTo(next_id)
                        ]);
                    },
                    _ => {
                        let result = Node::new(
                            next_id,
                            TokenTypes::Variable(symbol.symbol.clone()),
                            symbol.start,
                            working_node.id,
                            symbol.len()
                        );
                        return Some(vec![
                            LexResult::New(result),
                            LexResult::Up,
                        ]);
                    },
                }
            }
            TokenTypes::Generic => {
                match c_symbol {
                    ">" => {
                        return Some(vec![
                            LexResult::Up,
                        ]);
                    }
                    "," => {
                        return None;
                    }
                    _ => {
                        if let Ok(operator) = symbol.symbol.clone().try_into() {
                            let result = Node::new(
                                next_id,
                                TokenTypes::Type(operator),
                                symbol.start,
                                working_node.id,
                                symbol.len()
                            );
                            return Some(vec![
                                LexResult::New(result),
                            ]);
                        }
                    }
                }
            }
            TokenTypes::Body => {
                match c_symbol {
                    "}" => {
                        // For checking if a function needs to be closed
                        carryover.push(symbol.clone());
                        return Some(vec![
                            LexResult::Up
                        ]);
                    }
                    "<<" => {
                        let result = Node::new(
                            next_id,
                            TokenTypes::Return,
                            symbol.start,
                            working_node.id,
                            symbol.len()
                        );
                        let statement = Node::new(
                            next_id + 1,
                            TokenTypes::Statement,
                            symbol.start,
                            next_id,
                            symbol.len()
                        );
                        return Some(vec![
                            LexResult::New(result),
                            LexResult::New(statement),
                            LexResult::ChangeTo(next_id + 1),
                        ]);
                    }
                    _ => {
                        // Could be a type to start a statement
                        if let Ok(operator) = symbol.symbol.clone().try_into() {
                            let statement = Node::new(
                                next_id,
                                TokenTypes::Statement,
                                symbol.start,
                                working_node.id,
                                0,
                            );
                            let result = Node::new(
                                next_id + 1,
                                TokenTypes::Type(operator),
                                symbol.start,
                                next_id,
                                symbol.len()
                            );
                            return Some(vec![
                                LexResult::New(statement),
                                LexResult::ChangeTo(next_id),
                                LexResult::New(result),
                            ]);
                        }
                        // Could be a function call
                        if let Some(function) = self.functions.get(&symbol.symbol) {
                            let result = Node::new(
                                next_id,
                                TokenTypes::Call(function.clone()),
                                symbol.start,
                                working_node.id,
                                symbol.len()
                            );
                            return Some(vec![
                                LexResult::New(result),
                                LexResult::ChangeTo(next_id),
                            ])
                        }
                    },
                }
            }
            TokenTypes::Call(_) => {
                match c_symbol {
                    "(" => {
                        let result = Node::new(
                            next_id, 
                            TokenTypes::CallParams,
                            symbol.start,
                            working_node.id,
                            symbol.len()
                        );
                        return Some(vec![
                            LexResult::New(result),
                            LexResult::ChangeTo(next_id)
                        ]);
                    },
                    _ => {}
                }
            }
            TokenTypes::CallParams => {
                match c_symbol {
                    ")" => {
                        return Some(vec![
                            LexResult::Up,
                            LexResult::Up,
                        ]);
                    }
                    _ => {
                        if let Some(lex_results) = self.lex_literals(
                            symbol,
                            c_symbol,
                            working_node,
                            next_id,
                            scope
                        ) {
                            return Some(lex_results);
                        }
                    }
                }
            }
            TokenTypes::Statement => {
                if DELIMITERS.contains(&c_symbol) {
                    return Some(vec![
                        LexResult::Up,
                    ]);
                }
                if let Some(lex_results) = self.lex_literals(
                    symbol,
                    c_symbol,
                    working_node,
                    next_id,
                    scope
                ) {
                    return Some(lex_results);
                }
            }
            TokenTypes::Return => {
                if DELIMITERS.contains(&c_symbol) {
                    return Some(vec![
                        LexResult::Up,
                    ]);
                }
                if let Some(lex_results) = self.lex_literals(
                    symbol,
                    c_symbol,
                    working_node,
                    next_id,
                    scope
                ) {
                    return Some(lex_results);
                }
            }
            TokenTypes::ShortReturn => {
                if DELIMITERS.contains(&c_symbol) {
                    return Some(vec![
                        LexResult::Up,
                        LexResult::Up,
                    ]);
                }
                if let Some(lex_results) = self.lex_literals(
                    symbol,
                    c_symbol,
                    working_node,
                    next_id,
                    scope
                ) {
                    return Some(lex_results);
                }
            }
            _ => {},
        }
        if symbol.symbol == "#" {
            carryover.push(symbol.clone());
            return None;
        }
        if let Some(carry) = carryover.pop() {
            match carry.symbol.as_str() {
                "#" => {
                    let result = Node::new(
                        next_id,
                        TokenTypes::Function(symbol.symbol.clone()),
                        symbol.start.clone(),
                        working_node.id,
                        symbol.len()
                    );
                    return Some(vec![
                        LexResult::New(result),
                        LexResult::ChangeTo(next_id)
                    ]);
                },
                _ => {}
            }
        }
        None
    }

    fn lex_literals(
        self: &Self,
        symbol: &Symbol,
        c_symbol: &str,
        working_node: &Node,
        next_id: u64,
        scope: &Scope,
    ) -> Option<Vec<LexResult>>{
        if OPERATORS.contains(&c_symbol) {
            let result = Node::new(
                next_id,
                TokenTypes::Operator(
                    symbol.symbol.clone().try_into()
                    .unwrap()
                ),
                symbol.start,
                working_node.id,
                symbol.len()
            );
            return Some(vec![
                LexResult::New(result)
            ])
        }
        // Could be a string with a modified delimiter
        if c_symbol.starts_with("q") {
            let mut literal_str = c_symbol.to_string();
            literal_str.remove(0);
            literal_str.remove(0);
            literal_str.remove(literal_str.len() - 1);
            let result = Node::new(
                next_id,
                TokenTypes::LiteralString(literal_str),
                symbol.start,
                working_node.id,
                symbol.len()
            );
            return Some(vec![
                LexResult::New(result),
            ]);
        }
        // Could be a string
        if c_symbol.starts_with("\"") {
            let mut literal_str = c_symbol.to_string();
            literal_str.remove(0);
            literal_str.remove(literal_str.len() - 1);
            let result = Node::new(
                next_id,
                TokenTypes::LiteralString(literal_str),
                symbol.start,
                working_node.id,
                symbol.len()
            );
            return Some(vec![
                LexResult::New(result),
            ]);
        }
        // Could be a float
        if c_symbol.contains(".") {
            let literal_flt = c_symbol.parse::<f64>();
            if let Ok(flt) = literal_flt {
                let result = Node::new(
                    next_id,
                    TokenTypes::LiteralFloat(flt),
                    symbol.start,
                    working_node.id,
                    symbol.len()
                );
                return Some(vec![
                    LexResult::New(result),
                ]);
            }
        }
        // Could be a literal int
        let literal_int = c_symbol.parse::<i64>();
        if let Ok(int) = literal_int {
            let result = Node::new(
                next_id,
                TokenTypes::LiteralInt(int),
                symbol.start,
                working_node.id,
                symbol.len()
            );
            return Some(vec![
                LexResult::New(result),
            ]);
        }
        // Could be a literal string
        if c_symbol.starts_with("\"") {
            let result = Node::new(
                next_id,
                TokenTypes::Variable(symbol.symbol.clone()),
                symbol.start,
                working_node.id,
                symbol.len()
            );
            return Some(vec![
                LexResult::New(result),
            ]);
        }
        if scope.function_exists(&symbol.symbol) {
            let result = Node::new(
                next_id,
                TokenTypes::Call(symbol.symbol.clone()),
                symbol.start,
                working_node.id,
                symbol.len()
            );
            return Some(vec![
                LexResult::New(result),
                LexResult::ChangeTo(next_id),
            ]);
        }
        if scope.variable_exists(&symbol.symbol) {
            let result = Node::new(
                next_id,
                TokenTypes::Variable(symbol.symbol.clone()),
                symbol.start,
                working_node.id,
                symbol.len()
            );
            return Some(vec![
                LexResult::New(result),
            ]);
        }
        return None;
    }
}
