 mod littleduck_l {
use lrlex::{LexerDef, LRNonStreamingLexerDef, Rule};

#[allow(dead_code)]
pub fn lexerdef() -> LRNonStreamingLexerDef<lrlex::lexemes::DefaultLexeme, u32> {
    let rules = vec![
        Rule::new(Some(0), Some("PROGRAM".to_string()), "program".to_string()).unwrap(),
        Rule::new(Some(3), Some("VAR".to_string()), "var".to_string()).unwrap(),
        Rule::new(Some(5), Some("COLON".to_string()), ":".to_string()).unwrap(),
        Rule::new(Some(2), Some("SEMICOLON".to_string()), ";".to_string()).unwrap(),
        Rule::new(Some(17), Some("PLUSSIGN".to_string()), "\\+".to_string()).unwrap(),
        Rule::new(Some(18), Some("MINUSSIGN".to_string()), "-".to_string()).unwrap(),
        Rule::new(Some(22), Some("MULTIPLICATIONSIGN".to_string()), "\\*".to_string()).unwrap(),
        Rule::new(Some(23), Some("DIVISIONSIGN".to_string()), "/".to_string()).unwrap(),
        Rule::new(Some(6), Some("LEFTBRACKET".to_string()), "\\{".to_string()).unwrap(),
        Rule::new(Some(7), Some("RIGTHBRACKET".to_string()), "\\}".to_string()).unwrap(),
        Rule::new(Some(4), Some("COMMA".to_string()), ",".to_string()).unwrap(),
        Rule::new(Some(12), Some("EQUALSIGN".to_string()), "=".to_string()).unwrap(),
        Rule::new(Some(20), Some("LESSSIGN".to_string()), "<".to_string()).unwrap(),
        Rule::new(Some(19), Some("GREATERSIGN".to_string()), ">".to_string()).unwrap(),
        Rule::new(Some(21), Some("NOTEQUALSIGN".to_string()), "<>".to_string()).unwrap(),
        Rule::new(Some(13), Some("PRINT".to_string()), "print".to_string()).unwrap(),
        Rule::new(Some(9), Some("LEFTPARENTHESES".to_string()), "\\(".to_string()).unwrap(),
        Rule::new(Some(10), Some("RIGHTPARENTHESES".to_string()), "\\)".to_string()).unwrap(),
        Rule::new(Some(8), Some("IF".to_string()), "if".to_string()).unwrap(),
        Rule::new(Some(11), Some("ELSE".to_string()), "else".to_string()).unwrap(),
        Rule::new(Some(15), Some("INT".to_string()), "int".to_string()).unwrap(),
        Rule::new(Some(16), Some("FLOAT".to_string()), "float".to_string()).unwrap(),
        Rule::new(Some(1), Some("ID".to_string()), "[A-Za-z]\\w*".to_string()).unwrap(),
        Rule::new(Some(24), Some("CTEI".to_string()), "[+-]?(\\d+)".to_string()).unwrap(),
        Rule::new(Some(25), Some("CTEF".to_string()), "[+-]?(\\d+)\\.(\\d*)".to_string()).unwrap(),
        Rule::new(Some(14), Some("CTESTR".to_string()), "\"[^\"]*\"".to_string()).unwrap(),
        Rule::new(Some(26), None, "[\\t \\n]+".to_string()).unwrap(),
    ];
    LRNonStreamingLexerDef::from_rules(rules)
}

#[allow(dead_code)]
pub const T_ELSE: u32 = 11;
#[allow(dead_code)]
pub const T_PROGRAM: u32 = 0;
#[allow(dead_code)]
pub const T_PLUSSIGN: u32 = 17;
#[allow(dead_code)]
pub const T_DIVISIONSIGN: u32 = 23;
#[allow(dead_code)]
pub const T_SEMICOLON: u32 = 2;
#[allow(dead_code)]
pub const T_COMMA: u32 = 4;
#[allow(dead_code)]
pub const T_ID: u32 = 1;
#[allow(dead_code)]
pub const T_VAR: u32 = 3;
#[allow(dead_code)]
pub const T_RIGTHBRACKET: u32 = 7;
#[allow(dead_code)]
pub const T_PRINT: u32 = 13;
#[allow(dead_code)]
pub const T_CTEF: u32 = 25;
#[allow(dead_code)]
pub const T_GREATERSIGN: u32 = 19;
#[allow(dead_code)]
pub const T_FLOAT: u32 = 16;
#[allow(dead_code)]
pub const T_LEFTPARENTHESES: u32 = 9;
#[allow(dead_code)]
pub const T_LEFTBRACKET: u32 = 6;
#[allow(dead_code)]
pub const T_INT: u32 = 15;
#[allow(dead_code)]
pub const T_RIGHTPARENTHESES: u32 = 10;
#[allow(dead_code)]
pub const T_MINUSSIGN: u32 = 18;
#[allow(dead_code)]
pub const T_MULTIPLICATIONSIGN: u32 = 22;
#[allow(dead_code)]
pub const T_LESSSIGN: u32 = 20;
#[allow(dead_code)]
pub const T_COLON: u32 = 5;
#[allow(dead_code)]
pub const T_NOTEQUALSIGN: u32 = 21;
#[allow(dead_code)]
pub const T_CTESTR: u32 = 14;
#[allow(dead_code)]
pub const T_EQUALSIGN: u32 = 12;
#[allow(dead_code)]
pub const T_IF: u32 = 8;
#[allow(dead_code)]
pub const T_CTEI: u32 = 24;
}