//taking from other token.rs without having to repeat
use crate::token::{Token, TokenKind};
//exit when something illegal found
use std::process::exit;

//Lexical analyzer trait
pub trait LexicalAnalyzer{
    fn get_char(&mut self) -> Option<char>;
    fn add_char(&mut self, c: char);
    fn lookup(&self, s: &str) -> bool;
    fn get_next_token(&mut self) -> Token;
}

// Char by char Lexer
pub struct Lexer<'a>{
    src: &'a str,
    iter: std::str::CharIndices<'a>,
    look: Option<(usize, char)>, 
    pub line: usize,
    pub col: usize,
    // for building a lexeme
    cur: String,
}

impl <'a> Lexer <'a>{
    pub fn new(src: &'a str) -> Self {
        let mut iter = src.char_indices();
        let look = iter.next();
        Self {
            src,
            iter,
            look,
            line: 1,
            col: 1,
            cur: String::new(),
        }
    }
    
    fn peek(&self) -> Option<char>{
        self.look.map(|(_,c)| c)
    }

    //returns current character and moves counter to next one
    fn bump(&mut self) -> Option<char> {
        let ch = self.look?; // grabs current characteer
        // advance line/col
        if ch.1 == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        self.look = self.iter.next(); // go to next character
        Some(ch.1) // return character just consumed
    }
    
    fn error_exit(&self, msg: &str) -> ! {
        eprintln!("Lexical error at line {}, col {}: {}", self.line, self.col, msg);
        exit(1);
    }

    fn is_hash_word(&self, upper: &str) -> bool{
        matches!(upper, 
            "HAI" | "KTHXBYE" | "OBTW" | "TLDR" | "MAEK" | "OIC" | 
            "GIMMEH" | "MKAY" | "I HAZ" | "IT IZ" | "LEMME SEE"
        )
    }

    fn is_keyword(&self, upper: &str) -> bool {
        matches!(upper,
            "HEAD" | "TITLE" | "PARAGRAF" | "BOLD" | "ITALICS" | 
            "LIST" | "ITEM" | "NEWLINE" | "SOUNDZ" | "VIDZ"
        )
    }
    // ensures every #OBTW has a closing #TLDR which is technically some syntax analysis but only for comments
    fn skip_multiline_comment(&mut self) {
        loop {
            if self.peek().is_none() {
                self.error_exit("Unclosed comment block - missing #TLDR");
            }
            
            if self.peek() == Some('#') {
                self.bump(); // consume #
                
                let mut word = String::new();
                while let Some(c) = self.peek() {
                    if c.is_ascii_alphabetic() {
                        word.push(self.bump().unwrap());
                    } else {
                        break;
                    }
                }
                
                if word.to_ascii_uppercase() == "TLDR" {
                    return; // Comment block closed
                }
                // Not TLDR, continue searching
            } else {
                self.bump();
            }
        }
    }

