use crate::tokenizer::Token;

#[derive(Debug)]
pub(crate) enum Statement{
    Let {
        name: String,
        type_annotation: Type,
        value: Expression,
    },
    Set {
        name: String,
        new_value: Expression
    },
    Ret {
        value: Expression
    },
    FunctionCall {
        call: Expression
    }


}

#[derive(Debug)]
pub(crate) enum Expression {
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    Variable(String),
    Binary {
        op: BinaryOp,
        left: Box<Expression>,
        right: Box<Expression>
    },
    FunctionCall {
        name: String,
        args: Vec<Expression>
    }
}

#[derive(Debug)]
pub(crate) enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,

}

#[derive(Debug)]
pub(crate) enum Type {
    I32,
    U32,
    F32,
    I64,
    U64,
    F64,
    BOOL,
    CHAR,
    STRING,
    Custom(String),

}

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
}

impl Parser<'_> {

    pub fn new<'a>(tokens: Vec<Token<'a>>) -> Parser<'a> {
        Parser {
            tokens: tokens,
            current: 0
        }
    }
    pub fn parse(&mut self) -> Result<Program, String> {
        let mut objs: Vec<ProgramObject> = Vec::new();
        loop {
            match self.parse_object() {
                Ok(obj) => objs.push(obj),
                Err(_) => break
            }

        }
        Ok( Program { objs } )
    }

    pub fn parse_object(&mut self) -> Result<ProgramObject, String> {
        match self.peek().ok_or("Failed to find program object to parse")? {
            Token::Function => {
                self.parse_function()
            }
            _ => Err("Unexpected token in place of the start of a program object".to_string())
        }
        
    }

    pub fn parse_function(&mut self) -> Result<ProgramObject, String> {
        self.expect_and_consume(Token::Function)?;
        let name: String = match self.advance() {
            Some(Token::Identifier(string)) => string.to_string(),
            _ => return Err("Expected function name after 'fn' keyword".into())
        };
        self.expect_and_consume(Token::LParen)?;
        let mut args: Vec<(String, Type)> = Vec::new();
        match self.expect(Token::RParen) {
            Ok(_) => {},
            Err(_) => {

                loop {
                    

                    let label: String = match self.advance() {
                        Some(Token::Identifier(string)) => string.to_string(),
                        _ => return Err("Expected function name after 'fn' keyword".into())
                    };
                    self.expect_and_consume(Token::TypeDecl)?;
                    
                    let type_annotation: Type = match self.advance() {
                        Some(Token::Identifier(string)) => {
                            match *string {
                                "i32" => Type::I32,
                                "i64" => Type::I64,
                                "u32" => Type::U32,
                                "u64" => Type::U64,
                                "f32" => Type::F32,
                                "f64" => Type::F64,
                                "bool" => Type::BOOL,
                                "char" => Type::CHAR,
                                "string" => Type::STRING,
                                _ => Type::Custom((*string).to_string())
                            }
                        },
                        _ => return Err("Expected type identifier after assignment symbol in function signature".to_string())
                    };
                    args.push((label.clone(), type_annotation));
                    
                    match self.expect(Token::RParen) {
                        Ok(_) => break,
                        Err(_) => {}
                    }
                    self.expect_and_consume(Token::ArgumentSeparator)?;

                }
            }
        }
        self.expect_and_consume(Token::RParen)?;
        let return_type: Option<Type> = match self.expect(Token::RetType) {
            Ok(_) => {
                self.current += 1;
                Some(match self.advance() {
                    Some(Token::Identifier(string)) => {
                        match *string {
                            "i32" => Type::I32,
                            "i64" => Type::I64,
                            "u32" => Type::U32,
                            "u64" => Type::U64,
                            "f32" => Type::F32,
                            "f64" => Type::F64,
                            "bool" => Type::BOOL,
                            "char" => Type::CHAR,
                            "string" => Type::STRING,
                            _ => Type::Custom((*string).to_string())
                        }
                    },
                    _ => return Err("Expected type identifier after assignment symbol in function signature".to_string())
                })
            }
            Err(_) => { None }
        };
        self.expect_and_consume(Token::LBrace)?;

        let statements = self.parse_statements()?;
        Ok(ProgramObject::Function { name, arguments: args, return_type, statements})
    }


    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> Option<&Token> {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
        self.tokens.get(self.current - 1)
    }

    fn is_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn expect(&self, tok: Token) -> Result<&Token, String> {
        if let Some(parser_tok) = self.peek() {
            if parser_tok.eq(&tok) {
                return Ok(parser_tok)
            }
            return Err(format!("Failed an expect. Expected {:?}, got {:?}", tok, parser_tok ).into())
        };
        Err(format!("ran out of tokens while expecting {:?}.", tok).into())
    }

    fn expect_and_consume(&mut self, tok: Token) -> Result<&Token, String> {
        if let Some(parser_tok) = self.advance() {
            if parser_tok.eq(&tok) {
                return Ok(parser_tok)
            }
            return Err(format!("Failed an expect. Expected {:?}, got {:?}", tok, parser_tok ).into())
        };
        Err(format!("ran out of tokens while expecting {:?}.", tok).into())
    }


    fn parse_statements(&mut self) -> Result<Vec<Statement>, String> {
        
        let mut statements: Vec<Statement> = Vec::new();
        loop {
            match self.peek() {
                Some(token) => {
                    match token {
                        Token::RBrace => {
                            self.current += 1;
                            return Ok(statements)
                        },
                        Token::EOF => {
                            return Err("unecpected EOF".to_string())
                        }
                        _ => statements.push(self.parse_statement()?)
                    }
                }
                None => {
                    return Err("NONE: unecpected EOF".to_string())
                }
            }
        }
    }
    fn parse_statement(&mut self) -> Result<Statement, String> {
        if let Some(token) = self.advance() {
            match token {
                Token::Let => self.parse_let_statement(),
                Token::Return => self.parse_ret_statement(),
                Token::Identifier(str) => {
                    let owned: String = str.to_owned().to_string();
                    self.parse_identifier_statement(owned)
                },
                _ => Err("Unexpected Statement".into())
            }
        } else {
            Err("Unexpected End of File".into())
        }
    }

    fn parse_ret_statement(&mut self) -> Result<Statement, String> {
        let value: Expression = self.parse_expression()?;
        self.expect_and_consume(Token::StatementEnd)?;
        Ok(Statement::Ret { value: value })
    }
    fn parse_let_statement(&mut self) -> Result<Statement, String> {
        let name: String = match self.advance() {
            Some(Token::Identifier(string)) => string.to_string(),
            _ => return Err("Expected Identifier after 'let' keyword".into())
        };
        self.expect_and_consume(Token::TypeDecl)?;
        let type_annotation: Type = match self.advance() {
            Some(Token::Identifier(string)) => {
                match *string {
                    "i32" => Type::I32,
                    "i64" => Type::I64,
                    "u32" => Type::U32,
                    "u64" => Type::U64,
                    "f32" => Type::F32,
                    "f64" => Type::F64,
                    "bool" => Type::BOOL,
                    "char" => Type::CHAR,
                    "string" => Type::STRING,
                    _ => Type::Custom((*string).to_string())
                }
            },
            _ => return Err("Expected type identifier after assignment symbol in let statement.".to_string())
        };

        self.expect_and_consume(Token::Assign)?;

        let value: Expression = self.parse_expression()?;
        self.expect_and_consume(Token::StatementEnd)?;

        Ok(Statement::Let { name, type_annotation, value })
    }

    fn parse_identifier_statement(&mut self, name: String) -> Result<Statement, String> {
        if let Some(next_token) = self.peek() {
            match next_token {
                Token::LParen => {
                    let call = self.parse_function_call(name)?;
                    self.expect_and_consume(Token::StatementEnd)?;
                    Ok(Statement::FunctionCall { call })

                },
                _ => {
                    self.expect_and_consume(Token::Assign)?;
                    let new_value: Expression = self.parse_expression()?;
                    self.expect_and_consume(Token::StatementEnd)?;
                    Ok(Statement::Set { name, new_value })
                }

            }
        } else {
            Err("Unexpected EOF".to_string())
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {

        self.parse_binary_expr()
    }

    fn parse_binary_expr(&mut self) -> Result<Expression, String> {
        let left = match self.advance().ok_or("Unecpected EOF".to_string())? {
            Token::LParen => {
                self.parse_binary_expr()?
            },
            Token::Number(number_str) => {
                match parse_number(&number_str)? {
                    Number::Integer(int) => {Expression::IntLiteral(int)}
                    Number::Float(int) => {Expression::FloatLiteral(int)}
                }
            }
            Token::Identifier(id_str) => {
                let id = id_str.to_owned().to_string();
                if let Some(token) = self.peek() {
                    match token {
                        Token::LParen => {
                            self.parse_function_call(id)?
                        },
                        _ => Expression::Variable(id)
                    }
                } else {
                    return Ok(Expression::Variable(id))
                }
            }
            _ => {return Err("Unexpected Token in Expression".to_string())}
        };
        if self.is_end() {
            return Ok(left)
        }
        match self.expect(Token::RParen) {
            Ok(_) => {self.advance();},
            Err(_) => {}
        };

        match self.expect(Token::StatementEnd) {
            Ok(_) => {return Ok(left)},
            Err(_) => {}
        };
        let op_token = self.advance().ok_or("Unecpected EOF".to_string())?;
        let op = match op_token {
            Token::Operator(op_string) => {
                match *op_string {
                    "+" => BinaryOp::Add,
                    "-" => BinaryOp::Sub,
                    "/" => BinaryOp::Div,
                    "*" => BinaryOp::Mul,
                    _   => return Err("NOT A BINARY Operator".to_string())
                }
            
            }
            Token::RParen => {return Ok(left)}
            Token::StatementEnd => {self.current -= 1; return Ok(left)}
            _ => {return Err(format!("Unexpected Token {:#?} in Position of Binary Operator", op_token).to_string())}
        };
        let right = match self.advance().ok_or("Unecpected EOF".to_string())? {
            Token::LParen => {
                self.parse_binary_expr()?
            },
            Token::Number(number_str) => {
                match parse_number(&number_str)? {
                    Number::Integer(int) => {Expression::IntLiteral(int)}
                    Number::Float(int) => {Expression::FloatLiteral(int)}
                }
            }
            Token::Identifier(id_str) => {
                let id = id_str.to_owned().to_string();
                if let Some(token) = self.peek() {
                    match token {
                        Token::LParen => {
                            self.parse_function_call(id)?
                        },
                        _ => Expression::Variable(id)
                    }
                } else {
                    return Ok(Expression::Variable(id))
                }
            }
            _ => {return Err("Unexpected Token in Expression".to_string())}
        };

        match self.expect(Token::RParen) {
            Ok(_) => {self.advance();},
            Err(_) => {}
        };
        Ok(Expression::Binary { op, left: Box::new(left), right: Box::new(right) }) 
    }
    
    fn parse_function_call(&mut self, name: String) -> Result<Expression, String> {
        self.expect_and_consume(Token::LParen)?;
        let mut args: Vec<Expression> = Vec::new();
        let mut outer = true;
        let mut depth = 0;
        let mut index = self.current;
        while outer == true {
            let mut inner = true;
            let mut expression_tokens: Vec<Token> = Vec::new();

            while inner == true {
                
                let mut add_token = true;
                index += 1;
                if let Some(tok) = self.tokens.get(index - 1) {
                    match tok {
                        Token::LParen => {
                            depth += 1
                        }
                        Token::RParen => {
                            if depth == 0 {
                                outer = false;
                                break;
                            }
                            depth -= 1;
                        }
                        Token::ArgumentSeparator => {
                            if depth == 0 {
                                inner = false;
                                add_token = false;
                            }
                        }
                        _ => {}
                    }
                    if add_token {
                        expression_tokens.push(tok.clone());
                    }

                }
            }
            args.push(evaluate_expression_from_tokens(expression_tokens)?)
        }
        self.current = index;
        return Ok(Expression::FunctionCall {
            name,
            args
        })
    }
}

fn evaluate_expression_from_tokens(tokens: Vec<Token>) -> Result<Expression, String> {
    Parser::new(tokens).parse_expression()
    
}


fn token_to_bin_op(tok: &Token) -> Option<BinaryOp> {
    match tok {
        Token::Operator(op_string) => {
            Some(
                match *op_string {
                    "+" => BinaryOp::Add,
                    "-" => BinaryOp::Sub,
                    "/" => BinaryOp::Div,
                    "*" => BinaryOp::Mul,
                    _   => return None
                }
            )
        },
        _ => {None}

    }
}

enum Number {
    Integer(i64),
    Float(f64),
}

fn parse_number(input: &str) -> Result<Number, String> {
    if let Ok(i) = input.parse::<i64>() {
        Ok(Number::Integer(i))
    } else if let Ok(f) = input.parse::<f64>() {
        Ok(Number::Float(f))
    } else {
        Err(format!("Failed to parse '{}' as a number", input))
    }
}






#[derive(Debug)]
pub(crate) struct Program {
    objs: Vec<ProgramObject>
}

#[derive(Debug)]
pub(crate) enum ProgramObject {
    Function {
        name: String,
        arguments: Vec<(String, Type)>,
        return_type: Option<Type>,
        statements: Vec<Statement>,
    }
    //TODO: ADD STRUCTS AND ENUMS (THAT SOUNDS HARD)
}
