//#region constants
pub const DELIMITERS:   [&'static str; 2] = [";", "\n"];
pub const OPEN_SYMBOLS: [&'static str; 4] = ["<", "{", "(", "["];
pub const CLOSE_SYMBOLS:[&'static str; 4] = [">", "}", ")", "]"];
pub const SEPARATORS:   [&'static str; 2] = [",", " "];
pub const COMMENT:      [&'static str; 2] = ["/", "/"];
pub const COMMENT_CHAR: [char; 2] = ['/', '/'];
pub const COMMENT_STR:  [&'static str; 1] = ["//"];
pub const KEYWORDS:     [&'static str; 10] = [
    "#", "^", "?", "!",
    "@", "~", ":", ".",
    "<<", "//",
];
pub const SINGLE_KEYWORDS: [&'static str; 8] = [
    "#", "^", "?", "!",
    "@", "~", ":", ".",
];
pub const DOUBLE_KEYWORDS: [&'static [&'static str; 2]; 2] = [
    &["<","<"],
    &["/", "/"],
];
pub const OPERATORS: [&'static str; 19] = [
    "=", "*", "+", "-", "/", "%",
    "<", ">", "|", "&",
    "++", "--", "-=", "+=",
    "==", ">=", "<=",
    "<<", ">>",
];
pub const STR_DELIMETER_DECLARATOR: [&'static str; 1] = ["qq"];
pub const STR_DELIMETER_DECLARATOR_DUO: [&'static [&'static str; 2]; 1] = [
    &["q", "q"],
];
pub const SINGLE_OPERATORS: [&'static str; 10] = [
    "=", "*", "+", "-", "/", "%",
    "<", ">", "|", "&",
];
pub const DOUBLE_OPERATORS: [&'static [&'static str; 2]; 9] = [
    &["+","+"], // Increase Assignmet
    &["-","-"], // Decrease Assignment
    &["-","="], // Subtractional Assignment
    &["+","="], // Addition Assignment
    &["=","="],
    &[">","="],
    &["<","="],
    &["<","<"],
    &[">",">"],
];
pub const BASIC_TYPES: [&'static str; 6] = ["int", "flt", "bol", "chr", "str", "vec"];
//#endregion