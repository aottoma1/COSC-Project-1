
//declares other folders to use
mod token;
mod lexer;

use lexer::{LexicalAnalyzer, Lexer};
use std::path::Path;

fn main() {
    //one input file to test
    let input = std::env::args().nth(1).unwrap_or_else(||{
        eprintln!("Usage: lolcompiler <file.lol>");
        std::process::exit(1);
    });

    //.lol extension for test
     if Path::new(&input).extension().and_then(|s| s.to_str()) != Some("lol") {
        eprintln!("Error: input file must have a .lol extension");
        std::process::exit(1);
    }

    //read file to a string
     let source = std::fs::read_to_string(&input).unwrap_or_else(|e| {
        eprintln!("Failed to read '{}': {}", input, e);
        std::process::exit(1);
    });

    //initialize lexer
     let mut lx = Lexer::new(&source);

     //pull all tokens in file and if invalid print error message and exit. 
     //if all valid output valid
     loop {
        let tok = lx.get_next_token();
        if let token::TokenKind::Eof = tok.kind {
            break;
        }
        
    }

    // lexer successs
    println!("valid");

}
