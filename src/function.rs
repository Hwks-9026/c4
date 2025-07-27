#![allow(dead_code)]
use crate::tokenizer::*;
use crate::tokenizer::Token::*;
use colored::Colorize;
#[derive(Debug)]
pub struct Function<'a> {
    name: String,
    arg_types: Vec<Token<'a>>,
    return_type: Token<'a>,
    statements: Vec<Vec<Token<'a>>>,
}

pub fn function_create(mut tokens: Vec<Token>) -> Result<Function, ()> {
    
    let name = match tokens[0] {
        Identifier(s) => {s},
        _ => {return Err(());}
    };
    tokens.remove(0);
    if tokens[0] != Token::LParen {
        eprintln!("{}", "ERR.".to_string().red());
        eprintln!("expected `(` after function name at function {}, got {:?}", name.yellow(), tokens[0]);
        return Err(());
    }

    Err(
    () 
    )
    
}
