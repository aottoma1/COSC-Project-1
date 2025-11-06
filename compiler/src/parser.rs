use crate::token::{Token, TokenKind};
use crate::lexer::{Lexer, LexicalAnalyzer};
use std::process::exit;

/// Parser trait for syntax analysis
pub trait Parser {
    fn parse(&mut self);
    fn next_token(&mut self) -> Token;
    fn current_token(&self) -> &Token;
}

// Parse tree structure to match grammar
#[derive(Debug, Clone)]
pub enum ASTNode {
    Program { children: Vec<ASTNode> },
    HeadSection { children: Vec<ASTNode> },
    ParagrafSection { children: Vec<ASTNode> },
    ListSection { children: Vec<ASTNode> },
    VariableDeclaration { name: String },
    VariableAssignment { name: String, value: String },
    VariableReference { name: String },
    Title { content: String },
    Text { content: String },
    Bold { content: Vec<ASTNode> },
    Italics { content: Vec<ASTNode> },
    Item { content: Vec<ASTNode> },
    Newline,
    Sound { url: String },
    Video { url: String },
}

//parser implementation
pub struct LolcodeParser<'a> {
    lexer: Lexer<'a>,
    current_tok: Token,
    pub parse_tree: Option<ASTNode>,
}

impl<'a> LolcodeParser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Lexer::new(source);
        let first_token = lexer.get_next_token();
        
        Self {
            lexer,
            current_tok: first_token,
            parse_tree: None,
        }
    }

    // error reporting with line/col information
    fn syntax_error(&self, msg: &str) -> ! {
        eprintln!(
            "Syntax error at line {}, col {}: {}",
            self.current_tok.line, self.current_tok.col, msg
        );
        exit(1);
    }

    //  checking that current token matches expected hashword
    fn match_hashword(&mut self, expected: &str) {
        if let TokenKind::HashWord(ref hw) = self.current_tok.kind {
            if hw == expected {
                self.next_token();
                return;
            }
        }
        self.syntax_error(&format!("Expected '{}' but found {:?}", expected, self.current_tok.kind));
    }

    // Checking that current token matches expected keyword
    fn match_keyword(&mut self, expected: &str) {
        if let TokenKind::Keyword(ref kw) = self.current_tok.kind {
            if kw == expected {
                self.next_token();
                return;
            }
        }
        self.syntax_error(&format!("Expected keyword '{}' but found {:?}", expected, self.current_tok.kind));
    }

    // Skip optional newlines
    fn skip_newlines(&mut self) {
        while matches!(self.current_tok.kind, TokenKind::Newline) {
            self.next_token();
        }
    }

    // consume a newline
    fn match_newline(&mut self) {
        if matches!(self.current_tok.kind, TokenKind::Newline) {
            self.next_token();
        } else {
            self.syntax_error("Expected newline");
        }
    }

    // grammar: <program> ::= #HAI <body> #KTHXBYE
    fn program(&mut self) -> ASTNode {
        self.match_hashword("#HAI");
        self.skip_newlines();
        
        let body = self.body();
        
        self.skip_newlines();
        self.match_hashword("#KTHXBYE");
        self.skip_newlines();  // will still end program if there is extra white space at end
        
        // Check for EOF
        if !matches!(self.current_tok.kind, TokenKind::Eof) {
            self.syntax_error("Unexpected tokens after #KTHXBYE");
        }
        
        ASTNode::Program { children: body }
    }

    // <body> ::= { <section> | <content> }
    fn body(&mut self) -> Vec<ASTNode> {
        let mut nodes = Vec::new();
        
        loop {
            self.skip_newlines();
            
            // Check for end of program
            if let TokenKind::HashWord(ref hw) = self.current_tok.kind {
                if hw == "#KTHXBYE" {
                    break;
                }
                if hw == "#MAEK" {
                    // Section
                    nodes.push(self.section());
                    continue;
                }
                // Check for variable declarations at top level
                if hw == "#I HAZ" {
                    nodes.push(self.variable_declaration());
                    self.skip_newlines();
                    // Check for assignment that follows
                    if let TokenKind::HashWord(ref hw2) = self.current_tok.kind {
                        if hw2 == "#IT IZ" {
                            nodes.push(self.variable_assignment());
                        }
                    }
                    continue;
                }
            if hw == "#LEMME SEE" {
                nodes.push(self.variable_reference());
                continue;
            }
                // some other hashword
            if hw == "#GIMMEH" {
                nodes.push(self.styled_text());
                continue;
                }
            }
            
            // Check for EOF
            if matches!(self.current_tok.kind, TokenKind::Eof) {
                break;
            }
            
            // Deals w/ text or other
            match &self.current_tok.kind {
                TokenKind::Text(t) => {
                    let text = t.clone();
                    self.next_token();
                    nodes.push(ASTNode::Text { content: text });
                }
                TokenKind::VarDef(v) => {
                    let var = v.clone();
                    self.next_token();
                    nodes.push(ASTNode::Text { content: var });
                }
                _ => {
                    self.next_token();
                }
            }
        }
        
        nodes
    }

    // grammar: <section> ::= <head_section> | <paragraf_section> | <list_section>
    fn section(&mut self) -> ASTNode {
        if let TokenKind::HashWord(ref hw) = self.current_tok.kind {
            match hw.as_str() {
                "#MAEK" => {
                    self.next_token();
                    self.skip_newlines();
                    
                    if let TokenKind::Keyword(ref kw) = self.current_tok.kind {
                        match kw.as_str() {
                            "HEAD" => return self.head_section(),
                            "PARAGRAF" => return self.paragraf_section(),
                            "LIST" => return self.list_section(),
                            _ => self.syntax_error(&format!("Unknown section type '{}'", kw)),
                        }
                    } else {
                        self.syntax_error("Expected section type after #MAEK");
                    }
                }
                _ => self.syntax_error(&format!("Expected #MAEK to start a section, found '{}'", hw)),
            }
        }
        self.syntax_error("Expected section");
    }

    // grammar:  <head_section> ::= #MAEK HEAD <head_content> #OIC
    fn head_section(&mut self) -> ASTNode {
        self.match_keyword("HEAD");
        self.skip_newlines();
        
        let mut children = Vec::new();
        
        // Parse head content until #OIC
        loop {
            self.skip_newlines();
            
            if let TokenKind::HashWord(ref hw) = self.current_tok.kind {
                if hw == "#OIC" {
                    break;
                }
                if hw == "#GIMMEH" {
                    children.push(self.head_content());
                    continue;
                }
            }
            
            // unexpected found, break
            if !matches!(self.current_tok.kind, TokenKind::Newline) {
                break;
            }
            self.next_token();
        }
        
        self.match_hashword("#OIC");
        
        ASTNode::HeadSection { children }
    }

    // grammar: <head_content> ::= #GIMMEH TITLE <text> #MKAY
    fn head_content(&mut self) -> ASTNode {
        self.match_hashword("#GIMMEH");
        self.match_keyword("TITLE");
        
        let mut title_text = String::new();
        
        // collect text until #MKAY
        loop {
            match &self.current_tok.kind {
                TokenKind::HashWord(hw) if hw == "#MKAY" => break,
                TokenKind::Text(t) => {
                    if !title_text.is_empty() {
                        title_text.push(' ');
                    }
                    title_text.push_str(t);
                }
                TokenKind::VarDef(v) => {
                    if !title_text.is_empty() {
                        title_text.push(' ');
                    }
                    title_text.push_str(v);
                }
                TokenKind::Newline => {
                    // Skip newlines in title
                }
                _ => {
                    self.syntax_error(&format!("Unexpected token in TITLE: {:?}", self.current_tok.kind));
                }
            }
            self.next_token();
        }
        
        self.match_hashword("#MKAY");
        
        ASTNode::Title { content: title_text.trim().to_string() }
    }

    // gtammar: <paragraf_section> ::= #MAEK PARAGRAF <paragraf_content> #OIC
    fn paragraf_section(&mut self) -> ASTNode {
        self.match_keyword("PARAGRAF");
        self.skip_newlines();
        
        let mut children = Vec::new();
        
        while !matches!(self.current_tok.kind, TokenKind::HashWord(ref hw) if hw == "#OIC") {
            children.push(self.paragraf_content());
            self.skip_newlines();
        }
        
        self.match_hashword("#OIC");
        
        ASTNode::ParagrafSection { children }
    }

    // grammar: <paragraf_content> ::= <variable_decl> | <variable_assign> | <styled_text> | <text>
    fn paragraf_content(&mut self) -> ASTNode {
        match &self.current_tok.kind {
            TokenKind::HashWord(hw) => {
                match hw.as_str() {
                    "#I HAZ" => self.variable_declaration(),
                    "#IT IZ" => self.variable_assignment(),
                    "#LEMME SEE" => self.variable_reference(),
                    "#GIMMEH" => self.styled_text(),
                    "#MAEK" => self.section(),
                    _ => self.syntax_error(&format!("Unexpected hashword in paragraf: {}", hw)),
                }
            }
            TokenKind::Text(t) => {
                let text = t.clone();
                self.next_token();
                ASTNode::Text { content: text }
            }
            TokenKind::VarDef(v) => {
                let var = v.clone();
                self.next_token();
                ASTNode::Text { content: var }
            }
            TokenKind::Newline => {
                self.next_token();
                ASTNode::Newline
            }
            _ => self.syntax_error("Unexpected token in paragraf content"),
        }
    }

    // grammar:  <variable_decl> ::= #I HAZ <varname>
    fn variable_declaration(&mut self) -> ASTNode {
        self.match_hashword("#I HAZ");
        
        if let TokenKind::VarDef(name) = &self.current_tok.kind {
            let var_name = name.clone();
            self.next_token();
            ASTNode::VariableDeclaration { name: var_name }
        } else {
            self.syntax_error("Expected variable name after #I HAZ");
        }
    }

    // grammar: <variable_assign> ::= #IT IZ <value> #MKAY
    fn variable_assignment(&mut self) -> ASTNode {
        self.match_hashword("#IT IZ");
        
        let mut value = String::new();
        
        // last variable assigned, need semantic to deal with scoping here
        while !matches!(self.current_tok.kind, TokenKind::HashWord(ref hw) if hw == "#MKAY") {
            match &self.current_tok.kind {
                TokenKind::Text(t) => value.push_str(t),
                TokenKind::VarDef(v) => value.push_str(v),
                _ => break,
            }
            self.next_token();
        }
        
        self.match_hashword("#MKAY");
        
        ASTNode::VariableAssignment { 
            name: String::new(), // Need semantic analyzer here
            value: value.trim().to_string() 
        }
    }

    // grammar:  <variable_reference> ::= #LEMME SEE <varname> #MKAY
    fn variable_reference(&mut self) -> ASTNode {
        self.match_hashword("#LEMME SEE");
        
        if let TokenKind::VarDef(name) = &self.current_tok.kind {
            let var_name = name.clone();
            self.next_token();
            self.match_hashword("#MKAY");
            ASTNode::VariableReference { name: var_name }
        } else {
            self.syntax_error("Expected variable name after #LEMME SEE");
        }
    }

    // grammar: <styled_text> ::= #GIMMEH <style> <text> #MKAY
    fn styled_text(&mut self) -> ASTNode {
        self.match_hashword("#GIMMEH");
        
        if let TokenKind::Keyword(style) = &self.current_tok.kind {
            let style_type = style.clone();
            self.next_token();
            
            // NEWLINE is special - doesn't need content or #MKAY
            if style_type == "NEWLINE" {
                return ASTNode::Newline;
            }
            
           // SOUNDZ and VIDZ take URLs
        if style_type == "SOUNDZ" || style_type == "VIDZ" {
            let mut url = String::new();
    
        // Collect the actual URL until #MKAY tag
        while !matches!(self.current_tok.kind, TokenKind::HashWord(ref hw) if hw == "#MKAY") {
            match &self.current_tok.kind {
                TokenKind::Text(t) => {
                url.push_str(t);
                self.next_token();
            }
            TokenKind::VarDef(v) => {
                url.push_str(v);
                self.next_token();
            }
            TokenKind::Address(a) => {
                url.push_str(a);
                self.next_token();
            }
            TokenKind::Newline => {
                // Skip newlines in URLs
                self.next_token();
            }
            _ => break,
        }
    }
    
    self.match_hashword("#MKAY");
    
    return if style_type == "SOUNDZ" {
        ASTNode::Sound { url: url.trim().to_string() }
    } else {
        ASTNode::Video { url: url.trim().to_string() }
    };
}
            
            //vector to hold italic/bold text
            let mut content = Vec::new();
            
            while !matches!(self.current_tok.kind, TokenKind::HashWord(ref hw) if hw == "#MKAY") {
                match &self.current_tok.kind {
                    TokenKind::HashWord(hw) if hw == "#LEMME SEE" => {
                        // variable reference inside styled
                        content.push(self.variable_reference());
            
        }
                    TokenKind::Text(t) => {
                        content.push(ASTNode::Text { content: t.clone() });
                        self.next_token();
                    }
                    TokenKind::VarDef(v) => {
                        content.push(ASTNode::Text { content: v.clone() });
                        self.next_token();
                    }
                    _ => break,
                }
                
            }
            
            self.match_hashword("#MKAY");
            
            match style_type.as_str() {
                "BOLD" => ASTNode::Bold { content },
                "ITALICS" => ASTNode::Italics { content },
                _ => ASTNode::Text { content: format!("{} text", style_type) },
            }
        } else {
            self.syntax_error("Expected style keyword after #GIMMEH");
        }
    }

    // grammar: <list_section> ::= #MAEK LIST <list_items> #OIC
    fn list_section(&mut self) -> ASTNode {
        self.match_keyword("LIST");
        self.skip_newlines();
        
        let mut items = Vec::new();
        
        while !matches!(self.current_tok.kind, TokenKind::HashWord(ref hw) if hw == "#OIC") {
            items.push(self.list_item());
            self.skip_newlines();
        }
        
        self.match_hashword("#OIC");
        
        ASTNode::ListSection { children: items }
    }

    // grammar: <list_item> ::= #GIMMEH ITEM <text> #MKAY
    fn list_item(&mut self) -> ASTNode {
    self.match_hashword("#GIMMEH");
    self.match_keyword("ITEM");
    
    let mut content = Vec::new();
    
    while !matches!(self.current_tok.kind, TokenKind::HashWord(ref hw) if hw == "#MKAY") {
        match &self.current_tok.kind {
            TokenKind::HashWord(hw) if hw == "#LEMME SEE" => {
                content.push(self.variable_reference());
            }
            TokenKind::Text(t) => {
                content.push(ASTNode::Text { content: t.clone() });
                self.next_token();
            }
            TokenKind::VarDef(v) => {
                content.push(ASTNode::Text { content: v.clone() });
                self.next_token();
            }
            _ => break,
        }
    }
    
    self.match_hashword("#MKAY");
    
    ASTNode::Item { content }
}
}

impl<'a> Parser for LolcodeParser<'a> {
    fn parse(&mut self) {
        // parsing from top level grammar rule
        let tree = self.program();
        self.parse_tree = Some(tree);
        
        println!("Parsing successful!");
    }

    fn next_token(&mut self) -> Token {
        let tok = self.lexer.get_next_token();
        self.current_tok = tok.clone();
        tok
    }

    fn current_token(&self) -> &Token {
        &self.current_tok
    }
}
