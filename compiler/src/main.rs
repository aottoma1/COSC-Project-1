
mod token;
mod lexer;
mod parser;
mod semantic;

use lexer::{LexicalAnalyzer, Lexer};
use parser::{LolcodeParser, Parser};
use semantic::LolcodeSemanticAnalyzer;
use std::path::Path;

fn main() {
    //one input file to test
    let input = std::env::args().nth(1).unwrap_or_else(||{
        eprintln!("Usage: lolcompiler <file.lol>");
        std::process::exit(1);
    });

    //make sure its a .lol file, error if not
    if Path::new(&input).extension().and_then(|s| s.to_str()) != Some("lol") {
        eprintln!("Error: input file must have a .lol extension");
        std::process::exit(1);
    }

    //read file to a string
    let source = std::fs::read_to_string(&input).unwrap_or_else(|e| {
        eprintln!("Failed to read '{}': {}", input, e);
        std::process::exit(1);
    });
    //Testing task 1: Lexical Analysis
    //test that all tokens are valid
    let mut lexer = Lexer::new(&source);
    loop {
        let tok = lexer.get_next_token();
        if let token::TokenKind::Eof = tok.kind {
            break;
        }
        // token was valid (lexer would exit on invalid tokens)
    }

    //Testing task 2: Syntax Analysis
    let mut parser = LolcodeParser::new(&source);
    
    //parse the source to build abstract syntax tree
    parser.parse();

    //Testing task 3: Semantic Analysis
    //get the parse tree from the parser
    if let Some(ref tree) = parser.parse_tree {
        let mut semantic_analyzer = LolcodeSemanticAnalyzer::new();
        semantic_analyzer.analyze_tree(tree, &input);
    } else {
        eprintln!("Error: No parse tree generated");
        std::process::exit(1);
    }

    //if we reach here, both lexical and syntax analysis succeeded
    println!("valid");
}