    fn read_hash_word(&mut self, start_line: usize, start_col: usize) -> Token {
        //consume #
        self.get_char();
        
        //read first word
        self.cur.clear();
        while let Some(c) = self.peek() {
            if c.is_ascii_alphabetic() {
                let ch = self.get_char().unwrap();
                self.add_char(ch);
            } else {
                break;
            }
        }
        
        let first_word = self.cur.to_ascii_uppercase();
        
        // Check for hashkey words with 2 words: #I HAZ, #IT IZ, #LEMME SEE
        let full_word = if first_word == "I" && self.peek() == Some(' ') {
            self.get_char(); // consume space
            let mut second = String::new();
            while let Some(c) = self.peek() {
                if c.is_ascii_alphabetic() {
                    let ch = self.get_char().unwrap();
                    second.push(ch);
                } else {
                    break;
                }
            }
            if second.to_ascii_uppercase() == "HAZ" {
                "I HAZ".to_string()
            } else {
                first_word
            }
        } else if first_word == "IT" && self.peek() == Some(' ') {
            self.get_char(); // consume space
            let mut second = String::new();
            while let Some(c) = self.peek() {
                if c.is_ascii_alphabetic() {
                    let ch = self.get_char().unwrap();
                    second.push(ch);
                } else {
                    break;
                }
            }
            if second.to_ascii_uppercase() == "IZ" {
                "IT IZ".to_string()
            } else {
                first_word
            }
        } else if first_word == "LEMME" && self.peek() == Some(' ') {
            self.get_char(); // consume space
            let mut second = String::new();
            while let Some(c) = self.peek() {
                if c.is_ascii_alphabetic() {
                    let ch = self.get_char().unwrap();
                    second.push(ch);
                } else {
                    break;
                }
            }
            if second.to_ascii_uppercase() == "SEE" {
                "LEMME SEE".to_string()
            } else {
                first_word
            }
        } else {
            first_word
        };
        
        //checking if valid hashtag word using lookup
        if !self.lookup(&full_word) {
            self.error_exit(&format!("Unrecognized hashtag word '#{}'", full_word));
        }
        
        // OBTW...TLDR is a multi-line comment block - skip it entirely
        if full_word == "OBTW" {
            self.skip_multiline_comment();
            // After comment, get next real token
            return self.get_next_token();
        }
        
        //put together token
        Token {
            kind: TokenKind::HashWord(format!("#{}", full_word)),
            line: start_line,
            col: start_col,
        }
    }

    fn read_word(&mut self, start_line: usize, start_col: usize) -> Token {
        self.cur.clear();
        
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() {
                let ch = self.get_char().unwrap();
                self.add_char(ch);
            } else {
                break;
            }
        }
        
        let upper = self.cur.to_ascii_uppercase();
        
        // Check if it's a keyword using lookup
        if self.lookup(&upper) {
            Token {
                kind: TokenKind::Keyword(upper),
                line: start_line,
                col: start_col,
            }
        } else {
            // defining a variable
            Token {
                kind: TokenKind::VarDef(self.cur.clone()),
                line: start_line,
                col: start_col,
            }
        }
    }

    fn read_text_line(&mut self, start_line: usize, start_col: usize) -> Token {
        let mut text = String::new();
        
        while let Some(c) = self.peek() {
            // Stop at newline or hashtag
            if c == '\n' || c == '#' {
                break;
            }
            text.push(self.bump().unwrap());
        }
        
        let trimmed = text.trim().to_string();
        
        // If empty skip to next token
        if trimmed.is_empty() {
            return self.get_next_token();
        }
        
        Token {
            kind: TokenKind::Text(trimmed),
            line: start_line,
            col: start_col,
        }
    }
}

// Implement the trait
impl<'a> LexicalAnalyzer for Lexer<'a> {
    fn get_char(&mut self) -> Option<char> {
        self.bump()
    }

    fn add_char(&mut self, c: char) {
        self.cur.push(c);
    }

    fn lookup(&self, s: &str) -> bool {
        self.is_keyword(s) || self.is_hash_word(s)
    }

    fn get_next_token(&mut self) -> Token {
        // Skip spaces/tabs, don't ignore newlines
        while let Some(c) = self.peek() {
            if c == ' ' || c == '\t' {
                self.bump();
            } else {
                break;
            }
        }

        let start_line = self.line;
        let start_col = self.col;

        // Check for EOF
        let ch = match self.peek() {
            Some(c) => c,
            None => return Token {
                kind: TokenKind::Eof,
                line: start_line,
                col: start_col,
            },
        };

        // newlines are significant
        if ch == '\n' {
            self.bump();
            return Token {
                kind: TokenKind::Newline,
                line: start_line,
                col: start_col,
            };
        }

        // Check for hashtag tokens
        if ch == '#' {
            return self.read_hash_word(start_line, start_col);
        }

        // Handle keywords and variable names
        if ch.is_ascii_alphabetic() {
            return self.read_word(start_line, start_col);
        }

        // anything else is treated as plain text (numbers, punctuation, etc.)
        self.read_text_line(start_line, start_col)
    }
}