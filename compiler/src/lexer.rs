//! Lexical analyzer for the LOLCODE language.
//! 
//! This module implements a character-by-character lexer that tokenizes LOLCODE source code.
//! The lexer recognizes keywords, hashtag words, variables, text content, and handles
//! multi-line comments.

//taking from other token.rs without having to repeat
use crate::token::{Token, TokenKind};
//exit when something illegal found
use std::process::exit;

/// Trait defining the interface for lexical analysis.
/// 
/// Provides methods for character-level scanning and token recognition.
pub trait LexicalAnalyzer {
    /// Retrieves and consumes the next character from the input.
    fn get_char(&mut self) -> Option<char>;
    
    /// Appends a character to the current lexeme being built.
    fn add_char(&mut self, c: char);
    
    /// Checks if a string is a valid keyword or hashtag word.
    fn lookup(&self, s: &str) -> bool;
    
    /// Retrieves the next token from the input stream.
    fn get_next_token(&mut self) -> Token;
}

/// Character-by-character lexer implementation for LOLCODE.
/// 
/// Maintains source position (line and column), character lookahead,
/// and builds lexemes token by token.
pub struct Lexer<'a> {
    src: &'a str,
    iter: std::str::CharIndices<'a>,
    look: Option<(usize, char)>, 
    /// Current line number (1-indexed)
    pub line: usize,
    /// Current column number (1-indexed)
    pub col: usize,
    // for building a lexeme
    cur: String,
}

impl <'a> Lexer <'a> {
    /// Creates a new lexer for the given source code.
    /// 
    /// Initializes the lexer at line 1, column 1 with the first character loaded.
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
    
    /// Peeks at the current character without consuming it.
    fn peek(&self) -> Option<char> {
        self.look.map(|(_,c)| c)
    }

    /// Consumes and returns the current character, advancing to the next one.
    /// 
    /// Updates line and column counters based on the consumed character.
    //returns current character and moves counter to next one
    fn bump(&mut self) -> Option<char> {
        let ch = self.look?; // grabs current character
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
    
    /// Reports a lexical error and exits the program.
    /// 
    /// # Panics
    /// 
    /// Always exits with status code 1
    fn error_exit(&self, msg: &str) -> ! {
        eprintln!("Lexical error at line {}, col {}: {}", self.line, self.col, msg);
        exit(1);
    }

    /// Checks if a string is a valid hashtag word.
    fn is_hash_word(&self, upper: &str) -> bool {
        matches!(upper, 
            "HAI" | "KTHXBYE" | "OBTW" | "TLDR" | "MAEK" | "OIC" | 
            "GIMMEH" | "MKAY" | "I HAZ" | "IT IZ" | "LEMME SEE"
        )
    }

    /// Checks if a string is a valid language keyword.
    fn is_keyword(&self, upper: &str) -> bool {
        matches!(upper,
            "HEAD" | "TITLE" | "PARAGRAF" | "BOLD" | "ITALICS" | 
            "LIST" | "ITEM" | "NEWLINE" | "SOUNDZ" | "VIDZ"
        )
    }
    
    /// Skips a multi-line comment block (`#OBTW` ... `#TLDR`).
    /// 
    /// Ensures every `#OBTW` has a matching `#TLDR` closing tag.
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

    /// Reads a hashtag word token (e.g., `#HAI`, `#I HAZ`, `#LEMME SEE`).
    /// 
    /// Handles both single-word and two-word hashtag keywords.
    /// Multi-line comments (`#OBTW` ... `#TLDR`) are skipped entirely.
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

    /// Reads a word token (keyword or variable name).
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

    /// Reads a line of plain text content.
    /// 
    /// Stops at newlines or hashtag symbols. Empty text is skipped.
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

    /// Retrieves the next token from the input.
    /// 
    /// Skips whitespace (spaces and tabs) but preserves newlines as tokens.
    /// Recognizes hashtag words, keywords, variables, and text content.
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