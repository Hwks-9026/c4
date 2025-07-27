use logos::Logos;
use colored::Colorize;
#[derive(Logos, Debug, PartialEq, Clone, Copy)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
pub enum Token<'a> {
    EOF,
    // Reserved keywords
    #[token("let")]
    Let,

    #[token("mut")]
    Mut,

    #[token("ret")]
    Return,

    #[token("=>")]
    FnArrow,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[token("while")]
    While,

    #[token("for")]
    For,

    #[token("fn")]
    Function,

    #[token("mod")]
    Module,
    
    #[token("struct")]
    Struct,

    #[token("enum")]
    Enum,

    //Operators
    #[token("<-")]
    Assign,

    #[token("|>")]
    FnPipe,

    #[token("|")]
    TypeDecl,

    #[token("->")]
    RetType,

    #[token("@")]
    PatternMatch,

    #[token("~")]
    Wildcard,

    //Delimiters and symbols
    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token(",")]
    ArgumentSeparator,

    #[token(";")]
    StatementEnd,

    #[token(".")]
    FieldAccessor,

    #[regex("[a-zA-Z0-9_]*")]
    Identifier(&'a str),

    #[regex("\\d+(\\.\\d+)?")]
    Number(&'a str),

    #[regex("\"([^\"\\\\]|\\\\.)*\"")]
    StringLiteral(&'a str),

    #[regex("'.'")]
    CharLiteral(&'a str),

    #[regex("true|false")]
    BoolLiteral(&'a str),

    #[regex("\\+|-|\\*|\\/|%")]
    Operator(&'a str),
}


pub fn parse(input: &str) -> Result<Vec<Token>, String> {
    let mut lex = Token::lexer(input);
    let mut tokens: Vec<Token> = Vec::new();

    let mut next = lex.next();
    while next != None {
        match next.unwrap() {
            Ok(token) => {
                tokens.push(token);
            }
            Err(_) => return Err("failed to parse file :(".to_string()),
        }
        next = lex.next()
    }
    tokens.push(Token::EOF);
    return Ok(tokens);
}

pub fn print_tokens(tokens: &Vec<Token>) {
    println!();
    let mut depth: u32 = 0;
    for token in tokens {
        match token {
            Token::LBrace => {
                depth += 1;
                println!("LBrace");
                let mut i = 0;
                while i < depth {
                    print!("\t");
                    i += 1;
                }
            }
            Token::RBrace => {
                println!();
                depth -= 1;
                let mut i = 0;
                while i < depth {
                    print!("\t");
                    i += 1;
                }
                println!("RBrace");
                i = 0;
                while i < depth {
                    print!("\t");
                    i += 1;
                }
            }
            Token::StatementEnd => {
                print!(";\n");
                let mut i = 0;
                while i < depth {
                    print!("\t");
                    i += 1;
                }
            }
            Token::Identifier(content) => {
                print!("{}{}{} ","IDENTIFIER(".yellow(), content.yellow(), ")".yellow())
            }
            _ => {
                print!("{:?} ", token)
            }
        }
    }
}
