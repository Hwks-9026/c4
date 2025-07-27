use crate::error;
use crate::tokenizer::*;
use crate::parser::*;
use std::fs::*;
pub fn compile(path: String) {
    let content: String = String::from_utf8_lossy(&read(path).unwrap()).to_string();
    let tokens_result = parse(&content);
    let tokens_vec: Vec<Token>;
    match tokens_result {
        Ok(tokens) => tokens_vec = tokens,
        Err(s) => {
            error(&s);
            return;
        }
    
    }

    println!("{:#?}", Parser::new(tokens_vec).parse());
    
    
    
    
    
}
