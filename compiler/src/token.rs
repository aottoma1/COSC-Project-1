#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    //words leading with hashtag
    HashWord(String),

    // keywords that dont start with a hashtag
    Keyword(String),

    // text that isn't a defined keyword in the grammar
    Address(String),
    Text(String),      
    VarDef(String),   
    VarVal(String),
    Newline,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
}