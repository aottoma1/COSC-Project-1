//! Token definitions for the LOLCODE lexer.
//! 
//! This module defines the token types used during lexical analysis of LOLCODE source files.
//! Tokens represent the atomic units of the language, such as keywords, hashtag words, 
//! text content, and variables.

/// Represents the different types of tokens in the LOLCODE language.
/// 
/// # Variants
/// 
/// * `HashWord` - Keywords prefixed with `#` (e.g., `#HAI`, `#KTHXBYE`, `#I HAZ`)
/// * `Keyword` - Language keywords without `#` prefix (e.g., `HEAD`, `TITLE`, `PARAGRAF`)
/// * `Address` - URL addresses for multimedia content
/// * `Text` - Plain text content that isn't a keyword
/// * `VarDef` - Variable definition identifier
/// * `VarVal` - Variable value content
/// * `Newline` - Explicit newline token
/// * `Eof` - End of file marker
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

/// Represents a complete token with its type and source location information.
/// 
/// Tracks line and column numbers for error reporting during compilation.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
}